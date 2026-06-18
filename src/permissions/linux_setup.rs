use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::permissions::traits::OsSetup;

pub struct LinuxSetup; 

impl OsSetup for LinuxSetup {
    fn run() -> Result<()> {
        if !Self::command_exists("setcap") {
            bail!("--> setcap not found. Install with: sudo apt install libcap2-bin");
        }

        let binary_path = std::env::current_exe()
            .context("Failed to detect binary path")?;
        let binary_path_str = binary_path.to_string_lossy();
        
        println!("--> Binary path: {}", binary_path_str);

        let current_capability = Self::get_current_capability(&binary_path_str)?;

        if current_capability.as_deref() == Some("cap_net_raw+ep") {
            println!("--> ✓ Capability already set correctly");
            return Ok(());
        }

        println!("--> Setting capability cap_net_raw+ep...");

        let status = Command::new("setcap")
            .args(["cap_net_raw+ep", &binary_path_str])
            .status()
            .context("Failed to run setcap")?;

        if !status.success() {
            bail!("--> setcap failed");
        }

        let new_capability = Self::get_current_capability(&binary_path_str)?;
        if new_capability.as_deref() != Some("cap_net_raw+ep") {
            bail!("--> Capability not set correctly after setcap");
        }

        println!("--> ✓ Capability set successfully");
        Ok(())
    }

    fn teardown() -> Result<()> {
        if !Self::command_exists("setcap") {
           bail!("--> setcap not found. Install with: sudo apt install libcap2-bin");
        } 

        let binary_path = std::env::current_exe()
           .context("Failed to detect binary path")?;

        let binary_path_str = binary_path.to_string_lossy();
        println!("--> Removing capability from {}...", binary_path_str);

        let status = Command::new("setcap")
            .args(["-r", &binary_path_str])
            .status()
            .context("Failed to run setcap -r")?;

        if !status.success() {
            bail!("--> setcap -r failed");
        }

        println!("--> ✓ Capability removed");
        Ok(())
    }
}

impl LinuxSetup {
    fn command_exists(cmd: &str) -> bool {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|line| line.status.success())
            .unwrap_or(false)
    }

    fn get_current_capability(binary_path: &str) -> Result<Option<String>> {
        let output = Command::new("getcap")
            .arg(binary_path)
            .output()
            .context("Failed to run getcap")?;
        
        if !output.status.success() {
            return Ok(None);    
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        if let Some(capability) = stdout.split("=").nth(1) {
            Ok(Some(capability.trim().to_string()))
        } else { 
            Ok(None)
        }
    }
}
