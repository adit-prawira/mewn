use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType};

use crate::theme::{PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER};
use crate::utilities::bytes_format::BytesFormat;

use super::resource::Process;

pub struct RamChartComponent;

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
