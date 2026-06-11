use crossterm::event::{self, Event, KeyCode};
use mewn::terminal::Terminal;

fn main() {
    let _terminal = Terminal::init();
    loop {
        let Ok(Event::Key(key)) = event::read() else {continue;};
        if key.code == KeyCode::Char('q') {
            break;
        }
    }
}
