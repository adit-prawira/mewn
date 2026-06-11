use std::io::stdout;
use std::ops::Drop;

use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};

pub struct Terminal;

impl Terminal {
    pub fn init() -> Self {
        enable_raw_mode().expect("failed to enable raw mode");
        execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");
        Terminal
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().expect("failed to disable raw mode");
        execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    }
}
