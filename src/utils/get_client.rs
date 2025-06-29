use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Client {
    pub address: String,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: u32,
    #[serde(rename = "initialTitle")]
    pub initial_title: String,
    pub at: (i32, i32),
    pub size: (i32, i32),
}

#[derive(Debug, Deserialize)]
pub struct MonitorInfo {
    pub x: i32,
    pub y: i32,
    focused: bool,
}

fn get_clients() -> Vec<Client> {
    let output = std::process::Command::new("hyprctl")
        .arg("clients")
        .arg("-j")
        .output()
        .expect("Failed to execute hyprctl command");

    let json_raw = String::from_utf8_lossy(&output.stdout);

    serde_json::from_str(&json_raw).expect("Failed to parse JSON from hyprctl output")
}

pub fn get_last_client() -> Client {
    let clients = get_clients();
    clients
        .into_iter()
        .find(|client| client.focus_history_id == 0)
        .unwrap_or_default()
}
pub fn get_hypremoji_client() -> Client {
    let clients = get_clients();
    clients
        .into_iter()
        .find(|client| client.initial_title.contains("HyprEmoji"))
        .unwrap_or_default()
}

pub fn get_current_offset() -> (i32, i32) {
    let output = std::process::Command::new("hyprctl")
        .arg("monitors")
        .arg("-j")
        .output()
        .expect("Failed to execute hyprctl command");

    if !output.status.success() {
        eprintln!("hyprctl returned an error");
        return (0, 0);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let monitors: Vec<MonitorInfo> = match serde_json::from_str(&stdout) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);

            return (0, 0);
        }
    };

    let focused_monitor = monitors
        .into_iter()
        .find(|monitor| monitor.focused == true)
        .unwrap_or_else(|| MonitorInfo {
            x: 0,
            y: 0,
            focused: false,
        });

    println!(
        "Focused monitor offset: ({}, {})",
        focused_monitor.x, focused_monitor.y
    );

    (focused_monitor.x, focused_monitor.y)
}
