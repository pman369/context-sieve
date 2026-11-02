//! Symbol value types produced by AST parsing.
//!
//! All fields are byte-offset spans into the **original source slice** — the
//! parser never copies source bytes into symbol storage.

/// Half-open byte span `[start, end)` into the original source buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[inline]
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Extract the substring from a source slice without allocation.
    #[inline]
    pub fn slice<'a>(&self, src: &'a [u8]) -> &'a [u8] {
        &src[self.start as usize..self.end as usize]
    }
}

/// Classification of a symbol extracted from the AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Import,
    TypeAlias,
    Constant,
    Variable,
    /// A call site — not a definition but a use of another symbol.
    CallSite,
}

/// A single named entity extracted from a source AST.
///
/// Symbols are intentionally `Copy`-able (no String, no Vec).
/// The `name` field is a span into the source buffer — callers use
/// [`Span::slice`] to read the raw bytes when needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol {
    /// Byte span of the symbol's identifier in the source.
    pub name: Span,
    /// Byte span of the entire node (declaration or call expression).
    pub body: Span,
    /// What kind of entity this symbol represents.
    pub kind: SymbolKind,
    /// Nesting depth inside the file (0 = top-level).
    pub depth: u8,
}

impl Symbol {
    pub fn new(name: Span, body: Span, kind: SymbolKind, depth: u8) -> Self {
        Self { name, body, kind, depth }
    }

    /// True if this symbol is a definition (not a call site / import).
    pub fn is_definition(&self) -> bool {
        !matches!(self.kind, SymbolKind::CallSite | SymbolKind::Import)
    }
}
