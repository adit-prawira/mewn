use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use super::traits::OsSetup;

const PLIST_PATH: &str = "/Library/LaunchDaemons/com.mewn.bpf.plist";
const PLIST_LABEL: &str = "com.mewn.bpf";

const PLIST_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.mewn.bpf</string>
    <key>ProgramArguments</key>
    <array>
        <string>/bin/sh</string>
        <string>-c</string>
        <string>chmod go+rw /dev/bpf*</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#;

/** Sets up BPF device permissions on macOS via a persistent LaunchDaemon
 *  and immediate `chmod`.
 *
 *  Creates /Library/LaunchDaemons/com.mewn.bpf.plist — a launchd job that
 *  runs `chmod go+rw /dev/bpf*` at every boot, granting read/write access
 *  to BPF packet capture devices for non-root users.
 *
 *  To avoid requiring a reboot, `apply_bpf_permissions()` also runs the
 *  equivalent `sh -c "chmod go+rw /dev/bpf*"` immediately after loading
 *  the daemon.
 *
 *  Teardown unloads the daemon via `launchctl unload` and removes the
 *  plist file.
 *
 *  Setup is idempotent: if the plist already exists and the daemon is
 *  loaded, no changes are made. If the plist exists but isn't loaded
 *  (e.g. after a manual unload), it is re-loaded and permissions are
 *  re-applied.
 *
 *  Must be run with `sudo mewn --setup`. Without root, writing to
 *  /Library/LaunchDaemons/ will fail with a permission error.
 */
pub struct MacosSetup;

impl OsSetup for MacosSetup {
    fn run() -> Result<()> {
        let plist_path = Path::new(PLIST_PATH);

        if plist_path.exists() {
            println!("--> Plist already exists, checking if loaded...");
            if Self::is_loaded() {
                println!("--> ✓ Already configured and loaded");
                return Ok(());
            } else {
                println!("--> Not loaded, loading now...");
                Self::load_plist()?;
                Self::apply_bpf_permissions()?;
                return Ok(());
            }
        }

        println!("--> Creating {}...", PLIST_PATH);
        fs::write(PLIST_PATH, PLIST_CONTENT).with_context(|| format!("Failed to write {}", PLIST_PATH))?;

        println!("--> Loading launch daemon...");
        Self::load_plist()?;
        Self::apply_bpf_permissions()?;
        Ok(())
    }

    fn teardown() -> Result<()> {
        println!("--> Unloading launch daemon...");
        let unload_status = Command::new("launchctl").args(["unload", PLIST_PATH]).status().context("Failed to run launchctl unload")?;

        if !unload_status.success() {
            println!("--> Warning: unload failed (may not be loaded)");
        }

        if Path::new(PLIST_PATH).exists() {
            println!("--> Removing {}...", PLIST_PATH);
            fs::remove_file(PLIST_PATH).with_context(|| format!("Failed to remove {}", PLIST_PATH))?;
            println!("--> ✓ PList removed");
        }
        Ok(())
    }
}

impl MacosSetup {
    fn apply_bpf_permissions() -> Result<()> {
        println!("--> Applying BPF permissions (chmod go+rw /dev/bpf*)...");
        let status = Command::new("sh")
            .args(["-c", "chmod go+rw /dev/bpf*"])
            .status()
            .context("Failed to run: sh -c 'chmod go+rw /dev/bpf*'")?;

        if !status.success() {
            bail!("--> sh -c 'chmod go+rw /dev/bpf* ' failed");
        }

        Ok(())
    }

    fn is_loaded() -> bool {
        // execute -> launchctl list
        Command::new("launchctl")
            .args(["list"])
            .output()
            .map(|line| String::from_utf8_lossy(&line.stdout).contains(PLIST_LABEL))
            .unwrap_or(false)
    }

    fn load_plist() -> Result<()> {
        // execute -> launchctl load -w PLIST_PATH
        let status = Command::new("launchctl")
            .args(["load", "-w", PLIST_PATH])
            .status()
            .context("Failed to run launchctl load")?;

        if !status.success() {
            bail!("--> launchctl load failed");
        }

        Ok(())
    }
}
