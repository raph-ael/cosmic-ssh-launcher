# cosmic-ssh-launcher

A panel applet for the [COSMIC](https://github.com/pop-os/cosmic-epoch) desktop environment that provides quick SSH access to hosts defined in `~/.ssh/config`.

![screenshot](https://raw.githubusercontent.com/raphaelhuefner/cosmic-ssh-launcher/main/screenshot.png)

## Features

- Reads SSH hosts from `~/.ssh/config`
- One-click connection opens `cosmic-term` with SSH session
- Refresh button to reload config
- Edit button to open config in `cosmic-edit`
- Instant popup (no GPU rendering overhead)

## Installation

### Dependencies

- [Rust](https://rustup.rs/)
- [just](https://github.com/casey/just)
- libwayland-dev, libxkbcommon-dev, pkg-config

### Build & Install

```sh
git clone https://github.com/raphaelhuefner/cosmic-ssh-launcher.git
cd cosmic-ssh-launcher
just build-release
sudo just install
```

### Add to Panel

Add `"io.github.cosmic-ssh-launcher"` to your panel config:

```
~/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings
```

Then restart the panel:

```sh
killall cosmic-panel
```

## License

MIT
