mod command;
mod content;
mod evaluate;
mod expression;
mod function;
mod literal;
mod preprocess;

pub(crate) use function::FunctionIdentifier;
use wotw_seedgen_data::UberIdentifier;

use self::preprocess::{Preprocessor, PreprocessorOutput};
use crate::{
    ast::{self, UberStateType},
    output::{
        self, intermediate::Literal, Command, CommandVoid, CompilerOutput, SnippetDebugOutput,
    },
    token::TOKENIZER,
    types::uber_state_type,
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    fmt::{self, Debug, Display},
    io::{self, Write},
};
use wotw_seedgen_assets::{SnippetAccess, Source, UberStateData};
use wotw_seedgen_parse::{
    parse_ast, Delimited, Error, Identifier, Once, Punctuated, Recoverable, Result,
    SeparatedNonEmpty, Span, Spanned,
};

#[derive(Debug)]
pub struct Compiler<'snippets, 'uberstates, F: SnippetAccess> {
    rng: Pcg64Mcg,
    snippet_access: &'snippets F,
    global: GlobalCompilerData<'uberstates>,
    compiled_snippets: FxHashSet<String>,
    errors: Vec<(Source, Vec<Error>)>,
}

/// How many memory slots to reserve for generated calculations
// TODO how much is needed
pub const RESERVED_MEMORY: usize = 10;

pub(crate) trait Compile<'source> {
    type Output;

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output;
}

impl<'source, T: Compile<'source>> Compile<'source> for Spanned<T> {
    type Output = T::Output;

    #[inline]
    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.data.compile(compiler)
    }
}
impl<'source, T: Compile<'source>> Compile<'source> for Result<T> {
    type Output = Option<T::Output>;

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        let compiled = self.map(|t| t.compile(compiler));
        compiler.consume_result(compiled)
    }
}
impl<'source, T: Compile<'source>, R> Compile<'source> for Recoverable<T, R> {
    type Output = Option<T::Output>;

    #[inline]
    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.result.compile(compiler)
    }
}
impl<'source, T: Compile<'source>> Compile<'source> for Vec<T> {
    type Output = Vec<T::Output>; // TODO experiment with returning iterators instead of vectors from collection compile implementations

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.into_iter().map(|t| t.compile(compiler)).collect()
    }
}
impl<'source, Open, Content: Compile<'source>, Close> Compile<'source>
    for Delimited<Open, Content, Close>
{
    type Output = Option<Content::Output>;

    #[inline]
    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.content.compile(compiler)
    }
}
impl<'source, T: Compile<'source>> Compile<'source> for Once<T> {
    type Output = T::Output;

    #[inline]
    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.0.compile(compiler)
    }
}
impl<'source, Item: Compile<'source>, Punctuation> Compile<'source>
    for Punctuated<Item, Punctuation>
{
    type Output = Vec<Item::Output>;

    #[inline]
    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.into_iter().map(|t| t.compile(compiler)).collect()
    }
}
impl<'source, Item: Compile<'source>, Separator> Compile<'source>
    for SeparatedNonEmpty<Item, Separator>
{
    type Output = Vec<Item::Output>;

    #[inline]
    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.into_iter().map(|t| t.compile(compiler)).collect()
    }
}
impl<'source> Compile<'source> for ast::Snippet<'source> {
    type Output = ();

    #[inline]
    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        self.contents.compile(compiler);
    }
}

