use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Gauge};

use crate::theme::Theme;
use crate::utilities::bytes_format::BytesFormat;

pub struct UtilisationGauge;

impl UtilisationGauge {
    pub fn render(interface_name: &str, upload_rate: u64, download_rate: u64, capacity_bps: u64, frame: &mut Frame, area: Rect) {
        let total_rate = upload_rate + download_rate;
        let effective_capacity = capacity_bps.max(125_000);
        let utilisation = ((total_rate as f64 / effective_capacity as f64) * 100.0).min(100.0);

        let color = if utilisation < 50.0 {
            Theme::info()
        } else if utilisation < 80.0 {
            Theme::warning()
        } else {
            Theme::danger()
        };

        let label = format!(
            "{} Utilisation: {:.1}% ({})",
            interface_name,
            utilisation,
            BytesFormat::format_bytes_per_seconds(total_rate as f64)
        );

        let block = Block::default()
            .title(label)
            .title_style(Style::default().fg(Theme::text()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Theme::border()));
        let gauge = Gauge::default().block(block).gauge_style(Style::default().fg(color)).percent(utilisation as u16);
        frame.render_widget(gauge, area);
    }
}
