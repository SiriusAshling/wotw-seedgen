use crate::ast::{self, RecoverContent};
use rustc_hash::FxHashSet;
use std::{hash::Hash, ops::Range};
use wotw_seedgen_parse::{Error, Recoverable, Span, Spanned};

#[derive(Default)]
pub(crate) struct Preprocessor {
    pub output: PreprocessorOutput,
    pub errors: Vec<Error>,
}
#[derive(Default)]
pub(crate) struct PreprocessorOutput {
    pub includes: FxHashSet<Spanned<String>>, // TODO can these be references?
    pub functions: FxHashSet<String>,         // TODO can these be references?
}
impl Preprocessor {
    pub(crate) fn preprocess(ast: &ast::Snippet) -> Self {
        let mut preprocessor = Self::default();
        preprocessor.preprocess_contents(&ast.contents);
        preprocessor
    }

    fn preprocess_contents(&mut self, contents: &[Recoverable<ast::Content, RecoverContent>]) {
        for content in contents
            .iter()
            .filter_map(|content| content.result.as_ref().ok())
        {
            match content {
                ast::Content::Command(_, content) => {
                    if let Ok(content) = &content.result {
                        match content {
                            ast::Command::Include(_, command) => {
                                if let Ok(command) = &command.result {
                                    if let Ok(args) = &command.content {
                                        insert_unique(
                                            &mut self.output.includes,
                                            &mut self.errors,
                                            Spanned::new(
                                                args.0 .0.data.to_string(),
                                                args.0 .0.span(),
                                            ),
                                            args.0 .0.span(),
                                            "Snippet already included".to_string(),
                                        );
                                    }
                                }
                            }
                            ast::Command::Callback(_, command) => {
                                if let Ok(command) = &command.result {
                                    if let Ok(args) = &command.content {
                                        insert_unique(
                                            &mut self.output.functions,
                                            &mut self.errors,
                                            args.0 .0.data.0.to_string(),
                                            args.0 .0.span(),
                                            "Function already defined".to_string(),
                                        );
                                    }
                                }
                            }
                            // TODO it seems difficult to evaluate ifs here but it's certainly odd to ignore the conditional compilation in this compiler.
                            // One side effect could be that a snippet successfully compiles which optionally declares a function behind an !if, but the client might error then
                            ast::Command::If(_, command) => {
                                if let Ok(contents) = &command.contents.content {
                                    self.preprocess_contents(contents)
                                }
                            }
                            _ => {}
                        }
                    }
                }
                ast::Content::Function(_, content) => {
                    if let Ok(function) = &content.result {
                        insert_unique(
                            &mut self.output.functions,
                            &mut self.errors,
                            function.identifier.data.0.to_string(),
                            function.identifier.span(),
                            "Function already defined".to_string(),
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

fn insert_unique<T: Hash + Eq>(
    set: &mut FxHashSet<T>,
    errors: &mut Vec<Error>,
    value: T,
    span: Range<usize>,
    message: String,
) {
    if !set.insert(value) {
        errors.push(Error::custom(message, span))
    }
}
