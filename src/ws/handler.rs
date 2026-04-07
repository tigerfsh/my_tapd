use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use redis::AsyncCommands;
use serde::Deserialize;
use tokio::time::{Duration, timeout};

use crate::service::auth_service::jwt_verify;

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

pub struct WsState {
    pub redis: redis::aio::ConnectionManager,
    pub jwt_secret: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<Arc<WsState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, query.token, state))
}

async fn handle_socket(mut socket: WebSocket, token: String, state: Arc<WsState>) {
    // Verify JWT token; close connection if invalid
    let claims = match jwt_verify(&token, &state.jwt_secret) {
        Ok(c) => c,
        Err(_) => return,
    };

    let user_id: i64 = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => return,
    };

    let queue_key = format!("notifications:{}", user_id);
    let mut conn = state.redis.clone();

    loop {
        // Poll Redis for pending notifications (RPOP)
        let msg: Option<String> = conn.rpop(&queue_key, None).await.unwrap_or(None);

        if let Some(payload) = msg {
            let ws_msg = serde_json::json!({
                "type": "notification",
                "data": serde_json::from_str::<serde_json::Value>(&payload)
                    .unwrap_or(serde_json::Value::Null)
            });
            if socket
                .send(Message::Text(ws_msg.to_string().into()))
                .await
                .is_err()
            {
                break; // Client disconnected
            }
        }

        // Check for client messages (ping / close) with a short timeout
        match timeout(Duration::from_millis(100), socket.recv()).await {
            Ok(Some(Ok(Message::Close(_)))) | Ok(None) => break,
            _ => {}
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
