# Issue Guidelines & Templates

This document explains our issue triage process and provides templates for reporting bugs, requesting features, and flagging AST pruning regressions.

---

## 1. Before Filing an Issue

To ensure issues are resolved quickly, please check the following:
1. **Search Existing Issues**: Search [Open and Closed Issues](https://github.com/pman369/context-sieve/issues) to verify your bug or feature request has not already been submitted.
2. **Update to the Latest Release**: Confirm the issue reproduces on the latest version of `context-sieve` (e.g. `v0.1.1` or `main`).
3. **Isolate the Behavior**: Verify whether the issue is caused by `context-sieve` or the upstream language server (e.g. `rust-analyzer`).

---

## 2. Issue Templates

### 🐛 Bug Report Template

When filing a bug, copy and paste this markdown template into your issue description:

```markdown
### Environment
- **context-sieve Version**: [e.g. v0.1.1]
- **Operating System**: [e.g. Windows 11 / macOS 14.5 Apple Silicon / Ubuntu 24.04]
- **Editor / Client**: [e.g. VS Code 1.90 / Neovim 0.10 / Cursor]
- **Upstream Language Server**: [e.g. rust-analyzer 2026-10-30 / pyright 1.1]

### Description of the Bug
A clear and concise description of what went wrong.

### Steps to Reproduce
1. Start context-sieve with `context-sieve-daemon -u <upstream>`
2. Connect editor or run standalone: `context-sieve-daemon --file <file>`
3. See error...

### Minimal Reproducible Code Sample
Provide the smallest source file that causes the failure:
```rust
// Paste minimal snippet here
```

### Expected Behavior
A clear description of what you expected to happen.

### Actual Output & Logs
Run with `--log-level trace` and paste the terminal logs here:
```text
[Paste logs here]
```
```

---

### ⚠️ Incorrect Pruning / AST Regression Template

Use this template if `context-sieve` inadvertently pruned code that was actually reachable, or left behind dead code that should have been pruned:

```markdown
### Language & Target
- **Language**: [e.g. Rust / Python / TypeScript / Go]
- **Depth Setting**: [e.g. --depth 2]
- **Cursor Position**: [Line X, Character Y or byte offset]

### Source Code Fixture
```rust
// Code before pruning
```

### What context-sieve Returned
```rust
// The condensed code emitted by context-sieve
```

### The Problem
- [ ] Active code was accidentally deleted (Semantic breakage).
- [ ] Dead code was improperly retained.

### Explanation of Why This Symbol is Reachable / Unreachable
Explain why the reachability query made the wrong call (e.g. macro invocation, dynamic dispatch, decorator, trait implementation).
```

---

### ✨ Feature Request Template

Use this template to suggest enhancements or new language parsers:

```markdown
### Feature Summary
One-sentence description of the proposed feature.

### Motivation & Problem
What pain point or use-case does this address? (e.g., "I want to use context-sieve with C++ codebases").

### Proposed Solution
Describe how you envision the feature working:
- CLI flags / settings
- Architectural changes
- Language grammar additions

### Alternatives Considered
What workarounds or alternate designs did you consider?
```

---

## 3. Issue Lifecycle

1. **New**: Awaiting maintainer triage.
2. **Needs-Repro**: Requires a minimal code snippet to reproduce.
3. **Confirmed**: Successfully reproduced by maintainers; prioritized.
4. **In Progress**: An active PR or branch is implementing the resolution.
5. **Resolved**: Merged to `main` and verified in CI; closed upon next release tag.
