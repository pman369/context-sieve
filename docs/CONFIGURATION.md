# context-sieve Configuration Reference

This guide provides an exhaustive reference for configuring `context-sieve` through command-line arguments, environment variables, and editor integration settings.

---

## 1. Command-Line Options

When launching `context-sieve-daemon` (or the `context-sieve` binary), the following flags are available:

```
Usage: context-sieve-daemon [OPTIONS]
```

| Flag | Short | Default | Description |
|---|---|---|---|
| `--upstream <COMMAND>` | `-u` | _None (interactive help)_ | The upstream language server command (e.g. `rust-analyzer`, `pyright`, `gopls`). Required for daemon proxy mode. |
| `--file <PATH>` | `-f` | _None_ | Analyze and condense a single source file in standalone test mode without starting an IPC server. |
| `--socket <PATH>` | `-s` | `/tmp/context-sieve.sock` (Unix)<br>`\\.\pipe\context-sieve` (Win) | IPC channel path. On Linux/macOS, specifies the Unix domain socket path. On Windows, specifies the Named Pipe name. |
| `--depth <NUMBER>` | `-d` | `2` | Maximum call-graph reachability hop depth from the cursor root symbol. |
| `--cursor <OFFSET>` | `-c` | `0` | Byte offset of the active cursor position when running in standalone `--file` mode. |
| `--log-level <LEVEL>` | `-l` | `info` | Logging verbosity (`trace`, `debug`, `info`, `warn`, `error`). |
| `--help` | `-h` | — | Prints detailed help and argument descriptions. |
| `--version` | `-V` | — | Prints the version string (`context-sieve 0.1.1`). |

---

## 2. Environment Variables

`context-sieve` respects standard Rust and environment configuration conventions:

| Variable | Values | Purpose |
|---|---|---|
| `RUST_LOG` | `trace`, `debug`, `info`, `warn`, `error` | Configures `tracing-subscriber` filters. Overrides `--log-level`. |
| `RUST_BACKTRACE` | `0`, `1`, `full` | Enables detailed stack traces on unexpected panics. |
| `TMPDIR` / `TEMP` | Path string | Determines default temporary directory for sockets on Unix systems. |

---

## 3. Tuning the Reachability Depth (`--depth`)

The `--depth` parameter dictates how many hops across the symbol call graph `context-sieve` explores before declaring a symbol dead and pruning it.

```
[Cursor Position]
       │
       ▼
  (Root Symbol) ──[Hop 1]──► (Direct Helper Functions & Methods)
                                     │
                                     ▼
                                  [Hop 2] ──► (Secondary Utilities & Sub-helpers)
                                                     │
                                                     ▼
                                                  [Hop 3+] ──► (Deep Leaf Nodes)
```

| Depth | Pruning Aggressiveness | Token Reduction | Recommended Use Case |
|---|---|---|---|
| **Depth 1** | Maximum (~75% - 85%) | High | Focused edits, bug fixes inside isolated functions, small context models. |
| **Depth 2** | Balanced (~55% - 70%) | **Default / Ideal** | General coding, feature authoring, refactoring with immediate helper routines. |
| **Depth 3+** | Conservative (~30% - 45%) | Low | Deep algorithmic tracing, architectural audits, complex state machines. |

---

## 4. Editor Configuration Examples

### Visual Studio Code / Cursor

Configure your language extension to route through the `context-sieve` proxy:

#### Windows (Named Pipe)
In your `settings.json`:
```json
{
  "rust-analyzer.server.extraEnv": {
    "CONTEXT_SIEVE_SOCKET": "\\\\.\\pipe\\context-sieve"
  }
}
```

#### macOS & Linux (Unix Domain Socket)
In your `settings.json`:
```json
{
  "rust-analyzer.server.path": "/tmp/context-sieve.sock"
}
```

### Neovim (Built-in LSP)

In your `init.lua` or `lsp.lua`:
```lua
local lspconfig = require('lspconfig')

-- Attach Neovim to context-sieve's IPC socket
local socket_path = vim.fn.has("win32") == 1 
    and "\\\\.\\pipe\\context-sieve" 
    or "/tmp/context-sieve.sock"

lspconfig.rust_analyzer.setup {
    cmd = vim.lsp.rpc.connect(socket_path),
}
```
