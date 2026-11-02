//! Language backend registry.

pub mod python;
pub mod rust;

use crate::ast::symbol::Symbol;
use tree_sitter::Parser;

/// Implemented by each language backend.
pub trait LanguageParser {
    /// Set the parser language and walk the tree, returning extracted symbols.
    fn parse(parser: &mut Parser, source: &[u8]) -> Option<Vec<Symbol>>;
}
