use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};
use super::resource::Connection;

pub struct ConnectionRenderer;

impl ConnectionRenderer {
    pub fn render(frame: &mut Frame, area: Rect, connections: &[Connection]) {
        let header_cells = ["PID", "Process",  "Protocol", "Local", "Remote", "State"]
            .iter()
            .map(|header| {
                let style = Style::default().fg(Color::Rgb(236, 175, 204)).bold();
                Cell::from(*header).style(style)
            });
        let table_header = Row::new(header_cells).height(1);
        let table_rows = connections.iter().map(|connection| {
            let style = Style::default().fg(Color::Gray);
            Row::new([
                connection.pid.to_string(),
                connection.process.to_string(),
                connection.protocol.to_string(),
                connection.local.to_string(),
                connection.remote.to_string(),
                connection.state.to_string(),
            ]).style(style)
        });

        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 2, 1, 1));
        let table = Table::new(table_rows, [
            Constraint::Length(6),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Length(12),
        ])
        .header(table_header)
        .block(content_block);

        frame.render_widget(table, area);
    } 
}
