use std::collections::HashMap;
use std::time::{Duration, Instant};

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::atoms::search_bar::SearchBarComponent;

use super::download_chart::DownloadChartComponent;
use super::resource::BandwidthStatistic;
use super::table::TableComponent;
use super::upload_chart::UploadChartComponent;

#[derive(Default)]
pub struct BandwidthUserInterface {
    upload_history: HashMap<String, Vec<u64>>,
    download_history: HashMap<String, Vec<u64>>,
    last_push_at: Option<Instant>,
    search_bar_component: SearchBarComponent,
    table_component: TableComponent,
}

impl BandwidthUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, bandwidth_statistics: &[BandwidthStatistic]) {
        let is_wide = area.width > 100 && area.width as f32 / area.height.max(1) as f32 > 1.5;
        let alignment = if is_wide { Direction::Horizontal } else { Direction::Vertical };

        let [main_area, graph_area] = Layout::default()
            .direction(alignment)
            .constraints([Constraint::Fill(1), Constraint::Percentage(45)])
            .areas::<2>(area);
        let [search_area, table_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas::<2>(main_area);

        let search_query = self.search_bar_component.get_search_query();
        let is_search_query_empty = search_query.is_empty();

        self.search_bar_component.render(frame, search_area);

        let filtered_bandwidth_statistics: Vec<&BandwidthStatistic> = if is_search_query_empty {
            bandwidth_statistics.iter().collect()
        } else {
            bandwidth_statistics
                .iter()
                .filter(|statistic| statistic.name.to_lowercase().contains(&search_query) || statistic.address.to_lowercase().contains(&search_query))
                .collect()
        };
        let should_push = self.last_push_at.is_none_or(|time| time.elapsed() >= Duration::from_secs(1));

        if should_push {
            // updating ring buffer and track 60 seconds worth of history data
            for statistics in bandwidth_statistics {
                let download = self.download_history.entry(statistics.name.to_string()).or_default();
                download.truncate(59);
                download.push(statistics.download_rate);

                let upload = self.upload_history.entry(statistics.name.to_string()).or_default();
                upload.truncate(59);
                upload.push(statistics.upload_rate);
            }
            self.last_push_at = Some(Instant::now());
        }

        self.table_component.render(filtered_bandwidth_statistics.clone(), frame, table_area);

        let Some(selected_statistic) = filtered_bandwidth_statistics.get(self.table_component.get_selected_row()) else {
            return;
        };

        let [upload_area, download_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .areas::<2>(graph_area);

        let upload_data = self.upload_history.get(&selected_statistic.name).map(|datum| datum.as_slice()).unwrap_or(&[]);
        let download_data = self.download_history.get(&selected_statistic.name).map(|datum| datum.as_slice()).unwrap_or(&[]);

        UploadChartComponent::render(upload_data, selected_statistic, frame, upload_area);
        DownloadChartComponent::render(download_data, selected_statistic, frame, download_area);
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
            KeyCode::Char('/') => self.search_bar_component.active(),
            _ => {}
        }
    }

    pub fn is_searching(&self) -> bool {
        self.search_bar_component.is_active()
    }
}
