use anyhow::Result;

use crate::permissions::traits::OsSetup;

/** Windows packet capture setup — informational only.
 *
 *  No automated setup is performed. Packet capture on Windows requires:
 *    1. Npcap installed (https://npcap.com).
 *    2. Running mewn as Administrator.
 *
 *  The setup command prints guidance for both prerequisites and returns
 *  Ok — there is nothing to configure programmatically.
 *
 *  Teardown is a no-op on Windows.
 */
pub struct WindowsSetup;

impl OsSetup for WindowsSetup {
    fn run() -> Result<()> {
        println!("--> Windows requires Administrator for packet capture.");
        println!("--> No setup available - run with elevated privileges");
        println!();
        println!("--> Usage:");
        println!("-->   Right-click mewn.exe -> Run as Administrator");
        println!();
        println!("--> Or from an elevated command Prompt:");
        println!("-->   mewn.exe");
        Ok(())
    }

    fn teardown() -> Result<()> {
        println!("--> No teardown needed on Windows.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_windows_setup_then_run_returns_ok() {
        assert!(WindowsSetup::run().is_ok());
    }

    #[test]
    fn given_windows_setup_then_teardown_returns_ok() {
        assert!(WindowsSetup::teardown().is_ok());
    }
}
