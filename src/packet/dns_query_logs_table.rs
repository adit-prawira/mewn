use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

use crate::theme::Theme;

use super::resource::Packet;

pub struct DnsQueryLogsTable;

impl DnsQueryLogsTable {
    pub fn render(packets: Vec<&Packet>, viewport: usize, scroll_offset: usize, selected_row: usize, frame: &mut Frame, area: Rect) {
        let header_cells = ["", "", "Timestamp", "Source", "Queried Domain", "DNS Server", ""].iter().map(|header| {
            let style = Style::default().fg(Theme::text()).bold();
            Cell::from(*header).style(style)
        });
        let default_text_style = Style::default().fg(Theme::text_dim());
        let table_header = Row::new(header_cells).height(1);
        let table_rows = packets.iter().enumerate().skip(scroll_offset).take(viewport).map(|(index, packet)| {
            let is_selected = index == selected_row;
            let selected_indicator = if is_selected { "▶".to_string() } else { String::from("") };

            let style = if is_selected {
                Style::default().fg(Theme::indicator()).bg(Theme::selected())
            } else {
                Style::default().fg(Theme::indicator())
            };

            Row::new([
                Cell::from(""),
                Cell::from(selected_indicator).style(default_text_style),
                Cell::from(packet.timestamp.to_string()).style(default_text_style),
                Cell::from(packet.source.to_string()).style(Style::default().fg(Theme::source_address())),
                Cell::from(packet.dns_domain.as_deref().unwrap_or("-")).style(Style::default().fg(Theme::dns())),
                Cell::from(packet.destination.to_string()).style(Style::default().fg(Theme::destination_address())),
                Cell::from(""),
            ])
            .style(style)
        });
        let content_block = Block::default()
            .title("Packets [DNS]")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Theme::border()))
            .padding(Padding::new(2, 2, 1, 1));

        let table = Table::new(
            table_rows,
            [
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(15),
                Constraint::Percentage(20),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
                Constraint::Length(1),
            ],
        )
        .header(table_header)
        .block(content_block);

        frame.render_widget(table, area);
    }
}
