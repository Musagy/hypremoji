use crate::{
    config::paste_config::PinMode, utils::{get_current_offset, get_hypremoji_client, write_both_configs},
};

pub enum PositionExpr {
    CursorOffset { x: &'static str, y: &'static str },
    Absolute { x: i32, y: i32 },
}

impl PositionExpr {
    pub fn to_conf(&self) -> String {
        match self {
            PositionExpr::CursorOffset { x, y } => format!("({}) ({})", x, y),
            PositionExpr::Absolute { x, y } => format!("{} {}", x, y),
        }
    }

    pub fn to_lua(&self) -> String {
        match self {
            PositionExpr::CursorOffset { x, y } => format!(r#""{}", "{}""#, x, y),
            PositionExpr::Absolute { x, y } => format!("{}, {}", x, y),
        }
    }
}

pub fn change_pin_type(pin_mode: PinMode) -> Result<(), Box<dyn std::error::Error>> {
    let hypremoji_client = get_hypremoji_client();
    let screens_size = get_current_offset();

    let (mut at_x, mut at_y) = hypremoji_client.at;
    let (size_x, size_y) = hypremoji_client.size;

    at_x -= screens_size.0;
    at_y -= screens_size.1;

    let expr = match pin_mode {
        PinMode::Point => {
            println!("Setting Point mode at: ({}, {})", at_x, at_y);
            PositionExpr::Absolute { x: at_x, y: at_y }
        }
        PinMode::CursorUp => {
            println!("Setting CursorUp mode");
            PositionExpr::CursorOffset {
                x: "cursor_x-(window_w*0.5)",
                y: "cursor_y-(window_h*0.05)",
            }
        }
        PinMode::CursorDown => {
            println!("Setting CursorDown mode");
            PositionExpr::CursorOffset {
                x: "cursor_x-(window_w*0.5)",
                y: "cursor_y-(window_h*0.95)",
            }
        }
    };

    let mode_name = match pin_mode {
        PinMode::Point => "Point",
        PinMode::CursorUp => "CursorUp",
        PinMode::CursorDown => "CursorDown",
    };

    println!(
        "Updating to {} mode with size: ({}, {})",
        mode_name, size_x, size_y
    );

    write_both_configs(&expr, size_x, size_y)
}