# Contributing to context-sieve

Thank you for your interest in contributing to **context-sieve**! We welcome bug fixes, performance improvements, documentation enhancements, and new language parsers.

---

## 1. Code of Conduct

We are committed to providing a welcoming, inclusive, and harassment-free environment for all contributors. Please treat everyone with respect and empathy.

---

## 2. Development Setup

### Prerequisites
- **Rust Toolchain**: Rust $\ge$ 1.77 (`rustup show`)
- **C Compiler**: `clang` or `gcc` (or MSVC on Windows) to build native Tree-sitter C grammar libraries
- **Git**: For version control
- (Optional) **wasm-pack**: `cargo install wasm-pack` if working on WASM targets

### Clone & Build
```bash
git clone https://github.com/pman369/context-sieve.git
cd context-sieve

# Check code compilation
cargo check --features daemon

# Build debug binary
cargo build --features daemon

# Run the test suite
cargo test --features daemon
```

---

## 3. Code Standards & Quality Checks

All Pull Requests must pass our CI validation suite before merging. Run these checks locally before opening a PR:

### 3.1 Formatting
Code must be formatted with `rustfmt`:
```bash
cargo fmt --all -- --check
```
To automatically apply formatting:
```bash
cargo fmt --all
```

### 3.2 Linter (Clippy)
Zero warnings allowed under `-D warnings`:
```bash
cargo clippy --all-targets --features daemon -- -D warnings
```

### 3.3 Unit and Integration Tests
```bash
cargo test --features daemon
```

### 3.4 WASM Build Verification
Ensure the `wasm32-unknown-unknown` target remains decoupled from C headers:
```bash
wasm-pack build --target web --features wasm --no-default-features
```

---

## 4. How to Add a New Language Parser

Adding support for a new language (e.g. `go`, `typescript`, `cpp`) involves three steps:

### Step 1: Add the Tree-sitter Dependency
In `Cargo.toml`, add the optional Tree-sitter grammar:
```toml
[dependencies]
tree-sitter-go = { version = "0.21", optional = true }

[features]
daemon = [
    # ...
    "dep:tree-sitter-go",
]
```

### Step 2: Implement `LanguageParser`
Create `src/ast/languages/go.rs` and implement the symbol extractor:
```rust
#![cfg(not(target_arch = "wasm32"))]

use crate::ast::languages::LanguageParser;
use crate::ast::symbol::{Span, Symbol, SymbolKind};

pub struct GoParser;

impl LanguageParser for GoParser {
    fn parse_symbols(&self, tree: &tree_sitter::Tree, source: &[u8]) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        // Walk the tree-sitter CST and push definitions & call sites
        symbols
    }
}
```

### Step 3: Register in `AstParser` and `Language`
1. Add the enum variant in `src/ast/parser.rs` (`Language::Go`).
2. Add the file extension mapping in `Language::from_extension`:
   ```rust
   "go" => Some(Language::Go),
   ```
3. Instantiate the parser in `AstParser::new()`.
4. Add an integration test in `tests/integration_test.rs` validating that symbol extraction and pruning reduce payloads correctly.

---

## 5. Commit & Pull Request Guidelines

We follow **Conventional Commits**:
- `feat(...)`: A new feature or capability.
- `fix(...)`: A bug fix.
- `perf(...)`: Performance optimization.
- `docs(...)`: Documentation additions or fixes.
- `test(...)`: Adding or refactoring tests.
- `ci(...)`: GitHub Actions or build configuration changes.

### Submitting a PR
1. Create a feature branch: `git checkout -b feat/my-new-feature`
2. Commit your changes with a descriptive message.
3. Push to your fork: `git push origin feat/my-new-feature`
4. Open a Pull Request against `main`. Provide a concise summary of changes and reference any relevant issue numbers.
