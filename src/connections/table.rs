use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

use crate::theme::{BLUE, GREEN, PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER, YELLOW, YELLOW_DARKER};

use super::resource::Connection;

#[derive(Default)]
pub struct TableComponent {
    selected_row: usize,
    scroll_offset: usize,
}

impl TableComponent {
    pub fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    pub fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    pub fn reset_selection(&mut self) {
        self.scroll_offset = 0;
        self.selected_row = 0;
    }

    pub fn render(&mut self, connections: Vec<&Connection>, title: String, frame: &mut Frame, area: Rect) {
        self.selected_row = self.selected_row.min(connections.len().saturating_sub(1));
        let viewport = (area.height as usize).saturating_sub(3).max(1);
        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "PID", "Process", "Protocol", "Local", "Remote", "State", ""].iter().map(|header| {
            let style = Style::default().fg(TEXT_COLOR).bold();
            Cell::from(*header).style(style)
        });

        let default_text_style = Style::default().fg(TEXT_COLOR_DARKER);
        let table_header = Row::new(header_cells).height(1);
        let table_rows = connections.iter().enumerate().skip(self.scroll_offset).take(viewport).map(|(index, connection)| {
            let is_selected = index == self.selected_row;
            let selected_indicator = if is_selected { "▶".to_string() } else { String::from("") };
            let style = if is_selected {
                Style::default().fg(Color::Gray).bg(Color::Rgb(132, 75, 92))
            } else {
                Style::default().fg(Color::Gray)
            };
            let protocol_style = match connection.protocol.as_str() {
                "TCP" => Style::default().fg(GREEN),
                "UDP" => {
                    let has_dns = connection.local.ends_with(":53") || connection.remote.ends_with(":53");

                    if has_dns { Style::default().fg(YELLOW) } else { Style::default().fg(BLUE) }
                }
                _ => default_text_style,
            };

            Row::new([
                Cell::from(""),
                Cell::from(selected_indicator).style(default_text_style),
                Cell::from(connection.pid.to_string()).style(default_text_style),
                Cell::from(connection.process.to_string()).style(Style::default().fg(GREEN)),
                Cell::from(connection.protocol.to_string()).style(protocol_style),
                Cell::from(connection.local.to_string()).style(Style::default().fg(YELLOW)),
                Cell::from(connection.remote.to_string()).style(Style::default().fg(YELLOW_DARKER)),
                Cell::from(connection.state.to_string()).style(default_text_style),
                Cell::from(""),
            ])
            .style(style)
        });

        let content_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(PRIMARY))
            .padding(Padding::new(2, 2, 1, 1));

        let table = Table::new(
            table_rows,
            [
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(6),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
                Constraint::Length(12),
                Constraint::Length(1),
            ],
        )
        .header(table_header)
        .block(content_block);

        frame.render_widget(table, area);
    }
}
