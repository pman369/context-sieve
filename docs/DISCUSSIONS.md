# Community Discussions & RFC Guidelines

We welcome community discussion, design feedback, and collaborative problem-solving around **context-sieve**. This document outlines how discussions are organized and explains our Request for Comments (RFC) process.

---

## 1. Where to Discuss

To keep the project organized, we separate bug tracking from open-ended discussion:

| Topic | Preferred Channel |
|---|---|
| **Bug Reports & Regressions** | [GitHub Issues](https://github.com/pman369/context-sieve/issues) |
| **Questions & Troubleshooting** | [GitHub Discussions (Q&A)](https://github.com/pman369/context-sieve/discussions/categories/q-a) |
| **Feature Ideas & Pruning Strategies** | [GitHub Discussions (Ideas)](https://github.com/pman369/context-sieve/discussions/categories/ideas) |
| **New Editor Plugins (VS Code, Neovim)** | [GitHub Discussions (Show and Tell)](https://github.com/pman369/context-sieve/discussions/categories/show-and-tell) |

---

## 2. Discussion Categories

### 💡 Ideas & Strategies
Discuss new algorithmic heuristics for dead-code pruning:
- Should we prune docstrings on unreferenced types?
- How should cross-file reachability be indexed without full-workspace overhead?
- Incremental graph patching vs full re-parsing on small edits.

### ❓ Q&A
Ask for guidance on:
- Setting up the daemon on specific operating systems or remote SSH hosts.
- Integrating `context-sieve` with custom agent frameworks (e.g. LangChain, AutoGen, custom LSP proxies).
- Configuring editor extensions.

### 🚀 Show and Tell
Share your custom setups, benchmark runs on large proprietary codebases, or editor configurations with the community!

---

## 3. The RFC (Request for Comments) Process

For significant architectural shifts, new language parser paradigms, or breaking protocol adjustments, we use an RFC process:

```
[Idea in Discussions] ──► [RFC Draft Markdown] ──► [Community Review] ──► [Approved Implementation]
```

### When is an RFC Required?
- Adding cross-file workspace indexing.
- Changing the JSON-RPC schema of `$/sieve/condensedBuffer`.
- Introducing new external runtime dependencies.
- Modifying the core condensation strategy or byte-slicing logic.

### How to Submit an RFC
1. Start an initial thread in **GitHub Discussions → Ideas** to gauge community interest.
2. If consensus emerges, write an RFC document following this outline:
   - **Summary**: One-paragraph explanation of the change.
   - **Motivation**: What problem does this solve? What metrics are affected?
   - **Detailed Design**: Precise types, algorithms, and data structures.
   - **Performance Impact**: Latency, memory overhead, and token reduction trade-offs.
   - **Drawbacks & Alternatives**: What other designs were considered?
3. Open a Pull Request adding your RFC to `docs/rfcs/000X-my-proposal.md`.
4. After community and maintainer review, the RFC is merged and ready for implementation.
