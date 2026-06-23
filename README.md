# Mewn

A terminal-based network monitor with a cute cat mascot.
<img width="984" height="712" alt="mewn" src="https://github.com/user-attachments/assets/6e71c124-0b62-4b8e-b7bc-d7eb8dd38620" />

## Features

- 🐱 **Cat animation** on startup
- 📊 **Active connections** monitoring (TCP/UDP)
- 📈 **Bandwidth tracking** with area charts
- 📦 **Packet capture** with protocol parsing and DNS resolution
- 🔍 **Process mapping** with per-process upload/download rates, CPU, RAM, and real-time charts
- 🌍 **Cross-platform** permission setup (macOS, Linux, Windows)

### Coming soon

- 🖥️ Help screen with keybinding reference
- 💾 Export to JSON/CSV
- ⚙️ Configurable via `~/.config/mewn/config.toml`

## Installation

### From source

```bash
git clone https://github.com/adit-prawira/mewn.git
cd mewn
cargo build --release
```

## Usage

```bash
# Run the dashboard
mewn

# Setup packet capture permissions (requires sudo)
sudo mewn --setup

# Remove packet capture permissions (requires sudo)
sudo mewn --teardown
```

## Keyboard Shortcuts

### Global

| Key | Action |
|-----|--------|
| `q` / `Q` | Quit |
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |

### Connections tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |

### Bandwidth tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |

### Packets tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |
| `t` / `T` | Filter TCP only |
| `u` / `U` | Filter UDP only |
| `i` / `I` | Filter ICMP only |
| `a` / `A` | Remove filter |

### Processes tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |
| `f` / `F` | Toggle All ↔ Active (networked only) |
| `n` / `N` | Sort by process name |
| `p` / `P` | Sort by PID |
| `c` / `C` | Sort by connections |
| `r` / `R` | Sort by CPU |
| `m` / `M` | Sort by RAM |
| `s` / `S` | Toggle auto-sort on/off |
| `u` / `U` | Auto-sort by upload rate |
| `d` / `D` | Auto-sort by download rate |

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

## Development

### First-time setup

Before running `cargo run` on a new machine, install BPF permissions once:

```bash
cargo build && sudo ./target/debug/mewn --setup
```

> [!WARNING]  
> Do **not** use `sudo cargo run -- --setup`. This runs the entire Rust toolchain as root, leaving root-owned files in `target/`. Subsequent `cargo run` as your normal user will fail with permission errors because cargo can't overwrite those files.

After setup, development is just:

```bash
cargo run
cargo build --release
cargo test --all
cargo clippy --all-targets -- -D warnings
```

Re-run setup only if the LaunchDaemon gets removed, or if BPF permissions break after an OS update.


