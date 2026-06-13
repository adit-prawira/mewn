use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};
use super::resource::Connection;

#[derive(Default)]
pub struct ConnectionUserInterface {
    selected_row: usize,
    scroll_offset: usize
}

const DEFAULT_TEXT_COLOR: Color = Color::Rgb(156, 164, 201);

impl ConnectionUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, connections: &[Connection]) {
        self.selected_row = self.selected_row.min(connections.len().saturating_sub(1));
        let viewport = (area.height as usize).saturating_sub(3).max(1);
        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "PID", "Process",  "Protocol", "Local", "Remote", "State", ""]
            .iter()
            .map(|header| {
                let style = Style::default().fg(Color::Rgb(186, 196, 238)).bold();
                Cell::from(*header).style(style)
            });

        let default_text_style = Style::default().fg(DEFAULT_TEXT_COLOR);
        let table_header = Row::new(header_cells).height(1);
        let table_rows = connections.iter().enumerate()
            .skip(self.scroll_offset)
            .take(viewport)
            .map(|(index, connection)| {
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
                    Cell::from(connection.pid.to_string()).style(default_text_style),
                    Cell::from(connection.process.to_string()).style(Style::default().fg(Color::Rgb(124, 170, 131))),
                    Cell::from(connection.protocol.to_string()).style(default_text_style),
                    Cell::from(connection.local.to_string()).style(Style::default().fg(Color::Rgb(240, 217, 168))),
                    Cell::from(connection.remote.to_string()).style(Style::default().fg(Color::Rgb(173, 132, 105))),
                    Cell::from(connection.state.to_string()).style(default_text_style),
                    Cell::from("")
                ]).style(style)
            });

        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Rgb(137, 180, 250)))
            .padding(Padding::new(2, 2, 1, 1));

        let table = Table::new(table_rows, [
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(6),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Length(12),
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
