use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType};

use crate::theme::Theme;
use crate::utilities::bytes_format::BytesFormat;

use super::resource::Process;

pub struct DownloadChartComponent;

impl DownloadChartComponent {
    pub fn render(data: &[f64], process: &Process, frame: &mut Frame, area: Rect) {
        let download_points: Vec<(f64, f64)> = data.iter().enumerate().map(|(index, &datum)| (index as f64, datum)).collect();
        let download_max = data.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let download_dataset = Dataset::default()
            .graph_type(GraphType::Area)
            .marker(Marker::Braille)
            .style(Style::default().fg(Theme::download_rate()))
            .data(&download_points);
        let download_chart = Chart::new(vec![download_dataset])
            .block(
                Block::default()
                    .title(format!(
                        "Download Rate ({}) [max: {}]",
                        process.download,
                        BytesFormat::format_bytes_per_seconds(download_max)
                    ))
                    .title_style(Style::default().fg(Theme::text()))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Theme::border())),
            )
            .x_axis(
                Axis::default()
                    .title("Seconds")
                    .bounds([0.0, download_points.len() as f64])
                    .style(Style::default().fg(Theme::text_dim())),
            )
            .y_axis(
                Axis::default()
                    .title("Bytes/s")
                    .bounds([0.0, download_max.max(1.0)])
                    .style(Style::default().fg(Theme::text_dim())),
            );

        frame.render_widget(download_chart, area);
    }
}
