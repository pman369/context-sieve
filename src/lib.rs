//! context-sieve library root.
//!
//! # Crate layout
//!
//! | Module | Responsibility |
//! |--------|---------------|
//! | [`ast`]    | Tree-sitter parsing + symbol extraction per language |
//! | [`graph`]  | Memory-mapped symbol dependency graph (petgraph) |
//! | [`sieve`]  | Reachability analysis + buffer condensation |
//! | [`lsp`]    | LSP JSON-RPC types + proxy intercept layer |
//! | [`daemon`] | Async IPC server (feature = "daemon") |
//! | [`wasm`]   | WASM bindings (feature = "wasm") |

pub mod ast;
pub mod graph;
pub mod sieve;
pub mod lsp;

#[cfg(feature = "daemon")]
pub mod daemon;

#[cfg(feature = "wasm")]
pub mod wasm;

// Re-export the primary entry point used by IDE plugins.
pub use sieve::{SieveConfig, SievePass};
