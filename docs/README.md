# context-sieve Documentation

Welcome to the official technical and operational documentation for **context-sieve** — the local-first Language Server Protocol (LSP) proxy that prunes dead-code context from LLM prompts via Tree-sitter Abstract Syntax Tree (AST) reachability analysis.

---

## Documentation Index

| Document | Description |
|---|---|
| [**Architecture**](ARCHITECTURE.md) | Deep dive into internal subsystems: LSP Intercept, Tree-sitter parsers, petgraph call-graph construction, BFS reachability query, and memory condensation. |
| [**Configuration**](CONFIGURATION.md) | Exhaustive reference for command-line arguments, environment variables, named pipe / socket configuration, and hop depth calibration. |
| [**LSP Specification**](LSP_SPECIFICATION.md) | Specification of the custom `$/sieve/condensedBuffer` JSON-RPC method, message framing, payload schemas, and upstream forwarding semantics. |
| [**Benchmarks & Performance**](BENCHMARKS.md) | Empirical throughput numbers, latency percentiles (< 1.4 ms/buffer), token reduction statistics (~68%), and Criterion testing methodology. |
| [**Contributing Guide**](CONTRIBUTING.md) | Guidelines for contributing code, adding support for new programming languages, running test suites, and formatting rules. |
| [**Discussion & RFC Guidelines**](DISCUSSIONS.md) | How the community conducts design discussions, proposes RFCs for AST pruning strategies, and tracks roadmap items. |
| [**Issue Guidelines & Templates**](ISSUE_GUIDELINES.md) | How to report bugs, submit reproducible examples, propose features, and follow issue lifecycle management. |

---

## Quick Navigation

- If you want a beginner's walkthrough on installing and running the binary on Windows, Mac, or Linux, see [**TUTORIAL.md**](../TUTORIAL.md).
- If you are building an IDE extension (VS Code, Cursor, Neovim), see [**LSP Specification**](LSP_SPECIFICATION.md) and [**Architecture**](ARCHITECTURE.md).
- If you want to add a new language parser (e.g., C++, Java, Kotlin), see [**Contributing Guide**](CONTRIBUTING.md).
