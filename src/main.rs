use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use mewn::cat::Cat;
use mewn::terminal::Terminal;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

fn main() {   
    let mut terminal = Terminal::init();
    let mut cat = Cat::default();
    
    loop {
        cat.animate(&mut terminal);
        if event::poll(Duration::from_millis(50)).expect("poll failed") 
            && let Ok(Event::Key(_)) = event::read(){
                break; 
        }

        if cat.is_complete() {
            break;
        }
    }

    terminal.clear_screen().expect("failed to clear screen");

    loop {
        terminal.draw(|f| {
            let style = Style::default().fg(Color::White);
            let span = Span::styled("Dashboard coming in phase 2", style);
            let line = Line::from(span);

            let text = vec![line];
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);
            let area = f.area();
            let y = area.y + area.height / 2;
            let rect = Rect::new(area.x, y, area.width, 1);
            f.render_widget(paragraph, rect);
        });

        if event::poll(Duration::from_millis(50)).expect("poll failed")
            && let Ok(Event::Key(key)) = event::read()
            && key.code == KeyCode::Char('q')
        {
            return;
        }
    }
}
