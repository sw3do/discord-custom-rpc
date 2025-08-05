use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use discord_rich_presence::activity::{Activity, Assets, Button, Timestamps};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityData {
    details: Option<String>,
    state: Option<String>,
    #[serde(rename = "type")]
    activity_type: Option<i32>,
    timestamps: Option<ActivityTimestamps>,
    assets: Option<ActivityAssets>,
    buttons: Option<Vec<ActivityButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTimestamps {
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityButton {
    label: String,
    url: String,
}

type RPCConnection = Arc<Mutex<Option<DiscordIpcClient>>>;

static RPC_CONNECTION: std::sync::OnceLock<RPCConnection> = std::sync::OnceLock::new();

fn get_rpc_connection() -> &'static RPCConnection {
    RPC_CONNECTION.get_or_init(|| Arc::new(Mutex::new(None)))
}

#[tauri::command]
async fn connect_discord_rpc(client_id: String) -> Result<String, String> {
    if client_id.trim().is_empty() {
        return Err("Client ID cannot be empty".to_string());
    }
    
    let mut client = DiscordIpcClient::new(&client_id)
        .map_err(|e| format!("Failed to create Discord IPC client: {}", e))?;
    
    client.connect()
        .map_err(|e| format!("Failed to connect to Discord: {}. Make sure Discord is running.", e))?;
    
    let connection = get_rpc_connection();
    *connection.lock().unwrap() = Some(client);
    
    Ok("Successfully connected to Discord RPC".to_string())
}

#[tauri::command]
async fn set_activity(activity: ActivityData) -> Result<String, String> {
    let connection = get_rpc_connection();
    let mut conn_guard = connection.lock().unwrap();
    
    let client = conn_guard.as_mut()
        .ok_or("Not connected to Discord RPC. Please connect first.")?;
    
    let mut discord_activity = Activity::new();
    
    if let Some(ref details) = activity.details {
        discord_activity = discord_activity.details(details);
    }
    
    if let Some(ref state) = activity.state {
        discord_activity = discord_activity.state(state);
    }
    
    if let Some(timestamps) = activity.timestamps {
        let mut ts = Timestamps::new();
        if let Some(start) = timestamps.start {
            ts = ts.start(start as i64);
        }
        if let Some(end) = timestamps.end {
            ts = ts.end(end as i64);
        }
        discord_activity = discord_activity.timestamps(ts);
    }
    
    if let Some(ref assets) = activity.assets {
        let mut discord_assets = Assets::new();
        if let Some(ref large_image) = assets.large_image {
            discord_assets = discord_assets.large_image(large_image);
        }
        if let Some(ref large_text) = assets.large_text {
            discord_assets = discord_assets.large_text(large_text);
        }
        if let Some(ref small_image) = assets.small_image {
            discord_assets = discord_assets.small_image(small_image);
        }
        if let Some(ref small_text) = assets.small_text {
            discord_assets = discord_assets.small_text(small_text);
        }
        discord_activity = discord_activity.assets(discord_assets);
    }
    
    if let Some(ref buttons) = activity.buttons {
        let discord_buttons: Vec<Button> = buttons.iter()
            .map(|b| Button::new(&b.label, &b.url))
            .collect();
        discord_activity = discord_activity.buttons(discord_buttons);
    }
    
    client.set_activity(discord_activity)
        .map_err(|e| format!("Failed to set activity: {}", e))?;
    
    Ok("Activity set successfully".to_string())
}

#[tauri::command]
async fn clear_activity() -> Result<String, String> {
    let connection = get_rpc_connection();
    let mut conn_guard = connection.lock().unwrap();
    
    let client = conn_guard.as_mut()
        .ok_or("Not connected to Discord RPC. Please connect first.")?;
    
    client.clear_activity()
        .map_err(|e| format!("Failed to clear activity: {}", e))?;
    
    Ok("Activity cleared successfully".to_string())
}

#[tauri::command]
async fn disconnect_discord_rpc() -> Result<String, String> {
    let connection = get_rpc_connection();
    let mut conn_guard = connection.lock().unwrap();
    
    if let Some(mut client) = conn_guard.take() {
        client.close()
            .map_err(|e| format!("Failed to disconnect: {}", e))?;
        Ok("Disconnected from Discord RPC".to_string())
    } else {
        Ok("Already disconnected".to_string())
    }
}

#[tauri::command]
async fn get_connection_status() -> Result<bool, String> {
    let connection = get_rpc_connection();
    let conn_guard = connection.lock().unwrap();
    Ok(conn_guard.is_some())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            connect_discord_rpc,
            set_activity,
            clear_activity,
            disconnect_discord_rpc,
            get_connection_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
