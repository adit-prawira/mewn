use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

use crate::theme::Theme;

use super::resource::{Packet, ProtocolFilter};

pub struct PacketsTable;

impl PacketsTable {
    pub fn render(
        packets: Vec<&Packet>,
        viewport: usize,
        scroll_offset: usize,
        selected_row: usize,
        active_protocol_filter: Option<ProtocolFilter>,
        frame: &mut Frame,
        area: Rect,
    ) {
        let header_cells = ["", "", "Timestamp", "Protocol", "Source", "Destination", "Size", ""].iter().map(|header| {
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

            let destination = if let Some(ref dns_domain) = packet.dns_domain {
                format!("{} ({})", packet.destination, dns_domain)
            } else {
                packet.destination.to_string()
            };

            let protocol_style = match packet.protocol.as_str() {
                "TCP" => Style::default().fg(Theme::tcp()),
                "UDP" => {
                    let has_dns = packet.dns_domain.is_some();
                    if has_dns {
                        Style::default().fg(Theme::udp_secondary())
                    } else {
                        Style::default().fg(Theme::udp())
                    }
                }
                "ICMPV4" => Style::default().fg(Theme::icmp()),
                "ICMPV6" => Style::default().fg(Theme::icmp()),
                _ => default_text_style,
            };

            Row::new([
                Cell::from(""),
                Cell::from(selected_indicator).style(default_text_style),
                Cell::from(packet.timestamp.to_string()).style(default_text_style),
                Cell::from(packet.protocol.to_string()).style(protocol_style),
                Cell::from(packet.source.to_string()).style(Style::default().fg(Theme::source_address())),
                Cell::from(destination).style(Style::default().fg(Theme::destination_address())),
                Cell::from(packet.size.to_string()).style(default_text_style),
                Cell::from(""),
            ])
            .style(style)
        });
        let content_block = Block::default()
            .title(match active_protocol_filter {
                Some(ref protocol_filter) => format!("Packets [{}]", protocol_filter),
                None => "Packets [ALL]".into(),
            })
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
                Constraint::Length(15),
                Constraint::Percentage(35),
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
