//! Language backend registry.

pub mod rust;
pub mod python;

use tree_sitter::Parser;
use crate::ast::symbol::Symbol;

/// Implemented by each language backend.
pub trait LanguageParser {
    /// Set the parser language and walk the tree, returning extracted symbols.
    fn parse(parser: &mut Parser, source: &[u8]) -> Option<Vec<Symbol>>;
}
