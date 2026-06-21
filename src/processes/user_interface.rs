use std::collections::HashMap;
use std::time::{Duration, Instant};

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, BorderType, Borders, Cell, Chart, Dataset, GraphType, Padding, Row, Table};

use crate::theme::{GREEN, PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER, YELLOW};
use crate::utilities::bytes_format::BytesFormat;

use super::resource::Process;

#[derive(Default, PartialEq)]
enum FilterMode {
    #[default]
    All,
    Active,
}

impl FilterMode {
    pub fn as_display(&self) -> String {
        match self {
            FilterMode::All => "ALL".into(),
            FilterMode::Active => "ACTIVE".into(),
        }
    }
}

#[derive(Default, PartialEq)]
enum SortType {
    #[default]
    Connection,
    Ram,
    Process,
    Pid,
    Cpu,
}

impl SortType {
    pub fn as_display(&self) -> String {
        match self {
            SortType::Ram => "RAM".into(),
            SortType::Process => "Name".into(),
            SortType::Pid => "PID".into(),
            SortType::Connection => "Connection".into(),
            SortType::Cpu => "CPU".into(),
        }
    }
}

#[derive(Default, PartialEq)]
enum SortMode {
    #[default]
    Desc,
    Asc,
}

impl SortMode {
    pub fn as_display(&self) -> String {
        match self {
            SortMode::Asc => "↑".into(),
            SortMode::Desc => "↓".into(),
        }
    }
}

#[derive(Default)]
pub struct ProcessUserInterface {
    selected_row: usize,
    scroll_offset: usize,
    filter_mode: FilterMode,
    sort_type: SortType,
    sort_mode: SortMode,
    cpu_percent_history: HashMap<u32, Vec<f64>>,
    ram_size_history: HashMap<u32, Vec<f64>>,
    upload_rate_history: HashMap<u32, Vec<f64>>,
    download_rate_history: HashMap<u32, Vec<f64>>,
    last_push_at: Option<Instant>,
}

impl ProcessUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, processes: &[Process]) {
        let mut filtered_processes: Vec<&Process> = processes
            .iter()
            .filter(|process| -> bool {
                match self.filter_mode {
                    FilterMode::All => true,
                    FilterMode::Active => process.connections > 0,
                }
            })
            .collect();

        filtered_processes.sort_by(|a, b| {
            let ordering = match self.sort_type {
                SortType::Ram => a.ram_bytes.cmp(&b.ram_bytes),
                SortType::Process => a.process.cmp(&b.process),
                SortType::Pid => a.pid.cmp(&b.pid),
                SortType::Connection => a.connections.cmp(&b.connections),
                SortType::Cpu => a.cpu_percent.total_cmp(&b.cpu_percent),
            };

            match self.sort_mode {
                SortMode::Desc => ordering.reverse(),
                SortMode::Asc => ordering,
            }
        });

