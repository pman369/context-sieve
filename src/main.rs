/// context-sieve — daemon entry point.
///
/// Starts a Unix-domain socket (Linux/macOS) or named-pipe (Windows) server
/// that IDE plugins connect to.  Each connection receives a JSON-RPC stream
/// identical to LSP; the daemon transparently forwards to the real language
/// server after executing the sieve pass on every `textDocument/didChange`
/// and `textDocument/didOpen` notification.
#[cfg(feature = "daemon")]
mod daemon_main {
    use clap::Parser;
    use tracing_subscriber::{fmt, EnvFilter};

    use context_sieve::daemon::{DaemonConfig, SieveDaemon};

    #[cfg(unix)]
    const DEFAULT_SOCKET: &str = "/tmp/context-sieve.sock";
    #[cfg(not(unix))]
    const DEFAULT_SOCKET: &str = r"\\.\pipe\context-sieve";

    /// Context-Sieve — LSP proxy for LLM prompt pruning.
    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Cli {
        /// Socket path (Unix) or pipe name (Windows) to listen on.
        #[arg(short, long, default_value = DEFAULT_SOCKET)]
        pub socket: String,

        /// Upstream LSP server command (e.g. "rust-analyzer").
        #[arg(short, long)]
        pub upstream: String,

        /// Maximum call-graph hop depth for reachability analysis.
        #[arg(short, long, default_value_t = 2)]
        pub depth: usize,

        /// Log level (trace | debug | info | warn | error).
        #[arg(short, long, default_value = "info")]
        pub log_level: String,
    }

    pub async fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();

        fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new(&cli.log_level)),
            )
            .init();

        let config = DaemonConfig {
            socket_path: cli.socket,
            upstream_cmd: cli.upstream,
            max_hop_depth: cli.depth,
        };

        let daemon = SieveDaemon::new(config).await?;
        daemon.run().await
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
