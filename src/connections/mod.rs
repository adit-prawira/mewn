use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

pub struct Connection {
    pid: u32,
    process: String,
    local: String, 
    remote: String, 
    state: String, 
    protocol: String
}

impl Connection {
    pub fn render(frame: &mut Frame, area: Rect) {
        let connections = Self::mock_data();
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

    fn mock_data() -> Vec<Connection> {
        vec![
            Connection {
                pid: 1234,
                process: "firefox".to_string(),
                local: "192.168.1.100:54321".to_string(),
                remote: "142.250.80.46:443".to_string(),
                state: "ESTABLISHED".to_string(),
                protocol: "TCP".to_string(),
            },
            Connection {
                pid: 5678,
                process: "chrome".to_string(),
                local: "192.168.1.100:54322".to_string(),
                remote: "151.101.1.140:443".to_string(),
                state: "ESTABLISHED".to_string(),
                protocol: "TCP".to_string(),
            },
            Connection {
                pid: 9012,
                process: "spotify".to_string(),
                local: "192.168.1.100:54323".to_string(),
                remote: "35.186.224.25:443".to_string(),
                state: "TIME_WAIT".to_string(),
                protocol: "TCP".to_string(),
            },
        ]
    }
}
