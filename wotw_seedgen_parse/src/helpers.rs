use crate::{Ast, ErrorKind, ParseIdentToken, Parser, Result, Tokenize};
use parse_display::Display;
use std::fmt::{self, Display};

/// [`Ast`] node parsing an identifier
///
/// [`Identifier`] implements [`Ast`] if your `Token` implements [`ParseIdentToken`].
/// If [`ParseIdentToken::is_ident`] returns `true`, the token's slice will be stored as-is inside this type.
///
/// ```
/// use wotw_seedgen_parse::{parse_ast, Identifier, ParseIdentToken, ParseToken};
///
/// #[derive(ParseToken)]
/// #[logos(skip r"\s+")]
/// enum Token {
///     #[regex(r"\w+")]
///     Identifier,
///     #[regex(r".", priority = 0)]
///     Symbol,
/// }
///
/// impl ParseIdentToken for Token {
///     fn is_ident(&self) -> bool {
///         matches!(self, Token::Identifier)
///     }
/// }
///
/// assert_eq!(
///     parse_ast::<Token, _>("   OriIsAGoodGame   ").parsed,
///     Ok(Identifier("OriIsAGoodGame"))
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[repr(transparent)]
pub struct Identifier<'source>(pub &'source str);
impl<'source, T> Ast<'source, T> for Identifier<'source>
where
    T: Tokenize,
    T::Token: ParseIdentToken,
{
    fn ast(parser: &mut Parser<'source, T>) -> Result<Self> {
        let (token, span) = parser.current();
        if token.is_ident() {
            let slice = parser.slice(span.clone());
            parser.step();
            Ok(Self(slice))
        } else {
            Err(parser.error(ErrorKind::ExpectedToken("identifier".to_string())))
        }
    }
}

/// [`Ast`] node parsing a specific [`char`]
///
/// The implementation will not check the kind of `Token`, but it will only succeed if the `Token` contains *only* the character
///
/// ```
/// use wotw_seedgen_parse::{Ast, parse_ast, ParseIntToken, ParseToken, Symbol};
///
/// #[derive(ParseToken)]
/// #[logos(skip r"\s+")]
/// enum Token {
///     #[regex(r"[A-Za-z_]\w*")]
///     Identifier,
///     #[regex(r"\d+")]
///     Number,
/// }
///
/// impl ParseIntToken for Token {
///     fn is_int(&self) -> bool {
///         matches!(self, Token::Number)
///     }
/// }
///
/// #[derive(Debug, PartialEq, Ast)]
/// struct HugsAmount {
///     amount: u128,
///     x: Symbol<'x'>,
///     hugs: HugsPlease,
/// }
/// #[derive(Debug, PartialEq, Ast)]
/// struct HugsPlease;
///
/// assert_eq!(
///     parse_ast("2x HugsPlease").parsed,
///     Ok(HugsAmount {
///         amount: 2,
///         x: Symbol,
///         hugs: HugsPlease,
///     })
/// );
///
/// // "xHugsPlease" will be tokenized as one identifier, so <Symbol<'x'>>::ast fill fail
/// assert!(parse_ast::<_, HugsAmount>("2xHugsPlease").parsed.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol<const CHAR: char>;
impl<'source, T, const CHAR: char> Ast<'source, T> for Symbol<CHAR>
where
    T: Tokenize,
{
    fn ast(parser: &mut Parser<'source, T>) -> Result<Self> {
        match parser.current_slice().strip_prefix(CHAR) {
            Some("") => {
                parser.step();
                Ok(Self)
            }
            _ => Err(parser.error(ErrorKind::ExpectedToken(Self.to_string()))),
        }
    }
}
impl<const CHAR: char> Display for Symbol<CHAR> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{CHAR}'")
    }
}

/// [`Ast`] node expecting the parser to be fully finished after parsing `T`
///
/// This usually won't actually be part of your Ast, rather it is returned by [`parse_ast`].
///
/// [`NoTrailingInput::ast`] will never return [`Err`], instead [`NoTrailingInput`] contains [`Result`]s representing the outcome.
/// After calling [`NoTrailingInput::ast`], the `parser` will always be exhausted.
///
/// ```
/// use wotw_seedgen_parse::{NoTrailingInput, parse_ast, ParseIntToken, ParseToken, Symbol};
///
/// #[derive(ParseToken)]
/// enum Token {
///     #[regex(r"\d+")]
///     Number,
///     #[regex(r".", priority = 0)]
///     Symbol,
/// }
///
/// impl ParseIntToken for Token {
///     fn is_int(&self) -> bool {
///         matches!(self, Token::Number)
///     }
/// }
///
/// assert!(matches!(
///     parse_ast::<Token, u8>("5$"),
///     NoTrailingInput {
///         parsed: Ok(5),
///         trailing: Err(_)
///     }
/// ));
/// ```
///
/// [`parse_ast`]: crate::parse_ast
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoTrailingInput<T> {
    pub parsed: Result<T>,
    pub trailing: Result<()>,
}
impl<T> NoTrailingInput<T> {
    pub fn into_result(self) -> Result<T> {
        self.trailing.and(self.parsed)
    }
}
impl<'source, T, V> Ast<'source, T> for NoTrailingInput<V>
where
    T: Tokenize,
    V: Ast<'source, T>,
{
    fn ast(parser: &mut Parser<'source, T>) -> Result<Self> {
        let parsed = V::ast(parser);
        let trailing = if parser.is_finished() {
            Ok(())
        } else {
            let err = parser.error(ErrorKind::ExpectedToken("end of input".to_string()));
            parser.jump(parser.end());
            Err(err)
        };
        Ok(Self { parsed, trailing })
    }
}
