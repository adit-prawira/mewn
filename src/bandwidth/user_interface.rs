use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

use crate::theme::{GREEN, PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER, YELLOW};

use super::resource::BandwidthStatistic;

#[derive(Default)]
pub struct BandwidthUserInterface {
    selected_row: usize,
    scroll_offset: usize
}

impl BandwidthUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, bandwidth_statistics: &[BandwidthStatistic]) {
        self.selected_row = self.selected_row.min(bandwidth_statistics.len().saturating_sub(1));
        let viewport = (area.height as usize).saturating_sub(3).max(1);
        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "Name", "Address", "Upload", "Download", "Total", "Maximum Transmission Unit", ""]
            .iter() 
            .map(|header| {
                let style = Style::default().fg(TEXT_COLOR).bold();
                Cell::from(*header).style(style)
            });
        let default_text_style = Style::default().fg(TEXT_COLOR_DARKER);
        let table_header = Row::new(header_cells).height(1);
        let table_rows = bandwidth_statistics.iter().enumerate()
            .skip(self.scroll_offset)
            .take(viewport)
            .map(|(index, bandwidth_statistic)| {
                let is_selected = index == self.selected_row;
                let selected_indicator = if is_selected {"▶".to_string()} else {String::from("")};
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Gray)
                        .bg(Color::Rgb(132, 75, 92))
                } else {
                    Style::default()
                        .fg(Color::Gray)
                };

                Row::new([
                    Cell::from(""),
                    Cell::from(selected_indicator).style(default_text_style),
                    Cell::from(bandwidth_statistic.name.to_string()).style(default_text_style),
                    Cell::from(bandwidth_statistic.address.to_string()).style(default_text_style),
                    Cell::from(bandwidth_statistic.upload.to_string()).style(Style::default().fg(GREEN)),
                    Cell::from(bandwidth_statistic.download.to_string()).style(Style::default().fg(YELLOW)),
                    Cell::from(bandwidth_statistic.total.to_string()).style(PRIMARY), 
                    Cell::from(
                       Line::from(Span::raw(&bandwidth_statistic.maximum_transmission_unit))
                        .alignment(Alignment::Right)
                    ).style(default_text_style),
                    Cell::from("")
                ]).style(style)
            });
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(PRIMARY))
            .padding(Padding::new(2, 2, 1, 1));

        let table = Table::new(table_rows, [
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15), 
            Constraint::Length(25),
            Constraint::Length(1)
        ])
        .header(table_header)
        .block(content_block);

        frame.render_widget(table, area);
    }

    pub fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    pub fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }
}
