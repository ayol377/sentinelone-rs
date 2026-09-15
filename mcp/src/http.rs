//! HTTP transport: a single JSON-RPC endpoint (MCP "Streamable HTTP", non-stream
//! mode — POST a request, get one JSON response). This is what `docker compose`
//! runs as a long-lived service. Plus `GET /health` for container healthchecks.

use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::mcp::{parse_error, process, ServerState};

pub async fn run(state: Arc<ServerState>, bind: &str) -> std::io::Result<()> {
    let app = Router::new()
        .route("/", post(rpc).get(info))
        .route("/mcp", post(rpc))
        .route("/health", get(health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(bind).await?;
    eprintln!("sentinelone-mcp: HTTP transport listening on http://{bind} (POST / or /mcp)");
    axum::serve(listener, app).await
}

async fn health() -> &'static str {
    "ok"
}

async fn info() -> &'static str {
    "sentinelone-mcp: send JSON-RPC via POST to / or /mcp"
}

async fn rpc(State(state): State<Arc<ServerState>>, body: Bytes) -> Response {
    let value = match serde_json::from_slice::<serde_json::Value>(&body) {
        Ok(v) => v,
        Err(_) => return (StatusCode::OK, Json(parse_error())).into_response(),
    };
    match process(&state, value).await {
        Some(resp) => (StatusCode::OK, Json(resp)).into_response(),
        // Notification(s) only — nothing to return.
        None => StatusCode::ACCEPTED.into_response(),
    }
}
