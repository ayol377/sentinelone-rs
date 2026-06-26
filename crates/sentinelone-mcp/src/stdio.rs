//! Stdio transport: newline-delimited JSON-RPC on stdin/stdout. This is the
//! transport MCP desktop/IDE clients use when they spawn the server process
//! (including `docker run -i`).

use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::mcp::{parse_error, process, ServerState};

pub async fn run(state: Arc<ServerState>) -> std::io::Result<()> {
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let response = match serde_json::from_str::<serde_json::Value>(line) {
            Ok(v) => process(&state, v).await,
            Err(_) => Some(parse_error()),
        };
        if let Some(resp) = response {
            let text = serde_json::to_string(&resp).unwrap_or_else(|_| "null".into());
            stdout.write_all(text.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }
    }
    Ok(())
}
