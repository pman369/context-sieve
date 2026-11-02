//! Language dispatch + Tree-sitter parsing.
//!
//! [`AstParser`] holds a pre-initialised `tree_sitter::Parser` and dispatches
//! to the correct language backend based on file extension.  The parser is
//! **not** thread-safe (Tree-sitter's C API is not Send), so callers should
//! hold one instance per thread or use a pool.

use tree_sitter::Parser;

use crate::ast::languages::{python::PythonParser, rust::RustParser, LanguageParser};
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

/// Facade that owns a `tree_sitter::Parser` and delegates symbol extraction
/// to per-language backends.
pub struct AstParser {
    inner: Parser,
}

impl AstParser {
    pub fn new() -> Self {
        Self {
            inner: Parser::new(),
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

impl Default for AstParser {
    fn default() -> Self {
        Self::new()
    }
}
