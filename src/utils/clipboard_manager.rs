use std::{process::Command, thread, time::Duration};

use arboard::Clipboard;
use chrono::Utc;

use crate::utils::get_last_client;

pub struct ClipboardManager {
    focused_window_id: String,
}
impl ClipboardManager {
    pub fn send_emoji_to_focused_window(&self, emoji: &str) {
        send_emoji(emoji, &self.focused_window_id);
    }
}

#[derive(Debug)]
struct OriginalClipboardContent {
    content: Option<String>, // Si es texto, aca es solo el texto, si es imagen, aca es el path al archivo temporal
    mime_type: String,
}

pub fn get_clipboard_manager() -> ClipboardManager {
    let address = get_last_client().address;

    ClipboardManager {
        focused_window_id: address,
    }
}

fn send_emoji(emoji: &str, window_id: &str) {
    let mut clipboard = Clipboard::new().expect("No se pudo acceder al clipboard");

    // 1. Intentar guardar el contenido original del clipboard
    let original_clipboard_content = save_original_clipboard_content();

    // 2. Set emoji en el clipboard
    clipboard
        .set_text(emoji.to_string())
        .expect("No se pudo poner el emoji en el clipboard");

    // 3. Insertar emoji en la ventana previamente enfocada
    let command_str = format!(
        "hyprctl dispatch sendshortcut CONTROL, V, address:{}",
        window_id
    );
    Command::new("sh")
        .arg("-c")
        .arg(&command_str)
        .output()
        .expect("Falló el hyprctl command");

    // 4. Esperar un poquito a que Hyprland pegue bien
    thread::sleep(Duration::from_millis(100));

    // 5. Restaurar el contenido original del clipboard
    if original_clipboard_content.content.is_some() {
        set_element_to_clipboard(original_clipboard_content);
    }
}

fn save_original_clipboard_content() -> OriginalClipboardContent {
    let types_output = Command::new("wl-paste")
        .arg("--list-types")
        .output()
        .expect("Fallo al listar tipos");

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
            eprintln!("Mime type no soportado: {}", mime_type);
            return OriginalClipboardContent {
                content: None,
                mime_type: mime_type.to_string(),
            };
        }
    };

    let full_path = format!("{}{}", path, extension);
    let command_str = format!("wl-paste --type {} > {}", mime_flag, full_path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command_str)
        .output()
        .expect(&format!("Fallo al ejecutar wl-paste para {}", mime_flag));

    if output.status.success() {
        content = Some(full_path);
    }

    OriginalClipboardContent {
        content: content,
        // mime_type: mime_type.to_string(),
        mime_type: mime_type.to_string(),
    }
}

fn set_element_to_clipboard(occ: OriginalClipboardContent) {
    let prefix = if occ.mime_type == "text/plain" {
        "--type text/plain <"
    } else {
        "<"
    };

    let command_str = format!("wl-copy {} {}", prefix, occ.content.unwrap_or_default());
    Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .expect("Falló al cargar a clipboard");
}
