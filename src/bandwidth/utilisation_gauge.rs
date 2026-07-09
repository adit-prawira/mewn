use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Gauge};

use crate::theme::Theme;
use crate::utilities::bytes_format::BytesFormat;

pub struct UtilisationGauge;

impl UtilisationGauge {
    pub fn render(interface_name: &str, upload_rate: u64, download_rate: u64, capacity_bps: u64, frame: &mut Frame, area: Rect) {
        let total_rate = upload_rate + download_rate;
        let utilisation = Self::calculate_utilisation(upload_rate, download_rate, capacity_bps);
        let color = Self::gauge_color(utilisation);

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

    fn calculate_utilisation(upload_rate: u64, download_rate: u64, capacity_bps: u64) -> f64 {
        let total_rate = upload_rate + download_rate;
        let effective_capacity = capacity_bps.max(125_000);
        ((total_rate as f64 / effective_capacity as f64) * 100.0).min(100.0)
    }

    fn gauge_color(utilisation: f64) -> Color {
        if utilisation < 50.0 {
            Theme::info()
        } else if utilisation < 80.0 {
            Theme::warning()
        } else {
            Theme::danger()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_traffic_then_utilisation_is_zero() {
        assert_eq!(UtilisationGauge::calculate_utilisation(0, 0, 1_000_000), 0.0);
    }

    #[test]
    fn given_half_capacity_then_utilisation_is_fifty() {
        assert_eq!(UtilisationGauge::calculate_utilisation(250_000, 250_000, 1_000_000), 50.0);
    }

    #[test]
    fn given_full_capacity_then_utilisation_is_hundred() {
        assert_eq!(UtilisationGauge::calculate_utilisation(600_000, 400_000, 1_000_000), 100.0);
    }

    #[test]
    fn given_excess_traffic_then_utilisation_capped_at_hundred() {
        assert_eq!(UtilisationGauge::calculate_utilisation(1_000_000, 1_000_000, 1_000_000), 100.0);
    }

    #[test]
    fn given_capacity_below_minimum_then_uses_minimum_125k() {
        assert_eq!(UtilisationGauge::calculate_utilisation(62_500, 62_500, 10_000), 100.0);
    }

    #[test]
    fn given_upload_only_traffic_then_utilisation_counts_both_directions() {
        assert_eq!(UtilisationGauge::calculate_utilisation(1_000_000, 0, 1_000_000), 100.0);
    }

    #[test]
    fn given_download_only_traffic_then_utilisation_counts_both_directions() {
        assert_eq!(UtilisationGauge::calculate_utilisation(0, 1_000_000, 1_000_000), 100.0);
    }

    #[test]
    fn utilisation_below_fifty_returns_info() {
        assert_eq!(UtilisationGauge::gauge_color(0.0), Color::Rgb(124, 170, 131));
        assert_eq!(UtilisationGauge::gauge_color(49.9), Color::Rgb(124, 170, 131));
    }

    #[test]
    fn utilisation_at_fifty_returns_warning() {
        assert_eq!(UtilisationGauge::gauge_color(50.0), Color::Rgb(240, 217, 168));
    }

    #[test]
    fn utilisation_below_eighty_returns_warning() {
        assert_eq!(UtilisationGauge::gauge_color(79.9), Color::Rgb(240, 217, 168));
    }

    #[test]
    fn utilisation_at_eighty_returns_danger() {
        assert_eq!(UtilisationGauge::gauge_color(80.0), Color::Rgb(243, 139, 168));
    }

    #[test]
    fn utilisation_above_eighty_returns_danger() {
        assert_eq!(UtilisationGauge::gauge_color(95.0), Color::Rgb(243, 139, 168));
    }

    #[test]
    fn utilisation_at_hundred_returns_danger() {
        assert_eq!(UtilisationGauge::gauge_color(100.0), Color::Rgb(243, 139, 168));
    }
}
