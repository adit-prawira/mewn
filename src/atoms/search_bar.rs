use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use crate::theme::Theme;

#[derive(Default)]
pub struct SearchBarComponent {
    search_active: bool,
    search_query: String,
    tick_count: u64,
}

impl SearchBarComponent {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.tick_count = self.tick_count.wrapping_add(1);

        // cursor should blink
        let search_cursor = if self.search_active && self.tick_count % 15 < 7 { "█" } else { " " };
        let search_text = if self.search_active || !self.search_query.is_empty() {
            format!("SEARCH: {}{}", self.search_query, search_cursor)
        } else {
            "Press / to search ...".into()
        };

        let search_style = if self.search_active {
            Style::default().fg(Theme::text())
        } else {
            Style::default().fg(Theme::text_dim())
        };

        let search_span = Span::styled(search_text, search_style);
        let search_bar_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Theme::border()))
            .padding(Padding::new(1, 1, 0, 0));
        let search_bar = Paragraph::new(search_span).block(search_bar_block);

        frame.render_widget(search_bar, area);
    }

    pub fn add_search_char(&mut self, c: char) {
        self.search_query.push(c);
    }

    pub fn remove_search_char(&mut self) {
        self.search_query.pop();
    }

    pub fn is_active(&self) -> bool {
        self.search_active
    }

    pub fn inactive(&mut self) {
        self.search_active = false;
    }

    pub fn active(&mut self) {
        self.search_active = true;
    }

    pub fn get_search_query(&self) -> String {
        self.search_query.clone()
    }

    pub fn reset(&mut self) {
        self.search_query.clear();
    }
}