        self.selected_row = self.selected_row.min(filtered_processes.len().saturating_sub(1));
        let [table_area, graph_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Percentage(40)])
            .areas::<2>(area);
        let viewport = (table_area.height as usize).saturating_sub(3).max(1);
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

        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "Process", "PID", "Connections", "Upload", "Download", "CPU", "RAM", ""].iter().map(|header| {
            let style = Style::default().fg(TEXT_COLOR).bold();
            Cell::from(*header).style(style)
        });

        let default_style_text = Style::default().fg(TEXT_COLOR_DARKER);
        let table_header = Row::new(header_cells).height(1);

        let table_rows = filtered_processes.iter().enumerate().skip(self.scroll_offset).take(viewport).map(|(index, process)| {
            let is_selected = index == self.selected_row;
            let selected_indicator = if is_selected { "▶".to_string() } else { String::from("") };
            let style = if is_selected {
                Style::default().fg(Color::Gray).bg(Color::Rgb(132, 75, 92))
            } else {
                Style::default().fg(Color::Gray)
            };

            Row::new([
                Cell::from(""),
                Cell::from(selected_indicator).style(default_style_text),
                Cell::from(process.process.to_string()).style(Style::default().fg(PRIMARY)),
                Cell::from(process.pid.to_string()).style(default_style_text),
                Cell::from(process.connections.to_string()).style(default_style_text),
                Cell::from(process.upload.to_string()).style(Style::default().fg(GREEN)),
                Cell::from(process.download.to_string()).style(Style::default().fg(YELLOW)),
                Cell::from(process.cpu.to_string()).style(Style::default().fg(YELLOW)),
                Cell::from(process.ram.to_string()).style(default_style_text),
                Cell::from(""),
            ])
            .style(style)
        });

        let sort_mode_display = self.sort_mode.as_display();
        let sort_type_display = self.sort_type.as_display();
        let filter_mode_display = self.filter_mode.as_display();

        let content_block = Block::default()
            .title(format!("Processes [{}] [{}{}]", filter_mode_display, sort_type_display, sort_mode_display))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(PRIMARY))
            .padding(Padding::new(2, 2, 1, 1));

        let table = Table::new(
            table_rows,
            [
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Percentage(20),
                Constraint::Length(8),
                Constraint::Length(12),
                Constraint::Length(14),
                Constraint::Length(14),
                Constraint::Length(8),
                Constraint::Length(12),
                Constraint::Length(1),
            ],
        )
        .header(table_header)
        .block(content_block);

        frame.render_widget(table, table_area);

        let Some(selected_process) = filtered_processes.get(self.selected_row) else {
            return;
        };

        let [top_graph_area, bottom_graph_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .areas::<2>(graph_area);

        let [upload_area, download_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas::<2>(top_graph_area);

        let [cpu_area, ram_area] = Layout::default()
            .direction(Direction::Horizontal)
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

    pub fn toggle_filter_mode(&mut self) {
        self.filter_mode = match self.filter_mode {
            FilterMode::All => FilterMode::Active,
            FilterMode::Active => FilterMode::All,
        }
    }

    pub fn sort_by_pid(&mut self) {
        self.toggle_sort_mode(SortType::Pid, SortMode::Desc);
    }

    pub fn sort_by_process_name(&mut self) {
        self.toggle_sort_mode(SortType::Process, SortMode::Desc);
    }

    pub fn sort_by_connections(&mut self) {
        self.toggle_sort_mode(SortType::Connection, SortMode::Desc);
    }

    pub fn sort_by_cpu(&mut self) {
        self.toggle_sort_mode(SortType::Cpu, SortMode::Desc);
    }

    pub fn sort_by_ram(&mut self) {
        self.toggle_sort_mode(SortType::Ram, SortMode::Desc);
    }

    pub fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    pub fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    fn toggle_sort_mode(&mut self, sort_type: SortType, default_sort_mode: SortMode) {
        if self.sort_type == sort_type {
            self.sort_mode = match self.sort_mode {
                SortMode::Desc => SortMode::Asc,
                SortMode::Asc => SortMode::Desc,
            };
        } else {
            self.sort_type = sort_type;
            self.sort_mode = default_sort_mode;
        }
    }
}

struct UploadChartComponent;

impl UploadChartComponent {
    pub fn render(data: &[f64], process: &Process, frame: &mut Frame, area: Rect) {
        let upload_points: Vec<(f64, f64)> = data.iter().enumerate().map(|(index, &datum)| (index as f64, datum)).collect();
        let upload_max = data.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let upload_dataset = Dataset::default()
            .graph_type(GraphType::Area)
            .marker(Marker::Braille)
            .style(Style::default().fg(GREEN))
            .data(&upload_points);
        let upload_chart = Chart::new(vec![upload_dataset])
            .block(
                Block::default()
                    .title(format!("Upload Rate ({}) [max: {}]", process.upload, BytesFormat::format_bytes_per_seconds(upload_max)))
                    .title_style(Style::default().fg(TEXT_COLOR))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(PRIMARY)),
            )
            .x_axis(
                Axis::default()
                    .title("Seconds")
                    .bounds([0.0, upload_points.len() as f64])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            )
            .y_axis(
                Axis::default()
                    .title("Bytes/s")
                    .bounds([0.0, upload_max.max(1.0)])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            );

        frame.render_widget(upload_chart, area);
    }
}

