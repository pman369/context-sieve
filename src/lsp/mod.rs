//! LSP JSON-RPC types and proxy intercept layer.
//!
//! context-sieve intercepts a subset of LSP notifications to trigger sieve
//! passes.  All other messages are forwarded to the upstream language server
//! verbatim.
//!
//! ## Intercepted messages
//!
//! | Method | Action |
//! |--------|--------|
//! | `textDocument/didOpen`   | Parse + build graph; store in buffer cache. |
//! | `textDocument/didChange` | Incremental re-parse; rebuild affected subgraph. |
//! | `textDocument/didSave`   | Flush buffer cache entry. |
//! | `$/sieve/condensedBuffer`| (extension) Return pruned buffer on demand. |

pub mod intercept;
pub mod types;

pub use intercept::LspIntercept;
