use ratatui::Frame;
use ratatui::style::Style;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap};

use crate::atoms::center::Center;
use crate::atoms::heading::Heading;
use crate::molecules::shortcut::Shortcut;
use crate::theme::Theme;

pub struct HelpModal;

impl HelpModal {
    pub fn render(frame: &mut Frame) {
        let area = frame.area();
        let overlay_area = Center::render(65, 90, area);

        frame.render_widget(Clear, overlay_area);

        let help_text = Text::from(vec![
            Heading::render("Global"),
            Shortcut::render("q", "Quit"),
            Shortcut::render("Tab", "Next tab"),
            Shortcut::render("BackTab", "Previous tab"),
            Shortcut::render("x", "Export current tab"),
            Shortcut::render("e", "Toggle export format (JSON / CSV)"),
            Shortcut::render("?", "Toggle this help"),
            Line::from(""),
            Heading::render("Processes"),
            Shortcut::render("/", "Search"),
            Shortcut::render("Space", "Pause / Resume"),
            Shortcut::render("Up/Down", "Navigate rows"),
            Shortcut::render("f", "Toggle filter (All / Active)"),
            Shortcut::render("p", "Sort by PID"),
            Shortcut::render("n", "Sort by process name"),
            Shortcut::render("c", "Sort by connections"),
            Shortcut::render("r", "Sort by CPU"),
            Shortcut::render("m", "Sort by RAM"),
            Shortcut::render("s", "Toggle auto-sort mode"),
            Shortcut::render("u", "Auto-sort by upload rate"),
            Shortcut::render("d", "Auto-sort by download rate"),
            Shortcut::render("t", "Toggle CPU gauge (bar / number)"),
            Line::from(""),
            Heading::render("Bandwidth / Connections"),
            Shortcut::render("/", "Search"),
            Shortcut::render("Space", "Pause / Resume"),
            Shortcut::render("Up/Down", "Navigate rows"),
            Line::from(""),
            Heading::render("Packet"),
            Shortcut::render("Space", "Pause / Resume"),
            Shortcut::render("Up/Down", "Navigate rows"),
            Shortcut::render("t", "Filter TCP"),
            Shortcut::render("u", "Filter UDP"),
            Shortcut::render("i", "Filter ICMP"),
            Shortcut::render("a", "Show all"),
            Shortcut::render("d", "Toggle DNS query log view"),
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::text_highlight()))
            .title(" Help (?/Esc to close) ")
            .style(Style::default().fg(Theme::text()))
            .padding(Padding::new(2, 2, 1, 1));

        let paragraph = Paragraph::new(help_text).block(block).wrap(Wrap { trim: false });

        frame.render_widget(paragraph, overlay_area);
    }
}
