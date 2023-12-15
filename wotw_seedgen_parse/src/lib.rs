//! # Ast Derive
//!
//! `ast_derive` is a parsing library that aims to derive at least a good chunk of your parser directly from your Ast ([Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree))
//! and allows you to manually implement parsing for any parts of it where the derive does not suffice.
//!
//! The resulting parser will be a recursive descent parser capable of generating partial syntax trees
//! and collecting as many errors in one parse as possible. The Ast may contain Spans for all its parsed nodes if desired.
//!
//! # How does it work
//!
//! Your Ast nodes implement the [`Ast`] trait. This allows freely mixing derived and manual [`Ast`] implementations.
//! For any type implementing [`Ast`] you can use [`parse_ast`] to parse it from a [`&str`](str).
//! This means in addition to parsing the full Ast, you will be able to parse any individual Ast node.
//!
//! The Ast may contain [`Result`]s to allow collecting multiple errors from one parse.
//! This relies on you providing implementations of the [`Recover`] trait.
//! You will have to traverse the Ast to collect all the errors, which can often be neatly integrated into future processing steps which may raise errors themselves.
//!
//! The Ast may contain [`Spanned`] wrappers which store the span of their parsed content.
//! You can derive or manually implement [`Span`] on all your Ast nodes to expose the span of higher-level Ast nodes by building it from their children's spans.
//!
//! # Features
//!
//! - `decorum`: implements [`Ast`] for the decorum types [`R32`] and [`R64`]
//! - `ariadne`: adds [`Error::write_pretty`] to write error messages with ariadne without further configuration

#![warn(clippy::todo)]

// TODO document collections
// TODO maybe more collections could implement span?

mod ast;
mod collections;
mod error;
mod helpers;
mod parser;
mod recover;
mod span;
mod tokenizer;

pub use wotw_seedgen_derive::{Ast, Span, TokenDisplay};

pub use ast::*;
pub use collections::*;
pub use error::*;
pub use helpers::*;
pub use parser::*;
pub use recover::*;
pub use span::*;
pub use tokenizer::*;
