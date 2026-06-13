use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use super::lsof_stream::LsofStream;

pub struct ConnectionRenderer;

impl ConnectionRenderer {
    pub fn render(frame: &mut Frame, area: Rect) {
        let connections = LsofStream::get_connections(); 
        let header_cells = ["PID", "Process", "Local", "Remote", "State", "Protocol"]
            .iter()
            .map(|header| {
                let style = Style::default().fg(Color::Yellow);
                Cell::from(*header).style(style)
            });
        let table_header = Row::new(header_cells).height(1);
        let table_rows = connections.iter().map(|connection| {
            Row::new([
                connection.pid.to_string(),
                connection.process.to_string(),
                connection.local.to_string(),
                connection.remote.to_string(),
                connection.state.to_string(),
                connection.protocol.to_string()
            ])
        });

        let table = Table::new(table_rows, [
            Constraint::Length(6),
            Constraint::Length(15),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Length(12),
            Constraint::Length(8)
        ])
        .header(table_header)
        .block(Block::default().borders(Borders::ALL).title("Connections"));

        frame.render_widget(table, area);
    } 
}
