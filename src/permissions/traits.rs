use anyhow::Result;

pub trait OsSetup {
    fn run() -> Result<()>;
    fn teardown() -> Result<()>;
}
