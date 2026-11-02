//! Async daemon — IPC server and upstream LSP bridge.
//!
//! On Linux/macOS this listens on a Unix-domain socket. On Windows it uses a
//! named pipe. Each accepted connection spawns a Tokio task that owns an
//! [`LspIntercept`] and bridges messages between the editor and the upstream
//! language server process.

use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tracing::{error, info, warn};

use crate::lsp::{
    intercept::{InterceptAction, LspIntercept},
    types::JsonRpcMessage,
};

/// Daemon configuration.
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    /// Path to the Unix-domain socket / Windows named pipe.
    pub socket_path: String,
    /// Shell command to launch the upstream LSP (e.g. "rust-analyzer").
    pub upstream_cmd: String,
    /// Default max hop depth for sieve passes.
    pub max_hop_depth: usize,
}

#[cfg(unix)]
pub struct SieveDaemon {
    config: Arc<DaemonConfig>,
    listener: tokio::net::UnixListener,
}

#[cfg(unix)]
impl SieveDaemon {
    pub async fn new(config: DaemonConfig) -> anyhow::Result<Self> {
        // Remove stale socket file if it exists.
        let _ = std::fs::remove_file(&config.socket_path);

        let listener = tokio::net::UnixListener::bind(&config.socket_path)?;
        info!(socket = %config.socket_path, "context-sieve daemon listening (unix domain socket)");

        Ok(Self {
            config: Arc::new(config),
            listener,
        })
    }

    /// Accept connections in a loop until the process is killed.
    pub async fn run(self) -> anyhow::Result<()> {
        loop {
            match self.listener.accept().await {
                Ok((stream, _addr)) => {
                    let config = Arc::clone(&self.config);
                    tokio::spawn(async move {
                        let (read, write) = stream.into_split();
                        if let Err(e) = handle_connection(read, write, config).await {
                            error!("connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    error!("accept error: {e}");
                }
            }
        }
    }
}

#[cfg(windows)]
pub struct SieveDaemon {
    config: Arc<DaemonConfig>,
    pipe_name: String,
}

#[cfg(windows)]
fn normalize_pipe_name(path: &str) -> String {
    if path.starts_with(r"\\.\pipe\") {
        path.to_string()
    } else {
        let clean = path
            .trim_start_matches('/')
            .trim_start_matches('\\')
            .replace(['/', '\\'], "-");
        format!(r"\\.\pipe\{}", clean)
    }
}

#[cfg(windows)]
impl SieveDaemon {
    pub async fn new(config: DaemonConfig) -> anyhow::Result<Self> {
        let pipe_name = normalize_pipe_name(&config.socket_path);
        info!(pipe = %pipe_name, "context-sieve daemon listening (windows named pipe)");

        Ok(Self {
            config: Arc::new(config),
            pipe_name,
        })
    }

    /// Accept connections in a loop until the process is killed.
    pub async fn run(self) -> anyhow::Result<()> {
        use tokio::net::windows::named_pipe::ServerOptions;

        let mut server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(&self.pipe_name)?;

        loop {
            server.connect().await?;
            let connected_client = server;

            // Prepare next instance for future connections
            server = ServerOptions::new().create(&self.pipe_name)?;

            let config = Arc::clone(&self.config);
            tokio::spawn(async move {
                let (read, write) = tokio::io::split(connected_client);
                if let Err(e) = handle_connection(read, write, config).await {
                    error!("connection error: {e}");
                }
            });
        }
    }
}

/// Handles a single editor connection end-to-end.
async fn handle_connection<R, W>(
    editor_read: R,
    mut editor_write: W,
    config: Arc<DaemonConfig>,
) -> anyhow::Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
    W: tokio::io::AsyncWrite + Unpin,
{
    // Launch upstream LSP process.
    let mut upstream_proc = Command::new(&config.upstream_cmd)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let upstream_stdin = upstream_proc.stdin.take().unwrap();
    let upstream_stdout = upstream_proc.stdout.take().unwrap();

    let mut editor_reader = BufReader::new(editor_read);
    let mut upstream_writer = tokio::io::BufWriter::new(upstream_stdin);
    let mut upstream_reader = BufReader::new(upstream_stdout);

    let mut intercept = LspIntercept::new(config.max_hop_depth);
    let mut line = String::new();

    // Simple line-delimited JSON-RPC framing (for demonstration).
    // Production: implement the LSP Content-Length header framing.
    loop {
        line.clear();
        let n = editor_reader.read_line(&mut line).await?;
        if n == 0 {
            break;
        } // EOF

        let msg: JsonRpcMessage = match serde_json::from_str(line.trim()) {
            Ok(m) => m,
            Err(e) => {
                warn!("JSON parse error: {e}");
                continue;
            }
        };

        match intercept.process(msg) {
            InterceptAction::Forward(fwd) => {
                let serialised = serde_json::to_string(&fwd)?;
                upstream_writer.write_all(serialised.as_bytes()).await?;
                upstream_writer.write_all(b"\n").await?;
                upstream_writer.flush().await?;

                // Read one response from upstream and forward back to editor.
                let mut resp_line = String::new();
                let rn = upstream_reader.read_line(&mut resp_line).await?;
                if rn > 0 {
                    editor_write.write_all(resp_line.as_bytes()).await?;
                }
            }
            InterceptAction::Respond(payload) => {
                let resp = serde_json::to_string(&payload)?;
                editor_write.write_all(resp.as_bytes()).await?;
                editor_write.write_all(b"\n").await?;
            }
            InterceptAction::Drop => {}
        }
    }

    Ok(())
}
