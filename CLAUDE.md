# cosmic-ssh-launcher

COSMIC panel applet that lists SSH hosts from `~/.ssh/config` for quick connection via `cosmic-term`.

## Features
- Parses `~/.ssh/config` and lists all hosts (excluding wildcards)
- Click a host to open `cosmic-term` with an SSH connection
- Refresh button to reload config
- Edit button to open config in `cosmic-edit`

## Build & Install
```bash
cargo build --release
sudo just install
```

## Development
```bash
just run          # Run with RUST_BACKTRACE=full
just check        # Clippy check
```

## Project Structure
- `src/main.rs` — Entry point, i18n init
- `src/app.rs` — COSMIC Application trait, popup UI, SSH config parsing
- `src/i18n.rs` — Localization setup

## GitHub
- Repo: https://github.com/raph-ael/cosmic-ssh-launcher
- Listed in: https://github.com/cosmic-utils/cosmic-project-collection
