use std::collections::HashMap;
use std::time::{Duration, Instant};

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::atoms::search_bar::SearchBarComponent;

use super::cpu_chart::CpuChartComponent;
use super::download_chart::DownloadChartComponent;
use super::ram_chart::RamChartComponent;
use super::resource::Process;
use super::table::TableComponent;
use super::upload_chart::UploadChartComponent;

#[derive(Default)]
pub struct ProcessUserInterface {
    cpu_percent_history: HashMap<u32, Vec<f64>>,
    ram_size_history: HashMap<u32, Vec<f64>>,
    upload_rate_history: HashMap<u32, Vec<f64>>,
    download_rate_history: HashMap<u32, Vec<f64>>,
    last_push_at: Option<Instant>,
    table_component: TableComponent,
    search_bar_component: SearchBarComponent,
}

impl ProcessUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, processes: &[Process]) {
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
        let mut filtered_processes: Vec<&Process> = processes
            .iter()
            .filter(|process| -> bool {
                let mode_filter_result = self.table_component.filter_by_mode(process);

                if is_search_query_empty {
                    return mode_filter_result;
                }

                mode_filter_result && (process.process.to_lowercase().contains(&search_query) || process.pid.to_string().to_lowercase().contains(&search_query))
            })
            .collect();

        filtered_processes.sort_by(|a, b| self.table_component.compare(a, b));

        let should_push = self.last_push_at.is_none_or(|time| time.elapsed() >= Duration::from_secs(1));

        if should_push {
            for process in &filtered_processes {
                let upload_rate = self.upload_rate_history.entry(process.pid).or_default();
                upload_rate.truncate(59);
                upload_rate.push(process.upload_rate as f64);

                let download_rate = self.download_rate_history.entry(process.pid).or_default();
                download_rate.truncate(59);
                download_rate.push(process.download_rate as f64);

                let cpu_percent = self.cpu_percent_history.entry(process.pid).or_default();
                cpu_percent.truncate(59);
                cpu_percent.push(process.cpu_percent);

                let ram_size = self.ram_size_history.entry(process.pid).or_default();
                ram_size.truncate(59);
                ram_size.push(process.ram_bytes as f64);
            }
            self.last_push_at = Some(Instant::now());
        }

        self.table_component.render(filtered_processes.clone(), frame, table_area);
        let Some(selected_process) = filtered_processes.get(self.table_component.get_selected_row()) else {
            return;
        };

        let [top_graph_area, bottom_graph_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .areas::<2>(graph_area);

        let bandwidth_chart_alignment = if is_wide { Direction::Vertical } else { Direction::Horizontal };
        let [upload_area, download_area] = Layout::default()
            .direction(bandwidth_chart_alignment)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas::<2>(top_graph_area);

        let machine_chart_alignment = if is_wide { Direction::Vertical } else { Direction::Horizontal };
        let [cpu_area, ram_area] = Layout::default()
            .direction(machine_chart_alignment)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas::<2>(bottom_graph_area);

        let upload_data = self.upload_rate_history.get(&selected_process.pid).map(|datum| datum.as_slice()).unwrap_or(&[]);
        let download_data = self.download_rate_history.get(&selected_process.pid).map(|datum| datum.as_slice()).unwrap_or(&[]);
        let cpu_percent_data = self.cpu_percent_history.get(&selected_process.pid).map(|datum| datum.as_slice()).unwrap_or(&[]);
        let ram_size_data = self.ram_size_history.get(&selected_process.pid).map(|datum| datum.as_slice()).unwrap_or(&[]);

        UploadChartComponent::render(upload_data, selected_process, frame, upload_area);
        DownloadChartComponent::render(download_data, selected_process, frame, download_area);
        CpuChartComponent::render(cpu_percent_data, selected_process, frame, cpu_area);
        RamChartComponent::render(ram_size_data, selected_process, frame, ram_area);
    }

    pub fn is_searching(&self) -> bool {
        self.search_bar_component.is_active()
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
            KeyCode::Char('f') | KeyCode::Char('F') => self.table_component.toggle_filter_mode(),
            KeyCode::Char('p') | KeyCode::Char('P') => self.table_component.sort_by_pid(),
            KeyCode::Char('n') | KeyCode::Char('N') => self.table_component.sort_by_process_name(),
            KeyCode::Char('c') | KeyCode::Char('C') => self.table_component.sort_by_connections(),
            KeyCode::Char('r') | KeyCode::Char('R') => self.table_component.sort_by_cpu(),
            KeyCode::Char('m') | KeyCode::Char('M') => self.table_component.sort_by_ram(),
            KeyCode::Char('s') | KeyCode::Char('S') => self.table_component.toggle_auto_sort_on(),
            KeyCode::Char('u') | KeyCode::Char('U') => self.table_component.auto_sort_by_upload_rate(),
            KeyCode::Char('d') | KeyCode::Char('D') => self.table_component.auto_sort_by_download_rate(),
            KeyCode::Char('/') => self.search_bar_component.active(),
            _ => {}
        }
    }
}
