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
- 🌏 **Geo-IP lookup** with country codes on remote IPs
- 🔎 **Search / filter** across all tabs
- ⏯️ **Pause / resume** per tab to freeze live data

### Coming soon

- 🖥️ Help screen with keybinding reference

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
# Binary lands in ~/.cargo/bin/mewn
```

## Usage

```bash
# Run the dashboard
mewn

# Show version
mewn version
mewn --version

# Show help
mewn --help

# Setup packet capture permissions (requires sudo)
sudo mewn setup
sudo mewn --setup

# Remove packet capture permissions (requires sudo)
sudo mewn teardown
sudo mewn --teardown

# Generate config template at ~/.config/mewn/config.toml
mewn init

# Update GeoIP database (requires IP2Location license key)
IP2LOCATION_LICENSE_KEY=your-key mewn geoip-update

# Export data (2s capture, then write to file)
mewn export --domain connection --format json --output out.json
mewn export -d bandwidth  -f csv -o out.csv
mewn export -d packet     -f json -o out.json
mewn export -d process    -f csv -o out.csv
```

### Export domains

| Domain | Description |
|--------|-------------|
| `connection` | Active TCP/UDP connections (PID, process, local, remote, state, protocol) |
| `bandwidth` | Per-interface upload/download rates (name, address, MTU, rates) |
| `packet` | Captured packet headers (timestamp, protocol, source, destination, size) |
| `process` | Per-process stats (process, PID, connections, upload, download, CPU, RAM) |

### Export formats

| Flag | Format |
|------|--------|
| `-f json` / `--format json` | Pretty-printed JSON |
| `-f csv` / `--format csv` | Comma-separated values |

## Keyboard Shortcuts

### Global

| Key | Action |
|-----|--------|
| `q` / `Q` | Quit |
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |
| `Space` | Pause / resume current tab |
| `x` / `X` | Export current tab to `~/.mewn-exports/` |
| `e` / `E` | Toggle export format (JSON ↔ CSV) |

### Connections tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |
| `/` | Search / filter connections |

### Bandwidth tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |
| `/` | Search / filter interfaces |

### Packets tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |
| `t` / `T` | Filter TCP only |
| `u` / `U` | Filter UDP only |
| `i` / `I` | Filter ICMP only |
| `a` / `A` | Remove filter |
| `d` / `D` | Toggle DNS query log view |

### Processes tab

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll rows |
| `/` | Search / filter processes |
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
sudo mewn setup
```

Creates a LaunchDaemon at `/Library/LaunchDaemons/com.mewn.bpf.plist` that runs on boot:

```
chmod go+rw /dev/bpf*
```

To remove:

```bash
sudo mewn teardown
```

### Linux

```bash
sudo mewn setup
```

Sets `cap_net_raw+ep` on the binary via `setcap`. Requires `libcap2-bin`:

```bash
sudo apt install libcap2-bin
```

To remove:

```bash
sudo mewn teardown
```

### Windows

Install [Npcap](https://npcap.com), then run as Administrator. No CLI setup required.

## GeoIP Lookup

The Connections tab shows a country code (e.g. `US`, `DE`) for each remote IP.

### Quick setup

1. Create a free account at <https://lite.ip2location.com>
2. Get your license key from the dashboard
3. Run: `IP2LOCATION_LICENSE_KEY=your-key mewn geoip-update`
4. Restart `mewn`

### Manual setup (alternative)

1. Download `IP2LOCATION-LITE-DB1.CSV.ZIP` from <https://lite.ip2location.com/database/ip-country>
2. Unzip and place `IP2LOCATION-LITE-DB1.CSV` at `~/.config/mewn/IP2LOCATION-LITE-DB1.CSV`
3. Restart `mewn`

### Behavior when DB is missing

- All rows show `-` in the Country column
- Local/private IPs (e.g. `192.168.x.x`) also show `-`
- IPv6 addresses not supported in DB1
- Update frequency: monthly. Re-run `mewn geoip-update` to refresh.

### Attribution

This product includes IP2Location LITE data available from
<https://lite.ip2location.com>.

## Development

### First-time setup

Before running `cargo run` on a new machine, install BPF permissions once:

```bash
cargo build && sudo ./target/debug/mewn setup
```

> [!WARNING]  
> Do **not** use `sudo cargo run -- setup`. This runs the entire Rust toolchain as root, leaving root-owned files in `target/`. Subsequent `cargo run` as your normal user will fail with permission errors because cargo can't overwrite those files.

After setup, development is just:

```bash
cargo run
cargo build --release
cargo test --all
cargo clippy --all-targets -- -D warnings
```

Re-run setup only if the LaunchDaemon gets removed, or if BPF permissions break after an OS update.


