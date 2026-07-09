use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::theme::Theme;

const KEY_WIDTH: usize = 8;

pub struct Shortcut;

impl Shortcut {
    pub fn render(key: &str, description: &str) -> Line<'static> {
        let key_text = Span::styled(format!("  {: <KEY_WIDTH$}  ", key), Style::default().fg(Theme::text_highlight()));
        let desc_text = Span::styled(description.to_string(), Style::default().fg(Theme::text_dim()));
        Line::from(vec![key_text, desc_text])
    }
}
