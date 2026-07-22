📘 También disponible en [Español 🇪🇸](./README.es.md)

# 😀 HyprEmoji

HyprEmoji is a lightweight and fast emoji picker for the **Hyprland** window manager, built with GTK4 and Rust.  
A sleek way to copy emojis into any window on your system!

![preview](./banner.png)
![preview](./screenshot.png)

## ✨ Features

- 🔍 **Instant search:** find emojis by typing their name (with debounce included).
- 📂 **Category navigation:** Smileys, Animals, Food, Objects, and more!
- 📋 **Clipboard copy with auto Ctrl+V** into the focused window.
- 🧠 **Recent history:** frequently used emojis are saved automatically.
- 🎨 **Modern and minimal UI**, customizable through CSS (including on launch with `hypremoji -s <path>`).
- 💾 **Remembers window size and position** across sessions.
- ⚡ **Lua config support** — works with both the classic `hyprland.conf` and the new Hyprland Lua config system (`hyprland.lua`).

## 📥 Installation

### 📦 From the AUR *(recommended)*

```bash
paru -S hypremoji
```
Or...
```bash
yay -S hypremoji
```

> ✅ After installing, run `hypremoji setup-hyprland` for automatic configuration, or if you'd rather do it yourself, copy these lines into your Hyprland config file (`hyprland.lua` or `hyprland.conf`):
>
> **`hyprland.lua`:**
> ```lua
> package.path = package.path .. ";" .. os.getenv("HOME") .. "/.config/hypremoji/?.lua"
> require("hypremoji")
> ```
>
> **`hyprland.conf`:**
> ```conf
> source = ~/.config/hypremoji/hypremoji.conf
> ```

🎉 Launch with `Super` + `.` and enjoy!

## ⚙️ Manual Installation

> ⚠️ **On Arch or an Arch-based distro? Just run `makepkg -si`** (see above) instead of the steps below — it's the real, complete installation: binary in `/usr/bin`, assets in `/usr/share`, tracked by pacman, uninstallable, upgradeable. The steps in this section only produce a local build that lives inside the cloned repo, mainly meant for **development/testing, or for distros without `makepkg`**. There isn't yet a one-command equivalent to `makepkg -si` for other distros — if you want a proper system-wide install on Fedora/Debian/Ubuntu/etc., you'll need to package it yourself (`.rpm`, `.deb`) or copy the binary and assets into place by hand.

### 📦 Requirements

