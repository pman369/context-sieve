/// context-sieve — daemon entry point.
///
/// Starts a Unix-domain socket (Linux/macOS) or named-pipe (Windows) server
/// that IDE plugins connect to. Each connection receives a JSON-RPC stream
/// identical to LSP; the daemon transparently forwards to the real language
/// server after executing the sieve pass on every `textDocument/didChange`
/// and `textDocument/didOpen` notification.
#[cfg(feature = "daemon")]
mod daemon_main {
    use clap::Parser;
    use std::path::PathBuf;
    use tracing_subscriber::{fmt, EnvFilter};

    use context_sieve::ast::Language;
    use context_sieve::daemon::{DaemonConfig, SieveDaemon};
    use context_sieve::{SieveConfig, SievePass};

    #[cfg(unix)]
    const DEFAULT_SOCKET: &str = "/tmp/context-sieve.sock";
    #[cfg(not(unix))]
    const DEFAULT_SOCKET: &str = r"\\.\pipe\context-sieve";

    /// Context-Sieve — LSP proxy for LLM prompt pruning.
    #[derive(Parser, Debug)]
    #[command(
        name = "context-sieve",
        version,
        about = "Local-first LSP proxy that prunes dead-code context from LLM prompts via Tree-sitter AST reachability analysis",
        long_about = None
    )]
    pub struct Cli {
        /// Socket path (Unix) or pipe name (Windows) to listen on.
        #[arg(short, long, default_value = DEFAULT_SOCKET)]
        pub socket: String,

        /// Upstream LSP server command (e.g. "rust-analyzer", "pyright").
        #[arg(short, long)]
        pub upstream: Option<String>,

        /// Path to a source file to analyze and prune directly in standalone test mode.
        #[arg(short, long)]
        pub file: Option<PathBuf>,

        /// Byte offset for cursor position in standalone file mode (default: 0).
        #[arg(short, long, default_value_t = 0)]
        pub cursor: usize,

        /// Maximum call-graph hop depth for reachability analysis.
        #[arg(short, long, default_value_t = 2)]
        pub depth: usize,

        /// Log level (trace | debug | info | warn | error).
        #[arg(short, long, default_value = "info")]
        pub log_level: String,
    }

    pub async fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();

        // 1. Standalone file condensation test mode
        if let Some(file_path) = cli.file {
            return run_file_mode(file_path, cli.cursor, cli.depth);
        }

        // 2. Interactive help if run with no upstream command
        if cli.upstream.is_none() {
            print_welcome_guide(&cli.socket);
            return Ok(());
        }

        // 3. Daemon mode
        fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new(&cli.log_level)),
            )
            .init();

        let upstream = cli.upstream.unwrap();

        println!("Context Sieve Daemon v{}", env!("CARGO_PKG_VERSION"));
        println!("Listening on: {}", cli.socket);
        println!("Upstream LSP: {}", upstream);
        println!("Hop depth:    {}", cli.depth);
        println!("Waiting for editor connections... (Press Ctrl+C to stop)\n");

        let config = DaemonConfig {
            socket_path: cli.socket,
            upstream_cmd: upstream,
            max_hop_depth: cli.depth,
        };

        let daemon = SieveDaemon::new(config).await?;
        daemon.run().await
    }

    fn print_welcome_guide(socket: &str) {
        println!("=================================================================");
        println!("          context-sieve v{}", env!("CARGO_PKG_VERSION"));
        println!("  Local-first LSP proxy for dead-code pruning from LLM prompts");
        println!("=================================================================\n");
        println!("How to use context-sieve:\n");
        println!("1. Run as a background LSP proxy for your IDE:");
        #[cfg(windows)]
        println!(
            "   context-sieve-daemon.exe --upstream rust-analyzer --socket \"{}\"",
            socket
        );
        #[cfg(not(windows))]
        println!(
            "   context-sieve-daemon --upstream rust-analyzer --socket \"{}\"",
            socket
        );
        println!("   (Point your IDE's LSP client at the socket/pipe above)\n");
        println!("2. Test reachability pruning directly on any source file:");
        println!("   context-sieve-daemon.exe --file path/to/source.rs --depth 2\n");
        println!("Options:");
        println!("  -u, --upstream <CMD>   Upstream LSP command (e.g. 'rust-analyzer', 'pyright')");
        println!("  -f, --file <PATH>      Analyze and prune a source file directly");
        println!("  -c, --cursor <BYTES>   Cursor byte offset for reachability root [default: 0]");
        println!(
            "  -s, --socket <PATH>    Socket or Named Pipe path [default: {}]",
            socket
        );
        println!("  -d, --depth <NUM>      Max call-graph hop depth [default: 2]");
        println!("  -h, --help             Show full command line help\n");

        #[cfg(windows)]
        {
            use std::io::{stdin, Read};
            println!("Press Enter to exit...");
            let _ = stdin().read(&mut [0u8]);
        }
    }

    fn run_file_mode(path: PathBuf, cursor: usize, depth: usize) -> anyhow::Result<()> {
        if !path.exists() {
            anyhow::bail!("File not found: {}", path.display());
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        let lang = match Language::from_extension(ext) {
            Some(l) => l,
            None => {
                anyhow::bail!(
                    "Unsupported file extension '.{}'. Supported: .rs, .py, .ts, .tsx, .go",
                    ext
                );
            }
        };

        let source = std::fs::read(&path)?;
        let mut sieve = SievePass::new();
        let config = SieveConfig {
            language: lang,
            max_hop_depth: depth,
            cursor_byte_offset: cursor,
        };

        println!("Analyzing {} ({} bytes)...", path.display(), source.len());
        match sieve.run(&source, &config) {
            Some(res) => {
                println!("Sieve Pass Complete:");
                println!("  Original size:        {} bytes", res.original_len);
                println!("  Pruned size:          {} bytes", res.pruned_len);
                println!("  Reduction:            {:.1}%", res.reduction_pct());
                println!("  Pruned dead symbols:  {}", res.pruned_symbol_count);
                println!("\n--- Condensed Code Preview ---\n");
                let text = String::from_utf8_lossy(&res.pruned);
                println!("{}", text);
            }
            None => {
                println!("Could not parse file with tree-sitter backend.");
            }
        }

        Ok(())
    }
}

#[cfg(feature = "daemon")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    daemon_main::run().await
}

#[cfg(not(feature = "daemon"))]
fn main() {
    eprintln!("Rebuild with --features daemon to run the native daemon.");
}
