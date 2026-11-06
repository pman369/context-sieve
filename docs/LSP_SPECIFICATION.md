# context-sieve LSP Protocol Specification

This document defines how `context-sieve` interacts with standard Language Server Protocol (LSP) clients and specifies the custom JSON-RPC extension method `$/sieve/condensedBuffer`.

---

## 1. Transport & Framing

### 1.1 Connection Channels
- **Unix (Linux & macOS)**: Unix Domain Socket at `/tmp/context-sieve.sock` (configurable via `--socket <PATH>`).
- **Windows**: Windows Named Pipe at `\\.\pipe\context-sieve` (configurable via `--socket <PIPE>`).

### 1.2 Framing Protocol
- Communications adhere to standard JSON-RPC 2.0 framing.
- In daemon proxy mode, messages are line-delimited JSON-RPC objects or standard LSP `Content-Length: <byte_count>\r\n\r\n` framed packets.

---

## 2. Standard LSP Pass-Through Semantics

`context-sieve` operates as a transparent proxy for all standard LSP methods:

```
Editor                                context-sieve                       Upstream LSP
  │                                         │                                   │
  │─── initialize / initialized ───────────►│─── forward directly ─────────────►│
  │◄── Response (capabilities) ─────────────│◄── Response ──────────────────────│
  │                                         │                                   │
  │─── textDocument/didOpen ───────────────►│ (cache buffer in memory)          │
  │                                         │─── forward directly ─────────────►│
  │                                         │                                   │
  │─── textDocument/didChange ─────────────►│ (update buffer in memory)         │
  │                                         │─── forward directly ─────────────►│
  │                                         │                                   │
  │─── textDocument/hover / definition ────►│─── forward directly ─────────────►│
  │◄── Response ────────────────────────────│◄── Response ──────────────────────│
```

1. **State Tracking**:
   - `textDocument/didOpen`: Inserts the file content into the daemon's internal buffer cache keyed by `uri`.
   - `textDocument/didChange`: Applies full or incremental changes to update the cached buffer.
   - `textDocument/didClose`: Evicts the file buffer from the memory cache.
2. **Transparent Forwarding**:
   - All diagnostic notifications, completion requests, code actions, and semantic tokens are forwarded untouched.

---

## 3. Custom Extension: `$/sieve/condensedBuffer`

IDE plugins invoke `$/sieve/condensedBuffer` whenever preparing a prompt payload for an LLM agent.

### 3.1 Request

- **Method**: `$/sieve/condensedBuffer`
- **Direction**: Client (Editor) → Server (`context-sieve`)

#### Parameters Schema (`SieveParams`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SieveParams",
  "type": "object",
  "required": ["uri", "cursor"],
  "properties": {
    "uri": {
      "type": "string",
      "description": "The URI of the document cached in the daemon (e.g. file:///project/src/main.rs)"
    },
    "cursor": {
      "type": "object",
      "required": ["line", "character"],
      "properties": {
        "line": {
          "type": "integer",
          "minimum": 0,
          "description": "0-based line number of the active cursor position"
        },
        "character": {
          "type": "integer",
          "minimum": 0,
          "description": "0-based character offset on the specified line"
        }
      }
    },
    "max_hops": {
      "type": "integer",
      "minimum": 1,
      "default": 2,
      "description": "Maximum call-graph reachability hop depth (default: 2)"
    }
  }
}
```

#### Example Request

```json
{
  "jsonrpc": "2.0",
  "id": 42,
  "method": "$/sieve/condensedBuffer",
  "params": {
    "uri": "file:///workspace/src/lib.rs",
    "cursor": {
      "line": 15,
      "character": 4
    },
    "max_hops": 2
  }
}
```

---

### 3.2 Response

- **Direction**: Server (`context-sieve`) → Client (Editor)

#### Result Schema (`SieveResponse`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SieveResponse",
  "type": "object",
  "required": ["uri", "condensed_text", "original_bytes", "pruned_bytes", "reduction_pct"],
  "properties": {
    "uri": {
      "type": "string",
      "description": "The URI of the processed document"
    },
    "condensed_text": {
      "type": "string",
      "description": "The pruned source text with unreachable symbol bodies removed"
    },
    "original_bytes": {
      "type": "integer",
      "description": "Original file size in bytes before condensation"
    },
    "pruned_bytes": {
      "type": "integer",
      "description": "Total count of bytes pruned from the file"
    },
    "reduction_pct": {
      "type": "number",
      "description": "Percentage reduction in payload size (0.0 to 100.0)"
    }
  }
}
```

#### Example Response

```json
{
  "jsonrpc": "2.0",
  "id": 42,
  "result": {
    "uri": "file:///workspace/src/lib.rs",
    "condensed_text": "pub fn active_handler() {\n    compute_result();\n}\n\nfn compute_result() -> u32 {\n    42\n}\n",
    "original_bytes": 18432,
    "pruned_bytes": 12100,
    "reduction_pct": 65.65
  }
}
```

---

### 3.3 Error Handling

If the document URI is unknown or the file cannot be parsed, `context-sieve` responds with standard JSON-RPC error codes:

| Error Code | Meaning | Remediation |
|---|---|---|
| `-32602` | Invalid params | Missing `uri` or `cursor` coordinates. |
| `-32001` | Document not found | Document URI has not been sent via `didOpen`. |
| `-32002` | Unsupported language | The file extension does not match a supported Tree-sitter grammar. |

When an error occurs, client extensions should fall back gracefully to injecting the raw file buffer unchanged.
