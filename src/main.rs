use crossterm::event::{self, Event, KeyCode};
use mewn::cat::Cat;
use mewn::terminal::Terminal;

fn main() {
    let mut terminal = Terminal::init();

    terminal.draw(|f| {
        let area = f.area();
        Cat::render(f, area);
    });

    loop {
        let Ok(Event::Key(key)) = event::read() else {continue;};
        if key.code == KeyCode::Char('q') {
            break;
        }
    }
}
