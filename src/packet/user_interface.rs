use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use crate::packet::packets_table::PacketsTable;
use crate::permissions::bpf::BpfAccess;
use crate::theme::Theme;

use super::dns_query_logs_table::DnsQueryLogsTable;
use super::resource::{Packet, ProtocolFilter};

#[derive(Default, PartialEq)]
enum ViewMode {
    #[default]
    Packets,
    DnsQueryLogs,
}

#[derive(Default)]
pub struct PacketUserInterface {
    selected_row: usize,
    scroll_offset: usize,
    active_protocol_filter: Option<ProtocolFilter>,
    view_mode: ViewMode,
    paused: bool,
    paused_snapshot: Vec<Packet>,
}

impl PacketUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, packets: &[Packet]) {
        if self.paused && self.paused_snapshot.is_empty() {
            self.paused_snapshot = packets.to_vec();
        }

        if !self.paused {
            self.paused_snapshot.clear();
        }

        let effective_data: &[Packet] = if self.paused { &self.paused_snapshot } else { packets };

        let filtered_packets: Vec<&Packet> = effective_data.iter().filter(|packet| self.handle_filter(packet)).collect();

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

        if filtered_packets.is_empty() && self.view_mode == ViewMode::DnsQueryLogs {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Theme::border()))
                .title("Packets [DNS]")
                .padding(Padding::new(2, 2, 1, 1));
            let lines = vec![Line::from(Span::styled("No DNS queries captured", Style::default().fg(Theme::text_dim()).bold()))];

            let placeholder = Paragraph::new(lines).block(block);
            frame.render_widget(placeholder, area);
            return;
        }

        self.selected_row = self.selected_row.min(filtered_packets.len().saturating_sub(1));

        let viewport = (area.height as usize).saturating_sub(3).max(1);
        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        match self.view_mode {
            ViewMode::Packets => PacketsTable::render(filtered_packets, viewport, self.scroll_offset, self.selected_row, self.active_protocol_filter, frame, area),
            ViewMode::DnsQueryLogs => DnsQueryLogsTable::render(filtered_packets, viewport, self.scroll_offset, self.selected_row, frame, area),
        }
    }

    pub fn handle_keys(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Up => self.previous_row(),
            KeyCode::Down => self.next_row(),
            KeyCode::Char('t') | KeyCode::Char('T') => self.filter_by_tcp(),
            KeyCode::Char('u') | KeyCode::Char('U') => self.filter_by_udp(),
            KeyCode::Char('i') | KeyCode::Char('I') => self.filter_by_icmp(),
            KeyCode::Char('a') | KeyCode::Char('A') => self.remove_filter(),
            KeyCode::Char('d') | KeyCode::Char('D') => self.toggle_view_mode(),
            KeyCode::Char(' ') => self.paused = !self.paused,
            _ => {}
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    fn filter_by_tcp(&mut self) {
        self.active_protocol_filter = Some(ProtocolFilter::Tcp);
    }

    fn filter_by_udp(&mut self) {
        self.active_protocol_filter = Some(ProtocolFilter::Udp);
    }

    fn filter_by_icmp(&mut self) {
        self.active_protocol_filter = Some(ProtocolFilter::Icmp);
    }

    fn remove_filter(&mut self) {
        self.active_protocol_filter = None;
    }

    fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Packets => ViewMode::DnsQueryLogs,
            ViewMode::DnsQueryLogs => ViewMode::Packets,
        };
    }

    fn handle_filter(&self, packet: &Packet) -> bool {
        match self.view_mode {
            ViewMode::Packets => self.active_protocol_filter.is_none_or(|protocol_filter| protocol_filter.matches(&packet.protocol)),
            ViewMode::DnsQueryLogs => packet.dns_domain.is_some(),
        }
    }
}
