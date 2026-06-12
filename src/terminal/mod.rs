use std::io::stdout;
use std::ops::Drop;

use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::{Frame, Terminal as RatatuiTerminal};
use ratatui::prelude::CrosstermBackend;

pub struct Terminal {
    inner: RatatuiTerminal<CrosstermBackend<std::io::Stdout>>
}

impl Terminal {
    pub fn init() -> Self {
        enable_raw_mode().expect("failed to enable raw mode");
        
        let mut stdout = stdout(); 
        
        execute!(stdout, EnterAlternateScreen).expect("failed to enter alternate screen");
        
        let backend = CrosstermBackend::new(stdout);
        
        Terminal {
            inner: RatatuiTerminal::new(backend)
                .expect("failed to create terminal")
        }
    }

    pub fn draw<F>(&mut self, f: F) where F: FnOnce(&mut Frame) {
        self.inner.draw(f).expect("failed to draw");
    }

    pub fn clear_screen(&mut self) -> std::io::Result<()> {
        execute!(self.inner.backend_mut(), Clear(ClearType::All))
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().expect("failed to disable raw mode");
        execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    }
}
