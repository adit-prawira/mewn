use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use mewn::cat::Cat;
use mewn::terminal::Terminal;

fn main() {   
    let mut terminal = Terminal::init();
    let mut next_blink = Instant::now();
    let mut is_startup = true;

    let blink_interval = Duration::from_secs(5);

    loop {
        if Instant::now() >= next_blink {
            let duration = if is_startup {
                Duration::from_secs(1)
            } else {
                Duration::from_millis(600)
            };

            let interval = if is_startup {
                Duration::from_millis(200)
            } else {
                Duration::from_millis(150)
            };

            Cat::animate(&mut terminal, duration, interval);
            is_startup = false;
            next_blink = Instant::now() + blink_interval; 
        }

        if event::poll(Duration::from_millis(50)).expect("poll failed") {
            let Ok(Event::Key(key)) = event::read() else {continue;};

            if key.code == KeyCode::Char('q') {
              break;
            }
        }
    }
}
