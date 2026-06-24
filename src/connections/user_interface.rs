use crate::theme::{BLUE, GREEN, PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER, YELLOW, YELLOW_DARKER};
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Table};

use super::resource::Connection;

#[derive(Default)]
pub struct ConnectionUserInterface {
    selected_row: usize,
    scroll_offset: usize,
    search_active: bool,
    search_query: String,
    tick_count: u64,
}

impl ConnectionUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, connections: &[Connection]) {
        let [search_area, table_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas::<2>(area);

        // cursor should blink
        let search_cursor = if self.search_active && self.tick_count % 15 < 7 { "█" } else { " " };
        let search_text = if self.search_active || !self.search_query.is_empty() {
            format!("SEARCH: {}{}", self.search_query, search_cursor)
        } else {
            "Press / to search ...".into()
        };

        let search_style = if self.search_active {
            Style::default().fg(TEXT_COLOR)
        } else {
            Style::default().fg(TEXT_COLOR_DARKER)
        };

        let search_span = Span::styled(search_text, search_style);
        let search_bar_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(PRIMARY))
            .padding(Padding::new(1, 1, 0, 0));
        let search_bar = Paragraph::new(search_span).block(search_bar_block);

        frame.render_widget(search_bar, search_area);

        let filtered_connections: Vec<&Connection> = if self.search_query.is_empty() {
            connections.iter().collect()
        } else {
            let query = self.search_query.to_lowercase();
            connections
                .iter()
                .filter(|connection| {
                    connection.process.to_lowercase().contains(&query)
                        || connection.pid.to_string().to_lowercase().contains(&query)
                        || connection.protocol.to_lowercase().contains(&query)
                        || connection.local.to_lowercase().contains(&query)
                        || connection.remote.to_lowercase().contains(&query)
                        || connection.state.to_lowercase().contains(&query)
                })
                .collect()
        };

        self.selected_row = self.selected_row.min(filtered_connections.len().saturating_sub(1));
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
        let table_rows = filtered_connections.iter().enumerate().skip(self.scroll_offset).take(viewport).map(|(index, connection)| {
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

        let title = if self.search_query.is_empty() {
            "Connections".into()
        } else {
            format!("Connections [{}]", self.search_query)
        };

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

        frame.render_widget(table, table_area);
    }

    pub fn handle_keys(&mut self, key_code: KeyCode) {
        if self.search_active {
            match key_code {
                KeyCode::Esc => {
                    self.search_active = false;
                    self.search_query.clear();
                    self.selected_row = 0;
                    self.scroll_offset = 0;
                }
                KeyCode::Enter => {
                    self.search_active = false;
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                    self.scroll_offset = 0;
                    self.selected_row = 0;
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                    self.scroll_offset = 0;
                    self.selected_row = 0;
                }
                _ => {}
            }
        }
        match key_code {
            KeyCode::Up => self.previous_row(),
            KeyCode::Down => self.next_row(),
            KeyCode::Char('/') => {
                self.search_active = true;
            }
            _ => {}
        }
    }

    pub fn is_searching(&mut self) -> bool {
        self.search_active
    }

    fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }
}
