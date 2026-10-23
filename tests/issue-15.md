# Issue #15: process remains after selection

Base: Musagy/hypremoji master, 7ed5a3fbc5173c57d81c1299c9a8adc6f79b7592.

## Lifecycle findings

- `src/main.rs`: GTK application runs, then `send_emoji_to_focused_window()`
  runs synchronously after the event loop exits.
- `src/ui/emoji_grid.rs`: child activation records the emoji and calls
  `window.close()`. Mouse activation and keyboard activation share this handler.
- `src/services/key_events_service.rs`: Escape calls `window.close()` without
  recording an emoji. Window-manager close uses GTK's default close handling.
- No custom close-request handler, application hold or hide-on-close was found.
- Selection alone enters clipboard restoration. The original
  `set_element_to_clipboard()` called `sh -c 'wl-copy ...'` with `.output()`.
  `wl-copy` forks a background clipboard owner; inherited stdout/stderr pipes
  can keep `.output()` waiting for EOF after the window has closed.
- Cancellation skips clipboard work because `chosen_emoji` remains `None`.
  This explains why Escape can exit while selection leaves a process alive.

## Scoped correction

Restore via `wl-copy` directly, with the saved file as stdin and stdout/stderr
set to null. `.status()` waits for the initial copier, not the clipboard owner's
pipe lifetime. Keep the clipboard owner alive so restored data remains usable.
Do not add `process::exit()` or `app.quit()`: neither addresses this post-GTK wait.

## Automated regression

Run `python3 tests/check_clipboard_exit.py` (requires Rust and Python).
It compiles the production restore function in isolation and provides a fake
`wl-copy` that forks with inherited stdout/stderr. Both text and binary PNG
fixtures must restore exactly, remove the temporary file, and return within two
seconds while the background clipboard owner remains alive. No user clipboard
or compositor is accessed. The original function times out on both fixtures;
the corrected function passes both.

Also run `cargo check --offline` and `cargo test --offline` when dependencies
are cached, or omit `--offline` otherwise.

## Interactive acceptance cases (Hyprland)

Use a build of this checkout and confirm no old hypremoji instance is running.
Track the launched PID (`hypremoji & picker_pid=$!`); do not count `wl-copy` as
an application leak. Exercise each route from a fresh launch.

| Route | Action | Expected result |
| --- | --- | --- |
| Selection | Seed clipboard with text; open picker; click an emoji | Window closes, emoji is pasted once, previous clipboard text is restored, launched hypremoji PID exits |
| Selection via keyboard | Repeat using focus + Return on an emoji | Same result as mouse selection |
| Window close | Close picker using the window manager's normal close action | Window closes and PID exits; no paste or clipboard change |
| Escape | Press Esc from the search field and, separately, from the emoji grid | Window closes and PID exits; no paste or clipboard change |

Repeat selection with an empty clipboard and a PNG clipboard. With PNG, paste
into an image-capable application afterward to confirm the restore is intact.
Repeat each exit route several times. With the configured
`pkill -x hypremoji || hypremoji` bind, one subsequent press must reopen the picker
after every completed exit; it must not consume a press killing a stale picker.

These interactive acceptance cases still require execution in a Hyprland session;
the isolated regression proves the subprocess blocking mechanism, not compositor
paste/focus behavior.
