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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_default_search_bar_when_checked_then_inactive_and_empty() {
        let bar = SearchBarComponent::default();
        assert!(!bar.is_active());
        assert_eq!(bar.get_search_query(), "");
    }

    #[test]
    fn given_inactive_bar_when_activated_then_becomes_active() {
        let mut bar = SearchBarComponent::default();
        bar.active();
        assert!(bar.is_active());
    }

    #[test]
    fn given_active_bar_when_deactivated_then_becomes_inactive() {
        let mut bar = SearchBarComponent::default();
        bar.active();
        bar.inactive();
        assert!(!bar.is_active());
    }

    #[test]
    fn given_empty_query_when_char_added_then_query_appends() {
        let mut bar = SearchBarComponent::default();
        bar.add_search_char('a');
        bar.add_search_char('b');
        assert_eq!(bar.get_search_query(), "ab");
    }

    #[test]
    fn given_non_empty_query_when_backspace_then_removes_last_char() {
        let mut bar = SearchBarComponent::default();
        bar.add_search_char('x');
        bar.add_search_char('y');
        bar.remove_search_char();
        assert_eq!(bar.get_search_query(), "x");
    }

    #[test]
    fn given_empty_query_when_backspace_then_remains_empty() {
        let mut bar = SearchBarComponent::default();
        bar.remove_search_char();
        assert_eq!(bar.get_search_query(), "");
    }

    #[test]
    fn given_non_empty_query_when_reset_then_query_cleared() {
        let mut bar = SearchBarComponent::default();
        bar.add_search_char('z');
        bar.reset();
        assert_eq!(bar.get_search_query(), "");
    }

    #[test]
    fn given_reset_bar_when_active_unchanged_then_is_active_preserved() {
        let mut bar = SearchBarComponent::default();
        bar.active();
        bar.add_search_char('x');
        bar.reset();
        assert!(bar.is_active());
        assert_eq!(bar.get_search_query(), "");
    }
}
