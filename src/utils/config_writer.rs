use crate::utils::{get_config_dir, pin_utils::PositionExpr};

const CONF_HEADER: &str = "# HyprEmoji Configuration

# Keybind to open hypremoji
bind = SUPER, period, exec, hypremoji

# Window rules for HyprEmoji
windowrule = float true, match:title ^(HyprEmoji)$ 
";

const LUA_HEADER: &str = "-- HyprEmoji Configuration

-- Keybind to open hypremoji
hl.bind(\"SUPER + period\", hl.dsp.exec_cmd(\"hypremoji\"))

-- Window rules for HyprEmoji
";

pub fn write_both_configs(
    expr: &PositionExpr,
    size_x: i32,
    size_y: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;

    let conf_path = config_dir.join("hypremoji.conf");
    let lua_path = config_dir.join("hypremoji.lua");

    let conf_exists = conf_path.exists();
    let lua_exists = lua_path.exists();

    if !conf_exists && !lua_exists {
        return Err("No config files found (conf nor lua)".into());
    }

    write_lua(expr, size_x, size_y)?;

    if conf_exists {
        write_conf(expr, size_x, size_y)?;
    } else {
        println!("Skipping hypremoji.conf (not found)");
    }

    Ok(())
}

fn write_conf(
    expr: &PositionExpr,
    size_x: i32,
    size_y: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_dir()?.join("hypremoji.conf");

    let position = expr.to_conf();

    let content = format!(
        "{}windowrule = move {}, match:title ^(HyprEmoji)$\nwindowrule = size {} {}, match:title ^(HyprEmoji)$\n",
        CONF_HEADER, position, size_x, size_y
    );

    std::fs::write(&config_path, content)?;
    println!("Config written to: {}", config_path.display());

    Ok(())
}

fn write_lua(
    expr: &PositionExpr,
    size_x: i32,
    size_y: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_dir()?.join("hypremoji.lua");

    let move_block = expr.to_lua();

    let content = format!(
        "{0}hl.window_rule({{
    match = {{ title = \"^(HyprEmoji)$\" }},
    float = true,
    move  = {{{1}}},
    size  = {{{2}, {3}}},
}})
",
        LUA_HEADER, move_block, size_x, size_y
    );

    std::fs::write(&config_path, content)?;
    println!("Config written to: {}", config_path.display());

    Ok(())
}