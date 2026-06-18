use std::process::Command;

use anyhow::Result;

/** Cross-platform entry point for packet capture permission management.
 *
 *  Dispatches to the platform-specific setup implementation at compile time
 *  via `#[cfg(target_os)]`, calling `MacosSetup`, `LinuxSetup`, or
 *  `WindowsSetup` depending on the host OS.
 *
 *  Commands:
 *
 *  mewn --setup    -> run_setup() -- configure BPF/capability permissions.
 *  Prints a warning if not running as root, executes the platform
 *  setup, verifies via `BpfAccess::is_available()`, and prints
 *  platform-specific troubleshooting guidance on failure.
 *
 *  mewn --teardown -> run_teardown() -- reverse the setup, removing
 *  launch daemons, file capabilities, or plist files.
 *
 *  Both commands exit the process immediately after completion -- the TUI
 *  dashboard is never launched.
 *
 *  Unsupported platforms receive a `bail!("Unsupported platform")` error.
 */
pub struct PermissionSetup; 

impl PermissionSetup {
    pub fn run_setup() -> Result<()> {
        println!("Setting up packet capture permissions...");
        println!();

        Self::warn_if_not_root();
        let result = Self::setup_platform();

        match result {
            Ok(()) => {
                println!("✓ Setup complete");
                Self::verify_permissions();
                Ok(())
            },
            Err(e) => {
                println!("✗ Setup failed: {}", e);
                Self::print_troubleshooting();
                Err(e)
            }
        }
    }

    pub fn run_teardown() -> Result<()> {
        println!("Removing packet capture permissions...");
        println!();

        let result = Self::teardown_platform();

        match result {
            Ok(()) => {
                println!("✓ Teardown complete");
                Ok(())
            }
            Err(e) => {
                println!("✗ Teardown failed: {}", e);
                Err(e)
            }
        }
    }

    fn warn_if_not_root() {
        if !cfg!(unix) {
            return;
        }

        let uid = Command::new("id")
            .arg("-u")
            .output()
            .map(|line| String::from_utf8_lossy(&line.stdout).trim().to_string())
            .unwrap_or_default();

        if uid != "0" {
            println!("⚠ Not running as root. Permission changes require sudo.");
            println!("  If this fails, re-run: sudo mewn --setup");
            println!();
        }
    }

    fn verify_permissions() {
        use super::bpf::BpfAccess;
        if BpfAccess::is_available() {
            println!("✓ Packet capture permission verified");
            println!("✓ You can now run: mewn");
        } else {
            println!("✗ Permission verification failed");
            println!("  You may need to reboot or run with: sudo mewn");
        }    
    }

    fn print_troubleshooting() {
        #[cfg(target_os = "macos")]
        {
            println!();
            println!("Troubleshooting:");
            println!("  1. Ensure you ran with: sudo mewn --setup");
            println!("  2. Try rebooting your system");
            println!("  3. Run with: sudo mewn");
        }

        #[cfg(target_os = "linux")]
        {
            println!();
            println!("Troubleshooting:");
            println!("  1. Install libcap: sudo apt install libcap2-bin");
            println!("  2. Ensure you ran with: sudo mewn --setup");
            println!("  3. Run with: sudo mewn");
        }

        #[cfg(target_os = "windows")]
        {
            println!();
            println!("Windows requires Administrator for packet capture.");
            println!("Right-click mewn.exe → Run as administrator");
        }
    }

    #[cfg(target_os = "macos")]
    fn setup_platform() -> Result<()> {
        use crate::permissions::traits::OsSetup;
        super::macos_setup::MacosSetup::run()
    }

    #[cfg(target_os = "linux")]
    fn setup_platform() -> Result<()> {
        use crate::permissions::traits::OsSetup;
        super::linux_setup::LinuxSetup::run()
    }

    #[cfg(target_os = "windows")]
    fn setup_platform() -> Result<()> {
        use crate::permissions::traits::OsSetup;
        super::windows_setup::WindowsSetup::run()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    fn setup_platform() -> Result<()> {
        anyhow::bail!("Unsupported platform")
    }

    #[cfg(target_os = "macos")]
    fn teardown_platform() -> Result<()> {
        use crate::permissions::traits::OsSetup;
        super::macos_setup::MacosSetup::teardown()
    }

    #[cfg(target_os = "linux")]
    fn teardown_platform() -> Result<()> {
        use crate::permissions::traits::OsSetup;
        super::linux_setup::LinuxSetup::teardown()
    }

    #[cfg(target_os = "windows")]
    fn teardown_platform() -> Result<()> {
        use crate::permissions::traits::OsSetup;
        super::windows_setup::WindowsSetup::teardown()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    fn teardown_platform() -> Result<()> {
        anyhow::bail!("Unsupported platform")
    }

}
