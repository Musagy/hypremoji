use std::io::{BufRead, BufReader};

use gtk::{
    glib,
    prelude::{ButtonExt, WidgetExt},
    Button,
};

use crate::{
    ui::{create_generic_btn, IconName},
    utils::{get_current_offset, get_hypremoji_client},
};

pub fn create_save_window_state_button() -> gtk::Button {
    let save_window_state_btn = create_generic_btn(IconName::AiFillPushpin, "pin-window-btn");
    save_window_state_btn.set_tooltip_text(Some("Save current location\nfor next time"));

    setup_save_locate_btn(save_window_state_btn.clone());
    save_window_state_btn
}

fn setup_save_locate_btn(btn: Button) {
    let btn_clone = btn.clone();
    btn.connect_clicked(move |_| {
        // Trigger click animation
        btn_clone.remove_css_class("shot-animation");

        // Force reflow to ensure GTK applies animation class again
        let btn_for_timeout = btn_clone.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(10), move || {
            btn_for_timeout.add_css_class("shot-animation");

            // Remove animation class after it ends
            let btn_for_removal = btn_for_timeout.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(1000), move || {
                btn_for_removal.remove_css_class("shot-animation");
            });
        });

        let hypremoji_client = get_hypremoji_client();
        let screens_size = get_current_offset();

        let (mut at_x, mut at_y) = hypremoji_client.at;
        let (size_x, size_y) = hypremoji_client.size;

        // Subtract screen offset
        at_x -= screens_size.0;
        at_y -= screens_size.1;

        println!(
            "Saving current location: ({}, {}) with size: ({}, {})",
            at_x, at_y, size_x, size_y
        );

        let rule_line_position =
            format!("windowrulev2 = move {} {}, title:^(HyprEmoji)$", at_x, at_y);
        let rule_line_size = format!(
            "windowrulev2 = size {} {}, title:^(HyprEmoji)$",
            size_x, size_y
        );
        let rule_line_float = "windowrulev2 = float, title:^(HyprEmoji)$".to_string();

        let rule_section_header = "# WindowRules for HyprEmojis";

        if let Err(e) = update_hyprland_config(
            rule_section_header,
            &rule_line_float,
            &rule_line_position,
            &rule_line_size,
        ) {
            println!("Error updating config: {}", e);
        } else {
            println!("Config updated successfully!");
        }
    });
}

fn update_hyprland_config(
    header: &str,
    float_rule: &str,
    position_rule: &str,
    size_rule: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = dirs::home_dir().unwrap().join(".config/hypremoji/hypremoji.conf");

    let mut lines = vec![];

    let mut in_target_section = false;
    let mut header_found = false;
    let mut header_line_index = None;
    let mut has_float_rule = false;
    let mut has_position_rule = false;
    let mut has_size_rule = false;
    let mut position_rule_index = None;
    let mut size_rule_index = None;

    if let Ok(file) = std::fs::File::open(&config_path) {
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            lines.push(line.clone());

            if line.trim() == header {
                header_found = true;
                in_target_section = true;
                header_line_index = Some(index);
                continue;
            }

            if in_target_section {
                if line.contains("windowrulev2 = float, title:^(HyprEmoji)$") {
                    has_float_rule = true;
                } else if line.contains("windowrulev2 = move") {
                    has_position_rule = true;
                    position_rule_index = Some(index);
                } else if line.contains("windowrulev2 = size") {
                    has_size_rule = true;
                    size_rule_index = Some(index);
                }

                if line.trim().is_empty()
                    || (!line.trim().starts_with("windowrule")
                        && !line.trim().starts_with("#")
                        && !line.trim().is_empty())
                {
                    in_target_section = false;
                }
            }
        }
    }

    if has_position_rule && position_rule_index.is_some() {
        lines[position_rule_index.unwrap()] = position_rule.to_string();
    }

    if has_size_rule && size_rule_index.is_some() {
        lines[size_rule_index.unwrap()] = size_rule.to_string();
    }

    if header_found && has_float_rule && has_position_rule && has_size_rule {
        println!("All rules already exist and have been updated");
        std::fs::write(config_path, lines.join("\n"))?;
        return Ok(());
    }

    if header_found {
        let insert_index = header_line_index.unwrap() + 1;
        let mut rules_to_add = vec![];

        if !has_float_rule {
            rules_to_add.push(float_rule.to_string());
        }
        if !has_position_rule {
            rules_to_add.push(position_rule.to_string());
        }
        if !has_size_rule {
            rules_to_add.push(size_rule.to_string());
        }

        for (i, rule) in rules_to_add.iter().enumerate() {
            lines.insert(insert_index + i + 1, rule.clone());
        }
    } else {
        let insert_position = find_windowrule_insert_position(&lines);

        let mut section = vec![
            header.to_string(),
            float_rule.to_string(),
            position_rule.to_string(),
            size_rule.to_string(),
        ];

        match insert_position {
            Some(pos) => {
                section.push("".to_string());
                for (i, line) in section.iter().enumerate() {
                    lines.insert(pos + i, line.clone());
                }
            }
            None => {
                if !lines.is_empty() && !lines.last().unwrap().is_empty() {
                    lines.push("".to_string());
                }
                lines.extend(section);
            }
        }
    }

    std::fs::write(config_path, lines.join("\n"))?;
    Ok(())
}

fn find_windowrule_insert_position(lines: &[String]) -> Option<usize> {
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("windowrule") && !trimmed.starts_with("#") {
            let mut current_index = index;
            while current_index > 0 && lines[current_index - 1].trim().starts_with("#") {
                current_index -= 1;
            }

            if current_index == 0 {
                return None;
            }

            return Some(current_index);
        }
    }
    None
}
