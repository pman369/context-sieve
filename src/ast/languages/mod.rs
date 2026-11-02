//! Language backend registry.
//!
//! This module and its submodules are compiled only for native targets where
//! the tree-sitter C runtime is available.  On `wasm32-unknown-unknown` the
//! entire module is excluded via `#[cfg]`.

#[cfg(not(target_arch = "wasm32"))]
pub mod python;
#[cfg(not(target_arch = "wasm32"))]
pub mod rust;

#[cfg(not(target_arch = "wasm32"))]
use crate::ast::symbol::Symbol;

/// Implemented by each language backend.
#[cfg(not(target_arch = "wasm32"))]
pub trait LanguageParser {
    /// Set the parser language and walk the tree, returning extracted symbols.
    fn parse(parser: &mut tree_sitter::Parser, source: &[u8]) -> Option<Vec<Symbol>>;
}
