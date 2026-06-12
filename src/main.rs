use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use mewn::cat::Cat;
use mewn::terminal::Terminal;

fn main() {   
    let mut terminal = Terminal::init();
    let mut cat = Cat::default();
    loop {
        cat.animate(&mut terminal);
        if event::poll(Duration::from_millis(50)).expect("poll failed") {
            let Ok(Event::Key(key)) = event::read() else {continue;};

            if key.code == KeyCode::Char('q') {
              break;
            }
        }
    }
}
