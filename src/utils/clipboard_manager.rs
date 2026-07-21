use std::{cell::RefCell, fs::OpenOptions, process::Command, rc::Rc, thread, time::Duration};
use std::os::unix::fs::OpenOptionsExt;

use chrono::Utc;

use crate::{config::{app_config::AppConfig}, utils::get_last_client};

#[derive(Clone)]
pub struct ClipboardManager {
    focused_window_id: String,
    focused_window_class: String,
    chosen_emoji: Rc<RefCell<Option<String>>>,
    app_config: AppConfig,
}

impl ClipboardManager {
    pub fn send_emoji_to_focused_window(&self) {
        if let Some(emoji) = self.chosen_emoji.borrow().as_ref() {
            send_emoji(emoji, &self.focused_window_id, &self.focused_window_class, &self.app_config);
        }
    }
    pub fn set_chosen_emoji(&self, emoji: String) {
        *self.chosen_emoji.borrow_mut() = Some(emoji);
    }
}

#[derive(Debug)]
struct OriginalClipboardContent {
    content: Option<String>,
    mime_type: String,
}

pub fn get_clipboard_manager(config: &AppConfig) -> ClipboardManager {
    let last_client = get_last_client();
    let chosen_emoji: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    ClipboardManager {
        focused_window_id: last_client.address,
        focused_window_class: last_client.class,
        chosen_emoji,
        app_config: config.clone(),
    }
}

fn send_emoji(emoji: &str, window_id: &str, window_class: &str, config: &AppConfig) {
    // 1. Try to save the original clipboard content
    let original_clipboard_content = save_original_clipboard_content();
    
    // 2. Set the emoji to the clipboard
    let emoji_status = Command::new("wl-copy")
        .arg("--type")
        .arg("text/plain")
        .arg("--")
        .arg(emoji)
        .status();

    if let Err(e) = emoji_status {
        eprintln!("Failed to copy emoji to clipboard: {}", e);
    }
    println!("Copying emoji to window: {} class: {}", window_id, window_class);
    
    // 3. Insert the emoji into the previously focused window
    let command_str = if config.needs_shift_for_paste(window_class) {
        format!(
            "hyprctl dispatch 'hl.dsp.send_shortcut({{ mods = \"CTRL SHIFT\", key = \"V\", window = \"address:{}\"}})'",
            window_id
        )
    } else {
        format!(
            "hyprctl dispatch 'hl.dsp.send_shortcut({{ mods = \"CTRL\", key = \"V\", window = \"address:{}\"}})'",
            window_id
        )
    };

    Command::new("sh")
        .arg("-c")
        .arg(&command_str)
        .output()
        .expect("Failed hyprctl command");

    // 4. Wait briefly to ensure Hyprland pastes correctly
    thread::sleep(Duration::from_millis(100));

    // 5. Restore the original clipboard content
    if original_clipboard_content.content.is_some() {
        set_element_to_clipboard(original_clipboard_content);
    }
}

fn save_original_clipboard_content() -> OriginalClipboardContent {
    let types_output = Command::new("wl-paste")
        .arg("--list-types")
        .output()
        .expect("Failed to list clipboard types");

    let stdout = String::from_utf8_lossy(&types_output.stdout);

    
    if stdout.contains("Nothing is copied") || stdout.trim().is_empty() {
        return OriginalClipboardContent {
            content: None,
            mime_type: "empty".to_string(),
        };
    }

    let mime_type = if stdout.contains("image/png") {
        "image/png"
    } else {
        "text/plain"
    };

    let mut content: Option<String> = None;
    let timestamp = Utc::now().timestamp();
    let path = format!("/tmp/hypremoji_{}.", timestamp);

    let (extension, mime_flag) = match mime_type {
        "image/png" => ("png", "image/png"),
        "text/plain" => ("txt", "text/plain"),
        _ => {
            eprintln!("Unsupported mime type: {}", mime_type);
            return OriginalClipboardContent {
                content: None,
                mime_type: mime_type.to_string(),
            };
        }
    };

    let full_path = format!("{}{}", path, extension);

    if let Err(e) = OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o600)
        .open(&full_path)
    {
        eprintln!("Failed to create temp file: {}", e);
        return OriginalClipboardContent {
            content: None,
            mime_type: mime_type.to_string(),
        };
    }

    let command_str = format!("wl-paste --type {} > {}", mime_flag, full_path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command_str)
        .output()
        .expect(&format!("Failed to execute wl-paste for {}", mime_flag));

    if output.status.success() {
        content = Some(full_path);
    }

    OriginalClipboardContent {
        content: content,
        mime_type: mime_type.to_string(),
    }
}

fn set_element_to_clipboard(occ: OriginalClipboardContent) {
    let prefix = if occ.mime_type == "text/plain" {
        "--type text/plain <"
    } else {
        "<"
    };

    let path = occ.content.unwrap_or_default();
    let command_str = format!("wl-copy {} {}", prefix, path);
    Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .expect("Failed to load content to clipboard");

    // Cleanup the temporary file
    if !path.is_empty() {
        let _ = std::fs::remove_file(&path);
    }
}