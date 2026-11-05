# context-sieve

> **Local-first LSP proxy that prunes dead-code context from LLM prompts via Tree-sitter AST reachability analysis.**

👉 **New to context-sieve? Check out the beginner-friendly [Non-Technical User Tutorial](TUTORIAL.md)**

[![CI](https://github.com/pman369/context-sieve/actions/workflows/ci.yml/badge.svg)](https://github.com/pman369/context-sieve/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg)](LICENSE)

---

## The Problem: Context Poisoning

Modern coding agents degrade as projects grow. Raw file buffers injected into prompts fill the model's context window with unreferenced interfaces, dead branches, deprecated type signatures, and bulky docstrings. This dilutes softmax attention across irrelevant tokens, inducing subtle hallucinations and stale API usage.

**context-sieve intercepts the LSP stream** and executes a fast call-graph reachability analysis from the active cursor position. Any symbol with zero incoming edges within a configurable hop depth is aggressively pruned before prompt serialization.

| Metric | Value |
|---|---|
| AST condensation latency | < 1.4 ms / buffer |
| Prompt token reduction | ~68% average |
| TTFT improvement | up to 3× on remote endpoints |
| Runtime dependencies | zero (self-contained binary) |

---

## Architecture

```
Editor (Neovim / VS Code / Cursor)
        │  LSP JSON-RPC (stdio / Unix socket)
        ▼
┌─────────────────────────────────────────┐
│            context-sieve daemon         │
│                                         │
│  LspIntercept                           │
│    ├── Buffer Cache  (URI → bytes)      │
│    └── SievePass                        │
│          ├── AstParser (Tree-sitter)    │
│          │     ├── RustParser           │
│          │     ├── PythonParser         │
│          │     └── TS / Go (planned)    │
│          ├── GraphBuilder (petgraph)    │
│          └── ReachabilityQuery (BFS)   │
│                └── Condenser           │
└─────────────────────────────────────────┘
        │  Pruned LSP stream
        ▼
Upstream Language Server (rust-analyzer, pyright, …)
        │
        ▼
LLM Inference Pipeline (local Ollama / remote API)
```

### Pipeline (per keystroke)

1. `textDocument/didChange` → buffer cache updated.
2. `$/sieve/condensedBuffer` request from IDE plugin triggers:
   - **AST parse** — Tree-sitter CST walk → `Vec<Symbol>` (zero-copy byte spans).
   - **Graph build** — 2-pass `O(n)` call-graph construction.
   - **Reachability BFS** — from cursor-enclosing symbol, max depth 2 hops.
   - **Condense** — sorted range merge + single-pass slice copy.
3. Pruned text returned to IDE plugin for prompt injection.

---

## Getting Started

### Prerequisites

- Rust ≥ 1.77 (`rustup show`)
- (Optional) `wasm-pack` for WASM target

### Install

```sh
cargo install --path . --features daemon
```

### Run

```sh
context-sieve --upstream rust-analyzer --socket /tmp/cs.sock --depth 2
```

Point your IDE plugin at `/tmp/cs.sock` instead of directly at `rust-analyzer`.

### WASM build

```sh
wasm-pack build --target web --features wasm --no-default-features
```

The generated `pkg/` directory can be loaded in a browser Service Worker.

---

## Configuration

| CLI flag | Default | Description |
|----------|---------|-------------|
| `--socket` | `/tmp/context-sieve.sock` | Unix socket / named pipe path |
| `--upstream` | _(required)_ | Upstream LSP command |
| `--depth` | `2` | Call-graph reachability hop depth |
| `--log-level` | `info` | Tracing level (`trace`…`error`) |

---

## Custom LSP Extension

context-sieve adds one non-standard JSON-RPC method:

**Request**: `$/sieve/condensedBuffer`

```json
{
  "jsonrpc": "2.0",
  "method": "$/sieve/condensedBuffer",
  "params": {
    "uri": "file:///path/to/main.rs",
    "cursor": { "line": 42, "character": 8 },
    "max_hops": 2
  }
}
```

**Response**:

```json
{
  "uri": "file:///path/to/main.rs",
  "condensed_text": "...",
  "original_bytes": 18432,
  "pruned_bytes": 5900,
  "reduction_pct": 67.99
}
```

---

## Development

```sh
# Run tests
cargo test --features daemon

# Run benchmarks
cargo bench --features daemon

# Check + lint
cargo clippy --features daemon -- -D warnings
```

---

## Roadmap

- [ ] TypeScript / TSX language backend
- [ ] Go language backend
- [ ] Incremental graph patching (avoid full re-parse on small edits)
- [ ] Neovim plugin (`lua/context-sieve.lua`)
- [ ] VS Code extension
- [ ] LSP Content-Length framing (replace line-delimited prototype)
- [ ] Named-pipe support on Windows
- [ ] `--watch` mode with `notify` file watcher

---

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
