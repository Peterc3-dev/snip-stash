# snip-stash

Code snippet manager TUI -- browse, search, and preview snippets stored as JSON in `~/.snip-stash/`.

## Features

- Split-pane TUI: snippet list on the left, syntax-highlighted preview on the right
- Ships with 15 starter snippets (Rust, Python, Bash, SQL, and more)
- Fuzzy search across title, language, tags, and code body with `/`
- Full-screen view mode with scrollable code and line numbers
- Delete snippets with `Ctrl-D`
- Persistent JSON storage in `~/.snip-stash/snippets.json`
- Language-colored labels in the list view
- Phosphor-green terminal aesthetic

## Install

```
cargo build --release
cp target/release/snip-stash ~/.local/bin/
```

## Usage

```bash
snip-stash    # launch the TUI
```

Snippets are stored in `~/.snip-stash/snippets.json` -- edit the file directly to bulk-add snippets.

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `k` | Move cursor down / up |
| `Enter` | Open full view mode |
| `/` | Search snippets |
| `Esc` | Exit search or view mode |
| `Ctrl-D` | Delete selected snippet |
| `j` / `k` (in view) | Scroll code |
| `q` | Quit |

Built with Rust + ratatui.
