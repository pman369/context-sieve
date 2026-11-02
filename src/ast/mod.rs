//! AST module — Tree-sitter parsing and symbol extraction.
//!
//! Each language backend implements the [`LanguageParser`] trait, returning a
//! flat list of [`Symbol`]s extracted from a source buffer.  The symbols are
//! cheap value types (no heap allocation per symbol) designed to be ingested
//! directly into [`crate::graph`].

pub mod languages;
pub mod parser;
pub mod symbol;

pub use parser::{AstParser, Language};
pub use symbol::{Span, Symbol, SymbolKind};
