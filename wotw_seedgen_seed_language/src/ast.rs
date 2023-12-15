use crate::{
    token::{Token, Tokenizer},
    types::Type,
};
use decorum::R32;
use parse_display::Display;
use std::str::FromStr;
use wotw_seedgen_parse::{
    Ast, ErrorKind, Identifier, Once, ParseIdentToken, Parser, Recover, Recoverable, Result,
    SeparatedNonEmpty, Span, Spanned, Symbol,
};
use wotw_seedgen_seed::PseudoTrigger;

pub type Delimited<const OPEN: char, Content, const CLOSE: char> =
    wotw_seedgen_parse::Delimited<Spanned<Symbol<OPEN>>, Content, Spanned<Symbol<CLOSE>>>;
pub type Punctuated<Item, const PUNCTUATION: char> =
    wotw_seedgen_parse::Punctuated<Item, Symbol<PUNCTUATION>>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Ast)]
pub struct Snippet<'source> {
    pub contents: Vec<Recoverable<Content<'source>, RecoverContent>>,
}
pub struct RecoverContent;
impl<'source> Recover<'source, Tokenizer> for RecoverContent {
    fn recover(parser: &mut Parser<'source, Tokenizer>) {
        // TODO this can skip delimiters
        while !(parser.is_finished() || matches!(parser.current_slice(), "#" | "!" | "on" | "fun"))
        {
            parser.step()
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum Content<'source> {
    Event(Spanned<On>, Recoverable<Event<'source>, RecoverContent>),
    Function(
        Spanned<Fun>,
        Recoverable<FunctionDefinition<'source>, RecoverContent>,
    ),
    Command(
        Spanned<Symbol<'!'>>,
        Recoverable<Command<'source>, RecoverContent>,
    ),
    Annotation(
        Spanned<Symbol<'#'>>,
        Recoverable<Annotation<'source>, RecoverContent>,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(token = Token::On)]
pub struct On;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct Event<'source> {
    pub trigger: Trigger<'source>,
    pub action: Recoverable<Action<'source>, RecoverContent>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum Trigger<'source> {
    Pseudo(Spanned<PseudoTriggerAst>),
    Binding(Spanned<Change>, TriggerBinding<'source>),
    Condition(Expression<'source>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PseudoTriggerAst(pub PseudoTrigger);
impl<'source> Ast<'source, Tokenizer> for PseudoTriggerAst {
    fn ast(parser: &mut Parser<'source, Tokenizer>) -> Result<Self> {
        let (token, span) = parser.current();
        if token.is_ident() {
            let slice = parser.slice(span.clone());
            let pseudo_trigger = PseudoTrigger::from_str(slice)
                // TODO if we have a good from_str error message, drop this
                .map_err(|_| parser.custom_error("Unknown pseudotrigger".to_string()))?;
            parser.step();
            Ok(Self(pseudo_trigger))
        } else {
            Err(parser.error(ErrorKind::ExpectedToken("identifier".to_string())))
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum TriggerBinding<'source> {
    UberIdentifier(UberIdentifier<'source>),
    Identifier(Spanned<Identifier<'source>>),
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(token = Token::Change)]
pub struct Change;
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(token = Token::Fun)]
pub struct Fun;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct FunctionDefinition<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub empty_args: (Symbol<'('>, Symbol<')'>),
    pub actions: Delimited<'{', Vec<Action<'source>>, '}'>,
}

// TODO I think I want a let-style syntax that compiles into all the set and get functions and then remove those to solve the confusion around store vs. set
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum Action<'source> {
    Condition(Spanned<If>, Box<ActionCondition<'source>>),
    Function(Box<FunctionCall<'source>>),
    Multi(Delimited<'{', Vec<Action<'source>>, '}'>),
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(token = Token::If)]
pub struct If;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ActionCondition<'source> {
    pub condition: Expression<'source>,
    pub action: Recoverable<Action<'source>, RecoverContent>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct FunctionCall<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub parameters: Delimited<'(', Punctuated<Expression<'source>, ','>, ')'>,
}

#[derive(Debug, Clone, PartialEq, Eq, Span)]
pub enum Expression<'source> {
    Value(ExpressionValue<'source>),
    Operation(Box<Operation<'source>>),
}
#[derive(Debug, Clone, PartialEq, Eq, Span)]
pub struct Operation<'source> {
    pub left: Expression<'source>,
    pub operator: Operator,
    pub right: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum ExpressionValue<'source> {
    Group(Delimited<'(', Once<Box<Expression<'source>>>, ')'>),
    Action(Action<'source>),
    Literal(Spanned<Literal<'source>>),
    Identifier(Spanned<Identifier<'source>>),
}
// Manual implementation to support operator precedence
impl<'source> Ast<'source, Tokenizer> for Expression<'source> {
    fn ast(parser: &mut Parser<'source, Tokenizer>) -> Result<Self> {
        fn precedence(operator: Operator) -> u8 {
            match operator {
                Operator::Arithmetic(ArithmeticOperator::Multiply | ArithmeticOperator::Divide) => {
                    3
                }
                Operator::Arithmetic(ArithmeticOperator::Add | ArithmeticOperator::Subtract) => 2,
                Operator::Comparator(_) => 1,
                Operator::Logic(_) => 0,
            }
        }
        fn resolve_precedence(
            mut sequence: SeparatedNonEmpty<ExpressionValue, Operator>,
        ) -> Expression {
            let current_operator_index = sequence
                .more
                .iter()
                .enumerate()
                .min_by_key(|(_, (operator, _))| precedence(*operator))
                .map(|(index, _)| index);

            match current_operator_index {
                None => Expression::Value(sequence.first),
                Some(index) => {
                    // We know index < len and split_off does not panic if index == len
                    let right = sequence.more.split_off(index + 1);
                    // We know len > 0 because we split off at index + 1
                    let (operator, first_right) = sequence.more.pop().unwrap();
                    let right_sequence = SeparatedNonEmpty {
                        first: first_right,
                        more: right,
                    };

                    Expression::Operation(Box::new(Operation {
                        left: resolve_precedence(sequence),
                        operator,
                        right: resolve_precedence(right_sequence),
                    }))
                }
            }
        }
        SeparatedNonEmpty::ast(parser).map(resolve_precedence)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ast)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Logic(LogicOperator),
    Comparator(Comparator),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ast)]
pub enum ArithmeticOperator {
    #[ast(token = Token::Add)]
    Add,
    #[ast(token = Token::Subtract)]
    Subtract,
    #[ast(token = Token::Multiply)]
    Multiply,
    #[ast(token = Token::Divide)]
    Divide,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ast)]
pub enum LogicOperator {
    #[ast(token = Token::And)]
    And,
    #[ast(token = Token::Or)]
    Or,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ast)]
pub enum Comparator {
    #[ast(token = Token::Equal)]
    Equal,
    #[ast(token = Token::NotEqual)]
    NotEqual,
    #[ast(token = Token::LessOrEqual)]
    LessOrEqual,
    #[ast(token = Token::Less)]
    Less,
    #[ast(token = Token::GreaterOrEqual)]
    GreaterOrEqual,
    #[ast(token = Token::Greater)]
    Greater,
}

#[derive(Debug, Clone, PartialEq, Eq, Ast)]
pub enum Literal<'source> {
    UberIdentifier(UberIdentifier<'source>),
    Boolean(bool),
    Integer(i32),
    Float(R32),
    String(&'source str),
    Constant(Constant<'source>),
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum UberIdentifier<'source> {
    Numeric(UberIdentifierNumeric),
    Name(UberIdentifierName<'source>),
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct UberIdentifierNumeric {
    pub group: Spanned<i32>,
    pub separator: Symbol<'|'>,
    pub member: Spanned<i32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct UberIdentifierName<'source> {
    pub group: Spanned<Identifier<'source>>,
    pub period: Symbol<'.'>,
    pub member: Spanned<Identifier<'source>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct Constant<'source> {
    pub kind: Spanned<Identifier<'source>>,
    pub separator: Variant,
    pub variant: Spanned<Identifier<'source>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(token = Token::Variant)]
pub struct Variant;

#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum Command<'source> {
    Include(Spanned<Include>, CommandArgs<IncludeArgs<'source>>),
    Callback(Spanned<Callback>, CommandArgs<CallbackArgs<'source>>),
    OnCallback(Spanned<OnCallback>, CommandArgs<OnCallbackArgs<'source>>),
    Share(Spanned<Share>, CommandArgs<ShareArgs<'source>>),
    Use(Spanned<Use>, CommandArgs<UseArgs<'source>>),
    Spawn(Spanned<Spawn>, CommandArgs<SpawnArgs<'source>>),
    Flags(
        Spanned<Flags>,
        CommandArgsCollection<SeparatedNonEmpty<FlagsArg<'source>, Symbol<','>>>,
    ),
    Config(Spanned<Config>, CommandArgs<ConfigArgs<'source>>),
    State(Spanned<State>, CommandArgs<StateArgs<'source>>),
    Let(Spanned<Let>, CommandArgs<LetArgs<'source>>),
    If(Spanned<If>, CommandIf<'source>),
    Repeat(Spanned<Repeat>, CommandRepeat<'source>),
    Add(Spanned<Add>, CommandArgs<AddArgs<'source>>),
    Remove(Spanned<Remove>, CommandArgs<AddArgs<'source>>),
    ItemData(Spanned<ItemData>, CommandArgs<ItemDataArgs<'source>>),
    ItemDataName(
        Spanned<ItemDataName>,
        CommandArgs<ItemDataNameArgs<'source>>,
    ),
    ItemDataPrice(
        Spanned<ItemDataPrice>,
        CommandArgs<ItemDataPriceArgs<'source>>,
    ),
    ItemDataDescription(
        Spanned<ItemDataDescription>,
        CommandArgs<ItemDataDescriptionArgs<'source>>,
    ),
    ItemDataIcon(
        Spanned<ItemDataIcon>,
        CommandArgs<ItemDataIconArgs<'source>>,
    ),
    SetLogicState(
        Spanned<SetLogicState>,
        CommandArgs<SetLogicStateArgs<'source>>,
    ),
    Preplace(Spanned<Preplace>, CommandArgs<PreplaceArgs<'source>>),
    ZoneOf(Spanned<ZoneOf>, CommandArgs<ZoneOfArgs<'source>>),
    ItemOn(Spanned<ItemOn>, CommandArgs<ItemOnArgs<'source>>),
    CountInZone(Spanned<CountInZone>, CommandArgs<CountInZoneArgs<'source>>),
    RandomInteger(
        Spanned<RandomInteger>,
        CommandArgs<RandomIntegerArgs<'source>>,
    ),
    RandomFloat(Spanned<RandomFloat>, CommandArgs<RandomFloatArgs<'source>>),
    RandomPool(Spanned<RandomPool>, CommandArgs<RandomPoolArgs<'source>>),
    RandomFromPool(
        Spanned<RandomFromPool>,
        CommandArgs<RandomFromPoolArgs<'source>>,
    ),
}
pub type CommandArgsCollection<Args> = Recoverable<Delimited<'(', Args, ')'>, RecoverContent>;
pub type CommandArgs<Args> = CommandArgsCollection<Once<Args>>;
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Include;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct IncludeArgs<'source>(pub Spanned<&'source str>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Callback;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct CallbackArgs<'source>(pub Spanned<Identifier<'source>>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct OnCallback;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct OnCallbackArgs<'source> {
    pub snippet_name: Spanned<&'source str>,
    pub comma: Symbol<','>,
    pub identifier: Spanned<Identifier<'source>>,
    pub comma_2: Symbol<','>,
    pub action: Action<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Share;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ShareArgs<'source>(pub Spanned<Identifier<'source>>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Use;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct UseArgs<'source> {
    pub snippet_name: Spanned<&'source str>,
    pub comma: Symbol<','>,
    pub identifier: Spanned<Identifier<'source>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Spawn;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct SpawnArgs<'source>(pub Spanned<&'source str>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Flags;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct FlagsArg<'source>(pub Spanned<&'source str>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Config;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ConfigArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub description: Spanned<&'source str>,
    pub comma_2: Symbol<','>,
    pub ty: Spanned<UberStateType>,
    pub comma_3: Symbol<','>,
    pub default: Spanned<Literal<'source>>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ast, Display)]
pub enum UberStateType {
    Boolean,
    Integer,
    Float,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct State;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct StateArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub ty: Spanned<UberStateType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Let;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct LetArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub value: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct CommandIf<'source> {
    pub condition: Expression<'source>,
    pub contents: Delimited<'{', Vec<Recoverable<Content<'source>, RecoverContent>>, '}'>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct CommandRepeat<'source> {
    pub amount: Expression<'source>,
    pub contents: Delimited<'{', Vec<Recoverable<Content<'source>, RecoverContent>>, '}'>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Repeat;
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Add;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct AddArgs<'source>(pub ChangeItemPoolArgs<'source>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Remove;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct RemoveArgs<'source>(pub ChangeItemPoolArgs<'source>);
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ChangeItemPoolArgs<'source> {
    pub item: Action<'source>,
    pub comma: Symbol<','>,
    pub amount: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct ItemData;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ItemDataArgs<'source> {
    pub item: Action<'source>,
    pub comma: Symbol<','>,
    pub name: Spanned<&'source str>,
    pub comma_2: Symbol<','>,
    pub price: Expression<'source>,
    pub comma_3: Symbol<','>,
    pub description: Expression<'source>,
    pub comma_4: Symbol<','>,
    pub icon: Expression<'source>,
    pub comma_5: Symbol<','>,
    pub map_icon: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct ItemDataName;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ItemDataNameArgs<'source> {
    pub item: Action<'source>,
    pub comma: Symbol<','>,
    pub name: Spanned<&'source str>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct ItemDataPrice;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ItemDataPriceArgs<'source> {
    pub item: Action<'source>,
    pub comma: Symbol<','>,
    pub price: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct ItemDataDescription;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ItemDataDescriptionArgs<'source> {
    pub item: Action<'source>,
    pub comma: Symbol<','>,
    pub description: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct ItemDataIcon;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ItemDataIconArgs<'source> {
    pub item: Action<'source>,
    pub comma: Symbol<','>,
    pub icon: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct SetLogicState;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct SetLogicStateArgs<'source>(pub Spanned<&'source str>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Preplace;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct PreplaceArgs<'source> {
    pub item: Action<'source>,
    pub comma: Symbol<','>,
    pub zone: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct ZoneOf;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ZoneOfArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub item: Action<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct ItemOn;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct ItemOnArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub trigger: Trigger<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct CountInZone;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct CountInZoneArgs<'source> {
    pub zone_bindings: Delimited<
        '[',
        Punctuated<Delimited<'(', Once<CountInZoneBinding<'source>>, ')'>, ','>,
        ']',
    >,
    pub comma: Symbol<','>,
    pub items: Delimited<'[', Punctuated<Action<'source>, ','>, ']'>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct CountInZoneBinding<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub zone: Expression<'source>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct RandomInteger;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct RandomIntegerArgs<'source>(pub RandomNumberArgs<'source>);
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct RandomFloat;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct RandomFloatArgs<'source>(pub RandomNumberArgs<'source>);
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct RandomNumberArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub min: Spanned<Expression<'source>>,
    pub comma_2: Symbol<','>,
    pub max: Spanned<Expression<'source>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct RandomPool;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct RandomPoolArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub ty: Spanned<Type>,
    pub comma_2: Symbol<','>,
    pub values: Delimited<'[', Punctuated<Expression<'source>, ','>, ']'>,
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct RandomFromPool;
#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub struct RandomFromPoolArgs<'source> {
    pub identifier: Spanned<Identifier<'source>>,
    pub comma: Symbol<','>,
    pub pool_identifier: Spanned<Identifier<'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Ast, Span)]
pub enum Annotation<'source> {
    Hide(Spanned<Hide>),
    Name(Spanned<Name>, CommandArgs<Spanned<&'source str>>),
    Category(Spanned<Category>, CommandArgs<Spanned<&'source str>>),
    Description(Spanned<Description>, CommandArgs<Spanned<&'source str>>),
}
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Hide;
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Name;
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Category;
#[derive(Debug, Clone, PartialEq, Eq, Ast)]
#[ast(case = "snake")]
pub struct Description;