// referencing the necessary data instead of the whole Compiler avoids being generic over the Compiler's FileAccess
// TODO remove debug derive from private types?
#[derive(Debug)]
pub(crate) struct GlobalCompilerData<'uberstates> {
    pub output: CompilerOutput,
    pub uber_state_data: &'uberstates UberStateData,
    pub callbacks: FxHashMap<String, FxHashMap<String, usize>>,
    pub shared_values: FxHashMap<String, FxHashMap<String, SharedValue>>,
    pub boolean_ids: IdProvider,
    pub integer_ids: IdProvider,
    pub float_ids: IdProvider,
    pub string_ids: IdProvider,
    pub boolean_state_id: usize,
    pub integer_state_id: usize,
    pub float_state_id: usize,
    pub message_ids: IdProvider,
    pub wheel_ids: IdProvider,
    pub warp_icon_ids: IdProvider,
    // TODO could be a reference
    pub config: FxHashMap<String, FxHashMap<String, String>>,
}
#[derive(Debug)]
pub(crate) enum SharedValue {
    Function(usize),
    Literal(Literal),
}
impl Display for SharedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SharedValue::Function(index) => write!(f, "function: {index}"),
            SharedValue::Literal(literal) => write!(f, "{literal}"),
        }
    }
}
impl<'uberstates> GlobalCompilerData<'uberstates> {
    pub(crate) fn new(
        uber_state_data: &'uberstates UberStateData,
        config: FxHashMap<String, FxHashMap<String, String>>,
    ) -> Self {
        Self {
            output: Default::default(),
            uber_state_data,
            callbacks: Default::default(),
            shared_values: Default::default(),
            boolean_ids: IdProvider::new(RESERVED_MEMORY),
            integer_ids: IdProvider::new(RESERVED_MEMORY),
            float_ids: IdProvider::new(RESERVED_MEMORY),
            string_ids: IdProvider::new(RESERVED_MEMORY + 1), // 1 reserved for spirit light strings
            boolean_state_id: 100,
            integer_state_id: 0,
            float_state_id: 150,
            message_ids: IdProvider::new(0),
            wheel_ids: IdProvider {
                offset: 0,
                ids: FxHashMap::from_iter([("root".to_string(), 0)]),
            },
            warp_icon_ids: IdProvider::new(0),
            config,
        }
    }
}
#[derive(Debug)]
pub(crate) struct IdProvider {
    offset: usize,
    ids: FxHashMap<String, usize>,
}
impl IdProvider {
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            ids: Default::default(),
        }
    }
    pub fn id(&mut self, id: String) -> usize {
        match self.ids.get(&id) {
            None => {
                let len = self.ids.len() + self.offset;
                self.ids.insert(id, len);
                len
            }
            Some(id) => *id,
        }
    }
}
// TODO not sure if all these fields are used anymore since pulling some stuff out into global
pub(crate) struct SnippetCompiler<'compiler, 'source, 'uberstates> {
    pub rng: Pcg64Mcg,
    pub identifier: String, // TODO could be a reference
    pub global: &'compiler mut GlobalCompilerData<'uberstates>,
    pub preprocessed: PreprocessorOutput,
    pub function_indices: FxHashMap<String, usize>, // TODO could maybe be a reference too?
    pub function_imports: FxHashMap<String, String>, // TODO could maybe be a reference too?
    pub variables: FxHashMap<Identifier<'source>, Literal>,
    pub random_pools: FxHashMap<String, Vec<Literal>>, // TODO could maybe be a reference too?
    pub errors: Vec<Error>,
}
const SEED_FAILED_MESSAGE: &str = "Failed to seed child RNG";
impl<'compiler, 'source, 'uberstates> SnippetCompiler<'compiler, 'source, 'uberstates> {
    // TODO weird api
    pub(crate) fn compile<R: Rng>(
        ast: ast::Snippet<'source>,
        rng: &mut R,
        identifier: String,
        global: &'compiler mut GlobalCompilerData<'uberstates>,
        preprocessed: PreprocessorOutput,
    ) -> Self {
        let function_indices = preprocessed
            .functions
            .iter()
            .cloned()
            .zip(global.output.command_lookup.len()..)
            .collect();
        global.output.command_lookup.extend(vec![
            Command::Void(CommandVoid::Multi { commands: vec![] }); // Fill with placeholders for all the functions, this also ensures a sane result if some of the functions fail to compile
            preprocessed.functions.len()
        ]);
        let mut compiler = Self {
            rng: Pcg64Mcg::from_rng(rng).expect(SEED_FAILED_MESSAGE),
            identifier,
            global,
            preprocessed,
            function_indices,
            function_imports: Default::default(),
            variables: Default::default(),
            random_pools: Default::default(),
            errors: Default::default(),
        };
        ast.compile(&mut compiler);
        if let Some(debug) = &mut compiler.global.output.debug {
            // TODO now it's inefficient that we're returning the whole compiler, could save some clones here
            // ... on the other hand, the things we're cloning are probably supposed to be references anyway
            debug.snippets.insert(
                compiler.identifier.clone(),
                SnippetDebugOutput {
                    variables: compiler
                        .variables
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                    function_indices: compiler.function_indices.clone(),
                },
            );
        }
        compiler
    }

