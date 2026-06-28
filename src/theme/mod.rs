use std::sync::OnceLock;

use ratatui::style::Color;

use crate::config::ColorConfig;

static TEXT: OnceLock<Color> = OnceLock::new();
static TEXT_HIGHLIGHT: OnceLock<Color> = OnceLock::new();
static TEXT_DIM: OnceLock<Color> = OnceLock::new();
static INDICATOR: OnceLock<Color> = OnceLock::new();
static SELECTED: OnceLock<Color> = OnceLock::new();
static BORDER: OnceLock<Color> = OnceLock::new();
static HIGHLIGHT: OnceLock<Color> = OnceLock::new();
static UPLOAD_RATE: OnceLock<Color> = OnceLock::new();
static DOWNLOAD_RATE: OnceLock<Color> = OnceLock::new();
static TCP: OnceLock<Color> = OnceLock::new();
static UDP: OnceLock<Color> = OnceLock::new();
static UDP_SECONDARY: OnceLock<Color> = OnceLock::new();
static DNS: OnceLock<Color> = OnceLock::new();
static ICMP: OnceLock<Color> = OnceLock::new();
static CPU: OnceLock<Color> = OnceLock::new();
static RAM: OnceLock<Color> = OnceLock::new();
static WARNING: OnceLock<Color> = OnceLock::new();
static SOURCE_ADDRESS: OnceLock<Color> = OnceLock::new();
static DESTINATION_ADDRESS: OnceLock<Color> = OnceLock::new();

pub struct Theme;

impl Theme {
    pub fn text() -> Color {
        *TEXT.get_or_init(|| Color::Rgb(186, 196, 238))
    }

    pub fn text_highlight() -> Color {
        *TEXT_HIGHLIGHT.get_or_init(|| Color::Rgb(124, 170, 131))
    }

    pub fn text_dim() -> Color {
        *TEXT_DIM.get_or_init(|| Color::Rgb(156, 164, 201))
    }

    pub fn indicator() -> Color {
        *INDICATOR.get_or_init(|| Color::Gray)
    }

    pub fn selected() -> Color {
        *SELECTED.get_or_init(|| Color::Rgb(132, 75, 92))
    }

    pub fn border() -> Color {
        *BORDER.get_or_init(|| Color::Rgb(137, 180, 250))
    }

    pub fn highlight() -> Color {
        *HIGHLIGHT.get_or_init(|| Color::Rgb(124, 170, 131))
    }

    pub fn upload_rate() -> Color {
        *UPLOAD_RATE.get_or_init(|| Color::Rgb(124, 170, 131))
    }

    pub fn download_rate() -> Color {
        *DOWNLOAD_RATE.get_or_init(|| Color::Rgb(240, 217, 168))
    }

    pub fn tcp() -> Color {
        *TCP.get_or_init(|| Color::Rgb(124, 170, 131))
    }

    pub fn udp() -> Color {
        *UDP.get_or_init(|| Color::Rgb(137, 180, 250))
    }

    pub fn udp_secondary() -> Color {
        *UDP_SECONDARY.get_or_init(|| Color::Rgb(240, 217, 168))
    }

    pub fn dns() -> Color {
        *DNS.get_or_init(|| Color::Rgb(240, 217, 168))
    }

    pub fn icmp() -> Color {
        *ICMP.get_or_init(|| Color::Rgb(243, 139, 168))
    }

    pub fn cpu() -> Color {
        *CPU.get_or_init(|| Color::Cyan)
    }

    pub fn ram() -> Color {
        *RAM.get_or_init(|| Color::Magenta)
    }

    pub fn warning() -> Color {
        *WARNING.get_or_init(|| Color::Rgb(240, 217, 168))
    }

    pub fn source_address() -> Color {
        *SOURCE_ADDRESS.get_or_init(|| Color::Rgb(240, 217, 168))
    }

    pub fn destination_address() -> Color {
        *DESTINATION_ADDRESS.get_or_init(|| Color::Rgb(173, 132, 105))
    }

    pub fn override_color(color_config: &ColorConfig) {
        Self::set_color(&TEXT, color_config.text);
        Self::set_color(&TEXT_HIGHLIGHT, color_config.text_highlight);
        Self::set_color(&TEXT_DIM, color_config.text_dim);
        Self::set_color(&BORDER, color_config.border);
        Self::set_color(&INDICATOR, color_config.indicator);
        Self::set_color(&SELECTED, color_config.selected);
        Self::set_color(&HIGHLIGHT, color_config.highlight);
        Self::set_color(&UPLOAD_RATE, color_config.upload_rate);
        Self::set_color(&DOWNLOAD_RATE, color_config.download_rate);
        Self::set_color(&TCP, color_config.tcp);
        Self::set_color(&UDP, color_config.udp);
        Self::set_color(&UDP_SECONDARY, color_config.udp_secondary);
        Self::set_color(&DNS, color_config.dns);
        Self::set_color(&RAM, color_config.ram);
        Self::set_color(&WARNING, color_config.warning);
        Self::set_color(&ICMP, color_config.icmp);
        Self::set_color(&CPU, color_config.cpu);
        Self::set_color(&SOURCE_ADDRESS, color_config.source_address);
        Self::set_color(&DESTINATION_ADDRESS, color_config.destination_address);
    }

    fn set_color(cell: &OnceLock<Color>, value: Option<Color>) {
        if let Some(color) = value {
            let _ = cell.set(color);
        }
    }
}