- **Rust + Cargo** — install via [rustup](https://rustup.rs) rather than your distro's package; distro-provided toolchains are often too old for this project's dependencies.
- **GTK 4 development headers** — the package name differs by distro:
  - Arch: `gtk4` (dev headers and runtime are bundled together)
  - Fedora: `gtk4-devel`
  - Debian/Ubuntu: `libgtk-4-dev`
- **pkg-config** and basic build tools (`gcc`, `make`) — usually pulled in by `base-devel` (Arch), `@development-tools` (Fedora), or `build-essential` (Debian/Ubuntu).
- **Hyprland** (with `hyprctl`) — see the [official Hyprland installation guide](https://wiki.hyprland.org/Getting-Started/Installation/) for your distro.
- **wl-clipboard** (`wl-copy`, `wl-paste`)
- **Noto Color Emoji (default)** or similar font

### 🚀 Steps

1. 📥 Clone the repo:

```bash
git clone https://github.com/Musagy/HyprEmoji.git
cd HyprEmoji
```

2. 🛠️ Build it:

```bash
cargo build --release
```

> ⚠️ Run the binary from inside the cloned repo (e.g. `./target/release/hypremoji`, right where you ran `cargo build`). It looks upward from its own location for the project's `assets/`/`config/` folders, so moving just the compiled binary elsewhere without the rest of the repo won't find them.

3. ⚙️ Wire HyprEmoji into your Hyprland config:

```bash
./target/release/hypremoji setup-hyprland
```

This detects whether you're on `hyprland.lua` or the classic `hyprland.conf` and appends the right lines automatically (with a timestamped backup of the file beforehand). Prefer doing it by hand? See the snippet in the [Installation](#-installation) section above — just keep in mind the shipped `bind`/`hl.bind` line calls plain `hypremoji`, which only resolves if the binary is on your `$PATH` (see the note below).

4. 🎉 Launch it directly to confirm it works:

```bash
./target/release/hypremoji
```

To use the `Super + .` keybind day-to-day, Hyprland needs to find `hypremoji` on your `$PATH` — this local build doesn't add it there. Two options:
- Edit the `bind`/`hl.bind` line `setup-hyprland` added, pointing it at the full path to your binary (e.g. `~/HyprEmoji/target/release/hypremoji`) — simplest, but tied to this exact folder; if you move or delete it, the bind breaks.
- Run `cargo install --path . --locked`, which copies the binary to `~/.cargo/bin/hypremoji` (already on `$PATH` if you installed Rust via rustup). Note this copy is *outside* the repo, so it can no longer find `assets/`/`config/` on its own — this only really works if you've already got `/usr/share/hypremoji` populated some other way, so it's not a reliable path for a first install.

Given both workarounds have rough edges, this manual flow is best treated as a way to build and test the app, not as a polished day-to-day install outside of Arch.

### Quick dependency install:

```bash
# Arch
sudo pacman -S gtk4 wl-clipboard noto-fonts-emoji

# Fedora
sudo dnf install gtk4-devel wl-clipboard google-noto-emoji-fonts

# Debian / Ubuntu
sudo apt install libgtk-4-dev wl-clipboard fonts-noto-color-emoji
```

>⚠️ You also need a running Hyprland setup for this to work!

## 🖱️ CLI Commands

HyprEmoji includes a command-line interface for configuration:
```bash
# Show help
hypremoji --help

# Launch with a custom CSS file for this session
hypremoji -s ~/.config/hypremoji/dark.css

# Reset configuration to defaults (window follows cursor below)
hypremoji reset

# Wire HyprEmoji into hyprland.lua or hyprland.conf
# (safe to re-run — it's a no-op if already configured)
hypremoji setup-hyprland
```

> 💡 **Tip:** By default, the window follows your cursor and appears below it. You can pin it to a fixed position anytime using the 📌 button inside the app.

## 🎨 Customization

You can tweak the theme via:

```bash
~/.config/hypremoji/style.css
```

You can also keep multiple variants (for example, `dark.css`, `light.css`) and launch Hypremoji with any of them on demand using `hypremoji -s /path/to/theme.css`. If you want to bind a specific theme to a Hyprland shortcut, update the corresponding `bind = … hypremoji` line in your Hyprland config to append the `-s` flag with the desired file.

#### Example:

```css
:root {
  --primary-col: #4b60a5;
  --primary-col-glow: #4b60a5aa;
  --gray: #444;
  --bg-col: #0F0F0F;
  --input-text-col: #FFFFFF;
  --btn-list-col: #181818;
  --entry-unfocus: #c41313;
  --btn-list-col-hover: #272727;
  --btn-list-col-hover-glow: #27272777;
  --btn-nav-col: #3E3E3E;
  --btn-nav-col-hover: #0F0F0F;
  --emoji-font: "Noto Color Emoji";
}
```

> 💬 Want that classic Discord emoji look? Check out [Twemoji](https://github.com/twitter/twemoji), it's the same font they use.

#### Icon colors follow your theme automatically

Icons (like the 📌 pin) use GTK's symbolic icon rendering, so their color comes straight from your CSS — no need to edit any SVG file by hand. Just tweak `--primary-col` in your `style.css` (see the example above) and the icons update along with the rest of the theme.

## 🤝 Contributions

Ideas, bug reports, and pull requests are very welcome!  
Open an [issue](https://github.com/Musagy/HyprEmoji/issues) or collaborate directly.

## 📄 License

This project is licensed under **ISC**. See [`LICENSE`](./LICENSE) for more details.

## 💸 Support me 

<p align="center"> 
  <a href="https://www.buymeacoffee.com/musagy" target="_blank" >
    <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;">
  </a>
</p>

![tengo-hambre](https://i.imgur.com/dT2gV43.png)  

<p align="center"> I'm hungry 🥵 </p>
