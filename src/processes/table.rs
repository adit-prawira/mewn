use std::cmp::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

use crate::theme::Theme;

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
            SortType::Process => "NAME".into(),
            SortType::Pid => "PID".into(),
            SortType::Connection => "CONNECTION".into(),
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

#[derive(Default, PartialEq)]
enum AutoSortType {
    #[default]
    Upload,
    Download,
}

impl AutoSortType {
    pub fn as_display(&self) -> String {
        match self {
            AutoSortType::Upload => "UPLOAD".into(),
            AutoSortType::Download => "DOWNLOAD".into(),
        }
    }
}

#[derive(Default)]
pub struct TableComponent {
    selected_row: usize,
    scroll_offset: usize,
    filter_mode: FilterMode,
    sort_type: SortType,
    sort_mode: SortMode,
    auto_sort_type: AutoSortType,
    auto_sort_mode: SortMode,
    auto_sort_on: bool,
}

impl TableComponent {
    pub fn get_selected_row(&self) -> usize {
        self.selected_row
    }

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

    pub fn auto_sort_by_upload_rate(&mut self) {
        self.toggle_auto_sort_mode(AutoSortType::Upload, SortMode::Desc);
    }

    pub fn auto_sort_by_download_rate(&mut self) {
        self.toggle_auto_sort_mode(AutoSortType::Download, SortMode::Desc);
    }

    pub fn toggle_auto_sort_on(&mut self) {
        self.auto_sort_on = !self.auto_sort_on;
    }

    pub fn filter_by_mode(&self, process: &Process) -> bool {
        match self.filter_mode {
            FilterMode::All => true,
            FilterMode::Active => process.connections > 0,
        }
    }

    pub fn compare(&self, a: &Process, b: &Process) -> Ordering {
        if self.auto_sort_on {
            let ordering = match self.auto_sort_type {
                AutoSortType::Upload => a.upload_rate.cmp(&b.upload_rate),
                AutoSortType::Download => a.download_rate.cmp(&b.download_rate),
            };
            return match self.auto_sort_mode {
                SortMode::Desc => ordering.reverse(),
                SortMode::Asc => ordering,
            };
        }
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
    }

    pub fn render(&mut self, processes: Vec<&Process>, frame: &mut Frame, area: Rect) {
        self.selected_row = self.selected_row.min(processes.len().saturating_sub(1));
        let viewport = (area.height as usize).saturating_sub(3).max(1);

        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "Process", "PID", "Connections", "Upload", "Download", "CPU", "RAM", ""].iter().map(|header| {
            let style = Style::default().fg(Theme::text()).bold();
            Cell::from(*header).style(style)
        });

        let default_style_text = Style::default().fg(Theme::text_dim());
        let table_header = Row::new(header_cells).height(1);

        let table_rows = processes.iter().enumerate().skip(self.scroll_offset).take(viewport).map(|(index, process)| {
            let is_selected = index == self.selected_row;
            let selected_indicator = if is_selected { "▶".to_string() } else { String::from("") };
            let style = if is_selected {
                Style::default().fg(Theme::indicator()).bg(Theme::selected())
            } else {
                Style::default().fg(Theme::indicator())
            };

            Row::new([
                Cell::from(""),
                Cell::from(selected_indicator).style(default_style_text),
                Cell::from(process.process.to_string()).style(default_style_text),
                Cell::from(process.pid.to_string()).style(default_style_text),
                Cell::from(process.connections.to_string()).style(default_style_text),
                Cell::from(process.upload.to_string()).style(Style::default().fg(Theme::upload_rate())),
                Cell::from(process.download.to_string()).style(Style::default().fg(Theme::download_rate())),
                Cell::from(process.cpu.to_string()).style(Style::default().fg(Theme::cpu())),
                Cell::from(process.ram.to_string()).style(Theme::ram()),
                Cell::from(""),
            ])
            .style(style)
        });

        let (sort_mode_display, sort_type_display) = if self.auto_sort_on {
            (self.auto_sort_mode.as_display(), self.auto_sort_type.as_display())
        } else {
            (self.sort_mode.as_display(), self.sort_type.as_display())
        };

        let auto_mode_display = if self.auto_sort_on { "AUTOMATIC".to_string() } else { "MANUAL".to_string() };

        let filter_mode_display = self.filter_mode.as_display();

        let content_block = Block::default()
            .title(format!(
                "Processes [{}] [{} {}] [{}]",
                filter_mode_display, sort_type_display, sort_mode_display, auto_mode_display
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Theme::border()))
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

        frame.render_widget(table, area);
    }

    fn toggle_auto_sort_mode(&mut self, auto_sort_type: AutoSortType, default_sort_mode: SortMode) {
        if self.auto_sort_type == auto_sort_type {
            self.auto_sort_mode = match self.auto_sort_mode {
                SortMode::Desc => SortMode::Asc,
                SortMode::Asc => SortMode::Desc,
            };
        } else {
            self.auto_sort_type = auto_sort_type;
            self.auto_sort_mode = default_sort_mode;
        }
    }

    fn toggle_sort_mode(&mut self, sort_type: SortType, default_sort_mode: SortMode) {
        if self.auto_sort_on {
            self.auto_sort_on = false;
        }
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
