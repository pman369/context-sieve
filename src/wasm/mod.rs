//! WebAssembly bindings for context-sieve.
//!
//! Exposes the sieve pass as a single `condense()` JavaScript function that
//! accepts a source string and a JSON config object, returning the pruned
//! source string.  The WASM module is designed to be loaded inside a
//! SharedWorker or a Service Worker — no DOM access is required.
//!
//! # Build
//!
//! ```sh
//! wasm-pack build --target web --features wasm --no-default-features
//! ```

use wasm_bindgen::prelude::*;

use crate::ast::Language;
use crate::sieve::{SieveConfig, SievePass};

/// Prune `source` using reachability analysis anchored at `cursor_offset`.
///
/// * `source`        — UTF-8 source text
/// * `language`      — `"rust"` | `"python"` | `"typescript"` | `"go"`
/// * `cursor_offset` — byte offset of the editor cursor
/// * `max_hops`      — call-graph depth (default 2)
///
/// Returns the pruned source text, or the original text if sieving fails.
#[wasm_bindgen]
pub fn condense(source: &str, language: &str, cursor_offset: usize, max_hops: usize) -> String {
    #[cfg(feature = "wasm")]
    console_error_panic_hook::set_once();

    let lang = match language {
        "rust" => Language::Rust,
        "python" => Language::Python,
        "typescript" => Language::TypeScript,
        "go" => Language::Go,
        _ => return source.to_owned(),
    };

    let config = SieveConfig {
        language: lang,
        max_hop_depth: if max_hops == 0 { 2 } else { max_hops },
        cursor_byte_offset: cursor_offset,
    };

    let mut pass = SievePass::new();
    match pass.run(source.as_bytes(), &config) {
        Some(result) => String::from_utf8_lossy(&result.pruned).into_owned(),
        None => source.to_owned(),
    }
}

/// Returns the reduction percentage for a given source (0.0–100.0).
#[wasm_bindgen]
pub fn reduction_pct(source: &str, language: &str, cursor_offset: usize, max_hops: usize) -> f64 {
    let lang = match language {
        "rust" => Language::Rust,
        "python" => Language::Python,
        "typescript" => Language::TypeScript,
        "go" => Language::Go,
        _ => return 0.0,
    };

    let config = SieveConfig {
        language: lang,
        max_hop_depth: if max_hops == 0 { 2 } else { max_hops },
        cursor_byte_offset: cursor_offset,
    };

    let mut pass = SievePass::new();
    pass.run(source.as_bytes(), &config)
        .map(|r| r.reduction_pct())
        .unwrap_or(0.0)
}