    pub(crate) fn resolve<'a>(
        &'a mut self,
        identifier: &'a Spanned<Identifier>,
    ) -> Option<&'a Literal> {
        let literal = self.variables.get(&identifier.data);
        if literal.is_none() {
            self.errors.push(Error::custom(
                "unknown identifier".to_string(),
                identifier.span(),
            ))
        }
        literal
    }

    pub(crate) fn consume_result<T>(&mut self, result: Result<T>) -> Option<T> {
        match result {
            Ok(t) => Some(t),
            Err(err) => {
                self.errors.push(err);
                None
            }
        }
    }

    pub(crate) fn uber_state_type<S: Span>(
        &mut self,
        uber_identifier: UberIdentifier,
        span: S,
    ) -> Option<UberStateType> {
        let ty = uber_state_type(&self.global.uber_state_data, uber_identifier);
        if ty.is_none() {
            self.errors
                .push(Error::custom("Unknown UberState".to_string(), span.span()))
        }
        ty
    }
}

impl<'snippets, 'uberstates, F: SnippetAccess> Compiler<'snippets, 'uberstates, F> {
    pub fn new<R: Rng>(
        rng: &mut R,
        snippet_access: &'snippets F,
        uber_state_data: &'uberstates UberStateData,
        config: FxHashMap<String, FxHashMap<String, String>>,
    ) -> Self {
        Self {
            rng: Pcg64Mcg::from_rng(rng).expect(SEED_FAILED_MESSAGE),
            snippet_access,
            global: GlobalCompilerData::new(uber_state_data, config),
            compiled_snippets: Default::default(),
            errors: Default::default(),
        }
    }

    pub fn debug(&mut self) {
        self.global.output.debug = Some(Default::default());
    }

    pub fn compile_snippet(&mut self, identifier: &str) -> std::result::Result<(), String> {
        if !self.compiled_snippets.insert(identifier.to_string()) {
            return Ok(());
        }

        let source = self.snippet_access.read_snippet(identifier)?;
        let mut errors = vec![];

        let ast = parse_ast(&source.content, TOKENIZER);
        // TODO this pattern seems inconvenient, maybe a result with multiple errors and then use extend instead?
        if let Err(err) = ast.trailing {
            errors.push(err);
        }
        match ast.parsed {
            Err(err) => errors.push(err),
            Ok(ast) => {
                let preprocessor = Preprocessor::preprocess(&ast);
                errors.extend(preprocessor.errors);

                for include in &preprocessor.output.includes {
                    if let Err(err) = self.compile_snippet(&include.data) {
                        errors.push(Error::custom(
                            format!("Failed to read snippet: {err}"),
                            include.span.clone(),
                        ));
                    }
                }

                let compiler = SnippetCompiler::compile(
                    ast,
                    &mut self.rng,
                    identifier.to_string(),
                    &mut self.global,
                    preprocessor.output,
                );

                errors.extend(compiler.errors);
            }
        }

        self.errors.push((source, errors));

        Ok(())
    }

    pub fn finish<W: Write>(self, write_errors: &mut W) -> io::Result<output::CompilerOutput> {
        let mut output = self.global.output;
        if let Some(debug) = &mut output.debug {
            debug.callbacks = self.global.callbacks;
        }

        let mut error_count = 0;

        for (source, errors) in self.errors {
            for error in errors {
                error_count += 1;
                error.write_pretty(&source, &mut *write_errors)?;
            }
        }

        if error_count == 0 {
            output.success = true;
        } else {
            writeln!(
                write_errors,
                "Failed to compile Snippets with {error_count} errors."
            )?;
        }

        Ok(output)
    }
}
