# Mewn

A terminal-based network monitor with a cute cat mascot.
<img width="984" height="712" alt="mewn" src="https://github.com/user-attachments/assets/6e71c124-0b62-4b8e-b7bc-d7eb8dd38620" />

## Features

- 🐱 **Cat animation** on startup
- 📊 **Active connections** monitoring (TCP/UDP)
- 📈 **Bandwidth tracking** with sparkline graphs
- 📦 **Packet capture** with protocol parsing
- 🔍 **Process mapping** (per-process network usage)
- 🌍 **Cross-platform** permission setup (macOS, Linux, Windows)

### Coming soon

- 💾 Export to JSON/CSV
- ⚙️ Configurable via `~/.config/mewn/config.toml`

## Installation

### From source

```bash
git clone https://github.com/adit-prawira/mewn.git
cd mewn
cargo build --release
```

### Install globally

```bash
cargo install --path .
```

## Usage

```bash
# Run the dashboard
mewn

# Setup packet capture permissions (requires sudo)
sudo mewn --setup

# Remove packet capture permissions (requires sudo)
sudo mewn --teardown

# Run with packet capture (requires sudo/permission setup)
sudo mewn
```

### What happens when you run `mewn`

1. If BPF permissions are missing, a warning is shown
2. Terminal enters alternate screen
3. Cat animation plays (2 seconds) — press any key to skip
4. Dashboard appears with tabs:
   - **Connections** — Live TCP/UDP connections
   - **Bandwidth** — Per-interface stats with sparkline graphs
   - **Packets** — Live packet capture (requires `--setup`)
5. Press `q` to quit

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` | Quit |
| `Tab` | Switch tabs |

## Permission Setup

Packet capture requires elevated permissions. Mewn provides a one-time setup per platform:

### macOS

```bash
sudo mewn --setup
```

Creates a LaunchDaemon at `/Library/LaunchDaemons/com.mewn.bpf.plist` that runs on boot:

```
chmod go+rw /dev/bpf*
```

To remove:

```bash
sudo mewn --teardown
```

### Linux

```bash
sudo mewn --setup
```

Sets `cap_net_raw+ep` on the binary via `setcap`. Requires `libcap2-bin`:

```bash
sudo apt install libcap2-bin
```

To remove:

```bash
sudo mewn --teardown
```

### Windows

Install [Npcap](https://npcap.com), then run as Administrator. No CLI setup required.

## Requirements

- Rust 1.70+ (for building from source)
- macOS: root/sudo for `--setup` and packet capture
- Linux: root/sudo for `--setup`, `libcap2-bin`
- Windows: Npcap, Administrator privileges

## Development

This project is developed in vertical slices. See `PLANNING.md` and `phases/` for details.

```bash
cargo run
cargo build --release
cargo test --all
```
