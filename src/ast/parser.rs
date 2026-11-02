//! Language dispatch + Tree-sitter parsing.
//!
//! [`AstParser`] holds a pre-initialised `tree_sitter::Parser` and dispatches
//! to the correct language backend based on file extension.  The parser is
//! **not** thread-safe (Tree-sitter's C API is not Send), so callers should
//! hold one instance per thread or use a pool.
//!
//! On `wasm32-unknown-unknown` the tree-sitter C runtime is unavailable, so
//! [`AstParser`] degrades gracefully: `parse()` always returns `None` and the
//! sieve pass forwards the source buffer unchanged.

use crate::ast::symbol::Symbol;

/// Supported source languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    Go,
}

impl Language {
    /// Infer language from file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python),
            "ts" | "tsx" => Some(Language::TypeScript),
            "go" => Some(Language::Go),
            _ => None,
        }
    }
}

// ── Native (daemon) build ────────────────────────────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
use crate::ast::languages::{python::PythonParser, rust::RustParser, LanguageParser};

/// Facade that owns a `tree_sitter::Parser` and delegates symbol extraction
/// to per-language backends.
#[cfg(not(target_arch = "wasm32"))]
pub struct AstParser {
    inner: tree_sitter::Parser,
}

#[cfg(not(target_arch = "wasm32"))]
impl AstParser {
    pub fn new() -> Self {
        Self {
            inner: tree_sitter::Parser::new(),
        }
    }

    /// Parse `source` as the given language and return the flat symbol list.
    ///
    /// Returns `None` if the language is not yet supported or parsing fails.
    pub fn parse(&mut self, language: Language, source: &[u8]) -> Option<Vec<Symbol>> {
        match language {
            Language::Rust => RustParser::parse(&mut self.inner, source),
            Language::Python => PythonParser::parse(&mut self.inner, source),
            _ => None, // TypeScript / Go: placeholder
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for AstParser {
    fn default() -> Self {
        Self::new()
    }
}

// ── WASM stub ────────────────────────────────────────────────────────────────

/// No-op stub for `wasm32-unknown-unknown`.
///
/// Tree-sitter's C runtime is unavailable in bare WASM; the sieve pass will
/// return the original source unchanged when `parse()` returns `None`.
#[cfg(target_arch = "wasm32")]
pub struct AstParser;

#[cfg(target_arch = "wasm32")]
impl AstParser {
    pub fn new() -> Self {
        Self
    }

    /// Always returns `None` on WASM — callers fall back to the original source.
    pub fn parse(&mut self, _language: Language, _source: &[u8]) -> Option<Vec<Symbol>> {
        None
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for AstParser {
    fn default() -> Self {
        Self::new()
    }
}
