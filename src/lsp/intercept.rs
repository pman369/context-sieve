//! LSP proxy intercept layer.
//!
//! [`LspIntercept`] parses incoming JSON-RPC messages, decides whether to run
//! the sieve pass, updates the buffer cache, and signals the daemon whether to
//! forward the (possibly mutated) message to upstream or drop it.

use std::collections::HashMap;
use std::path::Path;

use serde_json::Value;
use tracing::{debug, warn};

use crate::ast::Language;
use crate::lsp::types::{
    DidChangeParams, DidOpenParams, JsonRpcMessage, SieveRequest, SieveResponse,
};
use crate::sieve::{SieveConfig, SievePass};

/// Action the daemon should take after intercept.
pub enum InterceptAction {
    /// Forward the (possibly modified) message to upstream LSP.
    Forward(JsonRpcMessage),
    /// Respond directly to the client with the given payload (don't forward).
    Respond(Value),
    /// Drop the message entirely.
    Drop,
}

/// Stateful interceptor — one per connection.
pub struct LspIntercept {
    sieve: SievePass,
    /// Cache of per-URI source bytes (most recent full text).
    buffers: HashMap<String, Vec<u8>>,
    max_hop_depth: usize,
}

impl LspIntercept {
    pub fn new(max_hop_depth: usize) -> Self {
        Self {
            sieve: SievePass::new(),
            buffers: HashMap::new(),
            max_hop_depth,
        }
    }

    /// Process an inbound JSON-RPC message from the editor.
    pub fn process(&mut self, msg: JsonRpcMessage) -> InterceptAction {
        let method = match msg.method.as_deref() {
            Some(m) => m,
            None    => return InterceptAction::Forward(msg),
        };

        match method {
            "textDocument/didOpen"   => self.handle_did_open(msg),
            "textDocument/didChange" => self.handle_did_change(msg),
            "textDocument/didSave"   => self.handle_did_save(msg),
            "$/sieve/condensedBuffer" => self.handle_sieve_request(msg),
            _                        => InterceptAction::Forward(msg),
        }
    }

    // ── Handlers ─────────────────────────────────────────────────────────────

    fn handle_did_open(&mut self, msg: JsonRpcMessage) -> InterceptAction {
        if let Some(params) = msg.params.as_ref() {
            if let Ok(p) = serde_json::from_value::<DidOpenParams>(params.clone()) {
                debug!(uri = %p.text_document.uri, "caching opened buffer");
                self.buffers
                    .insert(p.text_document.uri, p.text_document.text.into_bytes());
            }
        }
        InterceptAction::Forward(msg)
    }

    fn handle_did_change(&mut self, msg: JsonRpcMessage) -> InterceptAction {
        if let Some(params) = msg.params.as_ref() {
            if let Ok(p) = serde_json::from_value::<DidChangeParams>(params.clone()) {
                for change in &p.content_changes {
                    if change.range.is_none() {
                        // Full-document sync.
                        self.buffers.insert(
                            p.text_document.uri.clone(),
                            change.text.clone().into_bytes(),
                        );
                    }
                    // Incremental sync: apply range patch.
                    // TODO: implement byte-range patching for incremental sync.
                }
            }
        }
        InterceptAction::Forward(msg)
    }

    fn handle_did_save(&mut self, msg: JsonRpcMessage) -> InterceptAction {
        // We retain the buffer cache across saves (no flush needed).
        InterceptAction::Forward(msg)
    }

    fn handle_sieve_request(&mut self, msg: JsonRpcMessage) -> InterceptAction {
        let req: SieveRequest = match msg.params
            .as_ref()
            .and_then(|p| serde_json::from_value(p.clone()).ok())
        {
            Some(r) => r,
            None    => {
                warn!("Invalid $/sieve/condensedBuffer params");
                return InterceptAction::Drop;
            }
        };

        let source = match self.buffers.get(&req.uri) {
            Some(b) => b.clone(),
            None    => {
                warn!(uri = %req.uri, "buffer not in cache for sieve request");
                return InterceptAction::Drop;
            }
        };

        let lang = language_from_uri(&req.uri);
        let cursor_byte = position_to_byte_offset(&source, req.cursor.line, req.cursor.character);

        let config = SieveConfig {
            language: lang.unwrap_or(Language::Rust),
            max_hop_depth: req.max_hops.unwrap_or(self.max_hop_depth),
            cursor_byte_offset: cursor_byte,
        };

        match self.sieve.run(&source, &config) {
            Some(result) => {
                let resp = SieveResponse {
                    uri: req.uri,
                    condensed_text: String::from_utf8_lossy(&result.pruned).into_owned(),
                    original_bytes: result.original_len,
                    pruned_bytes:   result.pruned_len,
                    reduction_pct:  result.reduction_pct(),
                };
                InterceptAction::Respond(serde_json::to_value(resp).unwrap_or(Value::Null))
            }
            None => InterceptAction::Drop,
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn language_from_uri(uri: &str) -> Option<Language> {
    let ext = Path::new(uri).extension()?.to_str()?;
    Language::from_extension(ext)
}

/// Convert (line, character) LSP position to a byte offset in `source`.
/// Both are 0-indexed; character is a UTF-16 code unit count per LSP spec.
fn position_to_byte_offset(source: &[u8], line: u32, character: u32) -> usize {
    let text = std::str::from_utf8(source).unwrap_or("");
    let mut cur_line = 0u32;
    let mut byte_pos = 0usize;

    for (i, ch) in text.char_indices() {
        if cur_line == line {
            // Approximate: treat character as byte offset within the line.
            return byte_pos + character as usize;
        }
        if ch == '\n' {
            cur_line += 1;
            byte_pos = i + 1;
        }
    }
    byte_pos
}