struct DownloadChartComponent;

impl DownloadChartComponent {
    pub fn render(data: &[f64], process: &Process, frame: &mut Frame, area: Rect) {
        let download_points: Vec<(f64, f64)> = data.iter().enumerate().map(|(index, &datum)| (index as f64, datum)).collect();
        let download_max = data.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let download_dataset = Dataset::default()
            .graph_type(GraphType::Area)
            .marker(Marker::Braille)
            .style(Style::default().fg(YELLOW))
            .data(&download_points);
        let download_chart = Chart::new(vec![download_dataset])
            .block(
                Block::default()
                    .title(format!(
                        "Download Rate ({}) [max: {}]",
                        process.download,
                        BytesFormat::format_bytes_per_seconds(download_max)
                    ))
                    .title_style(Style::default().fg(TEXT_COLOR))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(PRIMARY)),
            )
            .x_axis(
                Axis::default()
                    .title("Seconds")
                    .bounds([0.0, download_points.len() as f64])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            )
            .y_axis(
                Axis::default()
                    .title("Bytes/s")
                    .bounds([0.0, download_max.max(1.0)])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            );

        frame.render_widget(download_chart, area);
    }
}

struct CpuChartComponent;

impl CpuChartComponent {
    pub fn render(data: &[f64], process: &Process, frame: &mut Frame, area: Rect) {
        let cpu_percent_points: Vec<(f64, f64)> = data.iter().enumerate().map(|(index, &datum)| (index as f64, datum)).collect();
        let cpu_percent_max = data.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let cpu_percent_dataset = Dataset::default()
            .graph_type(GraphType::Area)
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&cpu_percent_points);
        let cpu_chart = Chart::new(vec![cpu_percent_dataset])
            .block(
                Block::default()
                    .title(format!("CPU Usage ({:.2}%) [max: {:.2}%]", process.cpu_percent, cpu_percent_max))
                    .title_style(Style::default().fg(TEXT_COLOR))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(PRIMARY)),
            )
            .x_axis(
                Axis::default()
                    .title("Seconds")
                    .bounds([0.0, cpu_percent_points.len() as f64])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            )
            .y_axis(
                Axis::default()
                    .title("%")
                    .bounds([0.0, cpu_percent_max.max(1.0)])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            );
        frame.render_widget(cpu_chart, area);
    }
}

struct RamChartComponent;

impl RamChartComponent {
    pub fn render(data: &[f64], process: &Process, frame: &mut Frame, area: Rect) {
        let ram_size_points: Vec<(f64, f64)> = data.iter().enumerate().map(|(index, &datum)| (index as f64, datum)).collect();
        let ram_size_max = data.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let ram_size_dataset = Dataset::default()
            .graph_type(GraphType::Area)
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Magenta))
            .data(&ram_size_points);

        let ram_chart = Chart::new(vec![ram_size_dataset])
            .block(
                Block::default()
                    .title(format!("RAM ({}) [max: {}]", process.ram, BytesFormat::format_bytes(ram_size_max)))
                    .title_style(Style::default().fg(TEXT_COLOR))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(PRIMARY)),
            )
            .x_axis(
                Axis::default()
                    .title("Seconds")
                    .bounds([0.0, ram_size_points.len() as f64])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            )
            .y_axis(
                Axis::default()
                    .title("Bytes")
                    .bounds([0.0, ram_size_max.max(1.0)])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            );
        frame.render_widget(ram_chart, area);
    }
}
