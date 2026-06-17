use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Table};

use crate::permissions::bpf::BpfAccess;
use crate::theme::{GREEN, PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER, YELLOW, YELLOW_DARKER};

use super::resource::Packet;


#[derive(Default)]
pub struct PacketUserInterface {
    selected_row: usize,
    scroll_offset: usize
}

impl PacketUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, packets: &[Packet]) {
        self.selected_row = self.selected_row.min(packets.len().saturating_sub(1));
        
        if packets.is_empty() && !BpfAccess::is_available() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(PRIMARY))
                .padding(Padding::new(2, 2, 1, 1));
            let lines = vec![
                Line::from(Span::styled("Permission Denied", Style::default().fg(YELLOW).bold())),
                Line::from(""),
                Line::from(BpfAccess::help_message()).style(Style::default().fg(TEXT_COLOR))
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

        let header_cells = ["", "", "Timestamp", "Protocol", "Source", "Destination", "Size", ""]
            .iter()
            .map(|header| {
                let style = Style::default().fg(TEXT_COLOR).bold();
                Cell::from(*header).style(style)
            });

        let default_text_style = Style::default().fg(TEXT_COLOR_DARKER);
        let table_header = Row::new(header_cells).height(1);
        let table_rows = packets.iter().enumerate()
            .skip(self.scroll_offset)
            .take(viewport)
            .map(|(index, packet)| {
                let is_selected = index == self.selected_row;
                let selected_indicator = if is_selected {"▶".to_string()} else {String::from("")};

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Gray)
                        .bg(Color::Rgb(132, 75, 92))
                } else {
                    Style::default()
                        .fg(Color::Gray)
                };

                Row::new([
                    Cell::from(""),
                    Cell::from(selected_indicator).style(default_text_style),
                    Cell::from(packet.timestamp.to_string()).style(default_text_style),
                    Cell::from(packet.protocol.to_string()).style(Style::default().fg(GREEN)),
                    Cell::from(packet.source.to_string()).style(Style::default().fg(YELLOW)),
                    Cell::from(packet.destination.to_string()).style(Style::default().fg(YELLOW_DARKER)),
                    Cell::from(packet.size.to_string()).style(default_text_style),
                    Cell::from("")
                ]).style(style)
            });
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(PRIMARY))
            .padding(Padding::new(2, 2, 1, 1));

        let table = Table::new(table_rows, [
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Length(1)
        ])
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
}
