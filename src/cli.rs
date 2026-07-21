use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "hypremoji")]
#[command(about = "A modern emoji picker for Hyprland, written in Rust + GTK4", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Custom CSS file path (e.g., /path/to/dark.css or /path/to/light.css)
    #[arg(short = 's', long = "style")]
    pub style: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Reset configuration to defaults
    Reset,

    /// Wire hypremoji into your hyprland.lua or hyprland.conf
    SetupHyprland,
}


#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum MousePosition {
    /// Window appears above the mouse cursor
    Up,
    /// Window appears below the mouse cursor
    Down,
    /// Remove windowrule for init in mouse
    None,
}