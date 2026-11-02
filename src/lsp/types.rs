//! Minimal LSP JSON-RPC type definitions.
//!
//! Only the fields context-sieve actually reads are modelled here.  Unknown
//! fields are preserved via `serde_json::Value` pass-through so the proxy
//! can forward them unmodified.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── JSON-RPC envelope ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

impl JsonRpcMessage {
    /// True if this is a notification (no id, has method).
    pub fn is_notification(&self) -> bool {
        self.id.is_none() && self.method.is_some()
    }
}

// ── textDocument/didChange ───────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct DidChangeParams {
    #[serde(rename = "textDocument")]
    pub text_document: VersionedTextDocumentIdentifier,
    #[serde(rename = "contentChanges")]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextDocumentContentChangeEvent {
    /// If `range` is absent, `text` is the entire new content (full sync).
    pub range: Option<Range>,
    pub text: String,
}

// ── textDocument/didOpen ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct DidOpenParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentItem,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextDocumentItem {
    pub uri: String,
    #[serde(rename = "languageId")]
    pub language_id: String,
    pub version: i64,
    pub text: String,
}

// ── Shared primitives ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

// ── context-sieve extension: $/sieve/condensedBuffer ────────────────────────

/// Request params for the context-sieve custom LSP method.
#[derive(Debug, Clone, Deserialize)]
pub struct SieveRequest {
    pub uri: String,
    /// Cursor position to anchor reachability analysis.
    pub cursor: Position,
    /// Override the default hop depth (optional).
    pub max_hops: Option<usize>,
}

/// Response to a sieve condensation request.
#[derive(Debug, Clone, Serialize)]
pub struct SieveResponse {
    pub uri: String,
    /// The pruned source text (UTF-8).
    pub condensed_text: String,
    pub original_bytes: usize,
    pub pruned_bytes: usize,
    pub reduction_pct: f64,
}
