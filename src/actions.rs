use crate::{
    cli::Commands,
    utils
};

pub fn handle_command(command: &Commands) {
    match command {
        Commands::Reset => {
            println!("Resetting Hypremoji configuration...");

            if let Err(e) = utils::reset_config() {
                eprintln!("Error resetting configuration: {}", e);
                std::process::exit(1);
            } else {
                println!("Configuration reset successfully!");
                return;
            }
        }
        Commands::SetupHyprland => {
            println!("Wiring hypremoji into your Hyprland config...");

            if let Err(e) = utils::setup_hyprland() {
                eprintln!("Error setting up Hyprland: {}", e);
                std::process::exit(1);
            } else {
                println!("Hyprland setup completed successfully!");
                return;
            }
        }
    }
}