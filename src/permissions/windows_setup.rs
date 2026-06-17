use anyhow::Result;

use crate::permissions::traits::OsSetup;

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
