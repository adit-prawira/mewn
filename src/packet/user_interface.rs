use std::fmt::Display;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Table};

use crate::permissions::bpf::BpfAccess;
use crate::theme::Theme;

use super::resource::Packet;

#[derive(Clone, PartialEq, Copy)]
enum ProtocolFilter {
    Tcp,
    Udp,
    Icmp,
}

impl Display for ProtocolFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolFilter::Tcp => write!(f, "TCP"),
            ProtocolFilter::Udp => write!(f, "UDP"),
            ProtocolFilter::Icmp => write!(f, "ICMP"),
        }
    }
}

impl ProtocolFilter {
    pub fn matches(&self, protocol: &str) -> bool {
        protocol.to_lowercase().starts_with(&self.to_string().to_lowercase())
    }
}

#[derive(Default)]
pub struct PacketUserInterface {
    selected_row: usize,
    scroll_offset: usize,
    active_protocol_filter: Option<ProtocolFilter>,
}

impl PacketUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, packets: &[Packet]) {
        let filtered_packets: Vec<&Packet> = packets
            .iter()
            .filter(|packet| self.active_protocol_filter.is_none_or(|protocol_filter| protocol_filter.matches(&packet.protocol)))
            .collect();

        self.selected_row = self.selected_row.min(filtered_packets.len().saturating_sub(1));

        if filtered_packets.is_empty() && !BpfAccess::is_available() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Theme::border()))
                .padding(Padding::new(2, 2, 1, 1));
            let lines = vec![
                Line::from(Span::styled("Permission Denied", Style::default().fg(Theme::warning()).bold())),
                Line::from(""),
                Line::from(BpfAccess::help_message()).style(Style::default().fg(Theme::text_dim())),
            ];

            let warning = Paragraph::new(lines).block(block);
            frame.render_widget(warning, area);
            return;
        }

        let viewport = (area.height as usize).saturating_sub(3).max(1);
        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "Timestamp", "Protocol", "Source", "Destination", "Size", ""].iter().map(|header| {
            let style = Style::default().fg(Theme::text()).bold();
            Cell::from(*header).style(style)
        });

        let default_text_style = Style::default().fg(Theme::text_dim());
        let table_header = Row::new(header_cells).height(1);
        let table_rows = filtered_packets.iter().enumerate().skip(self.scroll_offset).take(viewport).map(|(index, packet)| {
            let is_selected = index == self.selected_row;
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
            .title(match self.active_protocol_filter {
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

    pub fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    pub fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    pub fn filter_by_tcp(&mut self) {
        self.active_protocol_filter = Some(ProtocolFilter::Tcp);
    }

    pub fn filter_by_udp(&mut self) {
        self.active_protocol_filter = Some(ProtocolFilter::Udp);
    }

    pub fn filter_by_icmp(&mut self) {
        self.active_protocol_filter = Some(ProtocolFilter::Icmp);
    }

    pub fn remove_filter(&mut self) {
        self.active_protocol_filter = None;
    }
}
