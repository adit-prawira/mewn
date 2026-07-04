use std::collections::HashSet;

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

use crate::theme::Theme;

use super::resource::BandwidthStatistic;

#[derive(Default)]
pub struct TableComponent {
    selected_row: usize,
    scroll_offset: usize,
}

impl TableComponent {
    pub fn render(&mut self, bandwidth_statistics: &[&BandwidthStatistic], active_interface_names: &HashSet<String>, frame: &mut Frame, area: Rect) {
        self.selected_row = self.selected_row.min(bandwidth_statistics.len().saturating_sub(1));
        let viewport = (area.height as usize).saturating_sub(3).max(1);

        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "", "Name", "Address", "Upload", "Download", "Total", "Maximum Transmission Unit", ""]
            .iter()
            .map(|header| {
                let style = Style::default().fg(Theme::text()).bold();
                Cell::from(*header).style(style)
            });

        let default_text_style = Style::default().fg(Theme::text_dim());
        let table_header = Row::new(header_cells).height(1);
        let table_rows = bandwidth_statistics
            .iter()
            .enumerate()
            .skip(self.scroll_offset)
            .take(viewport)
            .map(|(index, bandwidth_statistic)| {
                let is_selected = index == self.selected_row;
                let selected_indicator = if is_selected { "▶".to_string() } else { String::from("") };
                let style = if is_selected {
                    Style::default().fg(Theme::indicator()).bg(Theme::selected())
                } else {
                    Style::default().fg(Theme::indicator())
                };

                let is_active = active_interface_names.contains(&bandwidth_statistic.name);
                let active_indicator = if is_active { "\u{25CF}" } else { "\u{25CB}" };
                let active_indicator_style = if is_active {
                    Style::default().fg(Theme::text_highlight())
                } else {
                    Style::default().fg(Theme::text_dim())
                };

                Row::new([
                    Cell::from(""),
                    Cell::from(selected_indicator).style(default_text_style),
                    Cell::from(active_indicator).style(active_indicator_style),
                    Cell::from(bandwidth_statistic.name.to_string()).style(default_text_style),
                    Cell::from(bandwidth_statistic.address.to_string()).style(default_text_style),
                    Cell::from(bandwidth_statistic.upload.to_string()).style(Theme::upload_rate()),
                    Cell::from(bandwidth_statistic.download.to_string()).style(Theme::download_rate()),
                    Cell::from(bandwidth_statistic.total.to_string()).style(default_text_style),
                    Cell::from(Line::from(Span::raw(&bandwidth_statistic.maximum_transmission_unit)).alignment(Alignment::Right)).style(default_text_style),
                    Cell::from(""),
                ])
                .style(style)
            });
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Theme::border()))
            .padding(Padding::new(2, 2, 0, 0));

        let table = Table::new(
            table_rows,
            [
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(10),
                Constraint::Length(20),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Length(25),
                Constraint::Length(1),
            ],
        )
        .header(table_header)
        .block(content_block);

        frame.render_widget(table, area);
    }

    pub fn reset_selection(&mut self) {
        self.scroll_offset = 0;
        self.selected_row = 0;
    }

    pub fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    pub fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    pub fn get_selected_row(&self) -> usize {
        self.selected_row
    }
}
