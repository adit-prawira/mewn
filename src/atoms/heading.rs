use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub struct Heading;

impl Heading {
    pub fn render(label: &str) -> Line<'static> {
        Line::from(Span::styled(format!("  {label}"), Style::default().fg(Color::White)))
    }
}
