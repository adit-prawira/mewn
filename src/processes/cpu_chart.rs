use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType};

use crate::theme::Theme;

use super::resource::Process;

pub struct CpuChartComponent;

impl CpuChartComponent {
    pub fn render(data: &[f64], process: &Process, frame: &mut Frame, area: Rect) {
        let cpu_percent_points: Vec<(f64, f64)> = data.iter().enumerate().map(|(index, &datum)| (index as f64, datum)).collect();
        let cpu_percent_max = data.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let cpu_percent_dataset = Dataset::default()
            .graph_type(GraphType::Area)
            .marker(Marker::Braille)
            .style(Style::default().fg(Theme::cpu()))
            .data(&cpu_percent_points);
        let cpu_chart = Chart::new(vec![cpu_percent_dataset])
            .block(
                Block::default()
                    .title(format!("CPU Usage ({:.2}%) [max: {:.2}%]", process.cpu_percent, cpu_percent_max))
                    .title_style(Style::default().fg(Theme::text()))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Theme::border())),
            )
            .x_axis(
                Axis::default()
                    .title("Seconds")
                    .bounds([0.0, cpu_percent_points.len() as f64])
                    .style(Style::default().fg(Theme::text_dim())),
            )
            .y_axis(
                Axis::default()
                    .title("%")
                    .bounds([0.0, cpu_percent_max.max(1.0)])
                    .style(Style::default().fg(Theme::text_dim())),
            );
        frame.render_widget(cpu_chart, area);
    }
}
