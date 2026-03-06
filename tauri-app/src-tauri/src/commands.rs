use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
pub struct DaemonStatus {
    pub connected: bool,
    pub version: String,
}

#[tauri::command]
pub async fn toggle_window(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    if window.is_visible().unwrap_or(false) {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_daemon_status() -> Result<DaemonStatus, String> {
    // Connect to the Python daemon WebSocket and send a ping
    let status = match try_ping_daemon().await {
        Ok(version) => DaemonStatus {
            connected: true,
            version,
        },
        Err(_) => DaemonStatus {
            connected: false,
            version: String::new(),
        },
    };
    Ok(status)
}

#[tauri::command]
pub async fn send_to_daemon(method: String, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });

    send_rpc(request).await
}

#[tauri::command]
pub async fn confirm_action(plan_id: String, confirmed: bool) -> Result<(), String> {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "confirm",
        "params": {
            "plan_id": plan_id,
            "confirmed": confirmed
        },
        "id": 1
    });

    send_rpc(request).await.map(|_| ())
}

async fn try_ping_daemon() -> Result<String, String> {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ping",
        "params": {},
        "id": 1
    });

    let response = send_rpc(request).await?;
    let version = response
        .get("result")
        .and_then(|r| r.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(version)
}

async fn send_rpc(request: serde_json::Value) -> Result<serde_json::Value, String> {
    use tokio_tungstenite::connect_async;
    use futures_util::{SinkExt, StreamExt};

    let url = "ws://127.0.0.1:8785";
    let (mut ws, _) = connect_async(url)
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    let msg = serde_json::to_string(&request).map_err(|e| e.to_string())?;
    ws.send(tokio_tungstenite::tungstenite::Message::Text(msg))
        .await
        .map_err(|e| format!("Failed to send: {}", e))?;

    if let Some(Ok(response)) = ws.next().await {
        let text = response.to_text().map_err(|e| e.to_string())?;
        let parsed: serde_json::Value =
            serde_json::from_str(text).map_err(|e| e.to_string())?;
        Ok(parsed)
    } else {
        Err("No response from daemon".to_string())
    }
}
