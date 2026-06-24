use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::resource::Connection;
use super::search_bar::SearchBarComponent;
use super::table::TableComponent;

#[derive(Default)]
pub struct ConnectionUserInterface {
    search_bar_component: SearchBarComponent,
    table_component: TableComponent,
}

impl ConnectionUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, connections: &[Connection]) {
        let [search_area, table_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas::<2>(area);
        let search_query = self.search_bar_component.get_search_query();

        self.search_bar_component.render(frame, search_area);

        let filtered_connections: Vec<&Connection> = if search_query.is_empty() {
            connections.iter().collect()
        } else {
            let query = search_query.to_lowercase();
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

        let title = if search_query.is_empty() {
            "Connections".into()
        } else {
            format!("Connections [{}]", search_query)
        };

        self.table_component.render(filtered_connections, title, frame, table_area);
    }

    pub fn handle_keys(&mut self, key_code: KeyCode) {
        if self.search_bar_component.is_active() {
            match key_code {
                KeyCode::Esc => {
                    self.search_bar_component.inactive();
                    self.search_bar_component.reset();
                    self.table_component.reset_selection();
                }
                KeyCode::Enter => {
                    self.search_bar_component.inactive();
                }
                KeyCode::Backspace => {
                    self.search_bar_component.remove_search_char();
                    self.table_component.reset_selection();
                }
                KeyCode::Char(c) => {
                    self.search_bar_component.add_search_char(c);
                    self.table_component.reset_selection();
                }
                _ => {}
            }
        }
        match key_code {
            KeyCode::Up => self.table_component.previous_row(),
            KeyCode::Down => self.table_component.next_row(),
            KeyCode::Char('/') => {
                self.search_bar_component.active();
            }
            _ => {}
        }
    }

    pub fn is_searching(&self) -> bool {
        self.search_bar_component.is_active()
    }
}
