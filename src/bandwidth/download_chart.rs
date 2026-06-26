use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType};

use crate::theme::{PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER, YELLOW};
use crate::utilities::bytes_format::BytesFormat;

use super::resource::BandwidthStatistic;

pub struct DownloadChartComponent;

impl DownloadChartComponent {
    pub fn render(data: &[u64], statistic: &BandwidthStatistic, frame: &mut Frame, area: Rect) {
        let download_points: Vec<(f64, f64)> = data.iter().enumerate().map(|(index, &datum)| (index as f64, datum as f64)).collect();
        let download_max = data.iter().max().copied().unwrap_or(0);

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
                        statistic.download,
                        BytesFormat::format_bytes_per_seconds(download_max as f64)
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
                    .bounds([0.0, (download_max as f64).max(1.0)])
                    .style(Style::default().fg(TEXT_COLOR_DARKER)),
            );

        frame.render_widget(download_chart, area);
    }
}
