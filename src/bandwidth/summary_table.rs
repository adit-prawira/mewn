use std::collections::HashMap;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

use crate::theme::Theme;
use crate::utilities::bytes_format::BytesFormat;

use super::resource::{BandwidthStatistic, TotalBytesTransferredEntry};

#[derive(Default)]
pub struct SummaryTableComponent;

impl SummaryTableComponent {
    pub fn render(&self, bandwidth_statistics: Vec<&BandwidthStatistic>, total_bytes_registry: &HashMap<String, TotalBytesTransferredEntry>, frame: &mut Frame, area: Rect) {
        let header_cells = ["", "", "Name", "⬆ Total Upload (Bytes)", "⬇ Total Download (Bytes)", ""].iter().map(|header| {
            let style = Style::default().fg(Theme::text()).bold();
            Cell::from(*header).style(style)
        });
        let table_headers = Row::new(header_cells).height(1);
        let table_rows = bandwidth_statistics.iter().filter_map(|statistic| {
            let entry = total_bytes_registry.get(&statistic.name)?;
            let total_upload_bytes = BytesFormat::format_bytes(entry.total_upload_bytes as f64);
            let total_download_bytes = BytesFormat::format_bytes(entry.total_download_bytes as f64);

            Some(Row::new([
                Cell::from(""),
                Cell::from(""),
                Cell::from(statistic.name.to_string()).style(Theme::text()),
                Cell::from(total_upload_bytes).style(Theme::upload_rate()),
                Cell::from(total_download_bytes).style(Theme::download_rate()),
                Cell::from(""),
            ]))
        });
        let block = Block::default()
            .title("Total Bytes Transferred Summary")
            .style(Style::default().fg(Theme::text_highlight()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Theme::border()))
            .padding(Padding::new(2, 2, 1, 1));

        let table = Table::new(
            table_rows,
            [
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(10),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
                Constraint::Length(1),
            ],
        )
        .header(table_headers)
        .block(block);
        frame.render_widget(table, area);
    }
}
