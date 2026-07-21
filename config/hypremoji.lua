-- HyprEmoji Configuration

-- Keybind to open hypremoji
hl.bind("SUPER + period", hl.dsp.exec_cmd("hypremoji"))

-- Window rules for HyprEmoji
hl.window_rule({
    match = { title = "^(HyprEmoji)$" },
    float = true,
    move  = {"cursor_x-(window_w*0.5)", "cursor_y-(window_h*0.05)"},
})