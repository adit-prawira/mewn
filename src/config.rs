use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
pub struct ColorConfig {
    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub text: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub text_highlight: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub text_dim: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub border: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub indicator: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub selected: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub highlight: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub upload_rate: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub download_rate: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub tcp: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub udp: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub udp_secondary: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub dns: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub icmp: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub cpu: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub ram: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub info: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub warning: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub danger: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub source_address: Option<Color>,

    #[serde(default, deserialize_with = "Config::deserialize_hex_color")]
    pub destination_address: Option<Color>,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_poll_interval")]
    pub poll_interval: u64,

    #[serde(default)]
    pub upload_threshold_mbps: Option<u64>,

    #[serde(default)]
    pub download_threshold_mbps: Option<u64>,

    #[serde(default)]
    pub interface: Option<String>,

    #[serde(default)]
    pub ip2location_license_key: Option<String>,

    #[serde(default)]
    pub colors: ColorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: 1,
            upload_threshold_mbps: None,
            download_threshold_mbps: None,
            interface: None,
            ip2location_license_key: None,
            colors: ColorConfig::default(),
        }
    }
}

impl Config {
    pub fn directory() -> PathBuf {
        Self::config_path().parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."))
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };

        match toml::from_str::<Self>(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("mewn: failed to parse config at {}: {}", path.display(), e);
                Self::default()
            }
        }
    }

    pub fn init() -> Result<()> {
        let config_directory = Self::config_path().parent().unwrap().to_path_buf();
        fs::create_dir_all(&config_directory).with_context(|| format!("failed to create config directory: {}", config_directory.display()))?;

        let path = Self::config_path();
        if path.exists() {
            eprintln!("mewn: config already exists at {}", path.display());
            return Ok(());
        }

        fs::write(&path, Self::template_content()).with_context(|| format!("failed to write config to {}", path.display()))?;

        println!("mewn: created config at {}", path.display());
        Ok(())
    }

    fn config_path() -> PathBuf {
        let home = std::env::var("HOME").ok().map(PathBuf::from);
        home.unwrap_or_else(|| PathBuf::from(".")).join(".config").join("mewn").join("config.toml")
    }

    fn default_poll_interval() -> u64 {
        1
    }

    fn deserialize_hex_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let hex: Option<String> = Option::deserialize(deserializer)?;
        let Some(hex) = hex else {
            return Ok(None);
        };
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(serde::de::Error::custom("hex color must be 6 chars, e.g. #BAC4EE"));
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(serde::de::Error::custom)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(serde::de::Error::custom)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(serde::de::Error::custom)?;
        Ok(Some(Color::Rgb(r, g, b)))
    }

    fn template_content() -> String {
        r##"# ~/.config/mewn/config.toml — all keys optional, defaults shown
            #
            # poll_interval = 1           # seconds between data refreshes
            # upload_threshold_mbps = 10 # flash row red when upload exceeds N mbps 
            # download_threshold_mbps = 50 # flash row red when download exceeds N mbps
            # interface = "en0"           # network interface override
            # ip2location_license_key = "" # free key from https://lite.ip2location.com (used by `mewn geoip-update`) 

            [colors]
            # text = "#BAC4EE"
            # text_highlight = "#FFFFFF"
            # text_dim = "#6E7198"
            # border = "#CBA6F7"
            # indicator = "#F5C2E7"
            # selected = "#94E2D5"
            # highlight = "#A6E3A1"
            # upload_rate = "#A6E3A1"
            # download_rate = "#F9E2AF"
            # tcp = "#A6E3A1"
            # udp = "#89B4FA"
            # udp_secondary = "#B4BEFE"
            # dns = "#F9E2AF"
            # icmp = "#F38BA8"
            # cpu = "#89DCEB"
            # ram = "#CBA6F7"
            # info = "#A6E3A1"
            # warning = "#F9E2AF"
            # danger = "#F38BA8"
            # source_address = "#F9E2AF"
            # destination_address = "#E6A0C4"
        "##
        .lines()
        .map(str::trim_start)
        .collect::<Vec<_>>()
        .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_poll_interval_is_one() {
        assert_eq!(Config::default_poll_interval(), 1);
    }

    #[test]
    fn default_interface_is_none() {
        let config = Config::default();
        assert!(config.interface.is_none());
    }

    #[test]
    fn default_colors_all_none() {
        let config = Config::default();
        assert!(config.colors.text.is_none());
        assert!(config.colors.text_highlight.is_none());
        assert!(config.colors.text_dim.is_none());
        assert!(config.colors.border.is_none());
        assert!(config.colors.indicator.is_none());
        assert!(config.colors.selected.is_none());
        assert!(config.colors.highlight.is_none());
        assert!(config.colors.upload_rate.is_none());
        assert!(config.colors.download_rate.is_none());
        assert!(config.colors.tcp.is_none());
        assert!(config.colors.udp.is_none());
        assert!(config.colors.udp_secondary.is_none());
        assert!(config.colors.dns.is_none());
        assert!(config.colors.icmp.is_none());
        assert!(config.colors.cpu.is_none());
        assert!(config.colors.ram.is_none());
        assert!(config.colors.info.is_none());
        assert!(config.colors.warning.is_none());
        assert!(config.colors.danger.is_none());
        assert!(config.colors.source_address.is_none());
        assert!(config.colors.destination_address.is_none());
    }

    #[test]
    fn parses_poll_interval() {
        let toml = r#"
            poll_interval = 5
            "#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.poll_interval, 5);
        assert!(config.interface.is_none());
    }

    #[test]
    fn parses_interface() {
        let toml = r#"
            interface = "en0"
            "#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.interface.as_deref(), Some("en0"));
        assert_eq!(config.poll_interval, 1);
    }

    #[test]
    fn parses_semantic_colors() {
        let toml = r##"
            [colors]
            tcp = "#7CAA83"
            udp = "#89B4FA"
            dns = "#F0D9A8"
            icmp = "#F38BA8"
            info = "#7CAA83"
            danger = "#F38BA8"
            "##;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.colors.tcp, Some(Color::Rgb(124, 170, 131)));
        assert_eq!(config.colors.udp, Some(Color::Rgb(137, 180, 250)));
        assert_eq!(config.colors.dns, Some(Color::Rgb(240, 217, 168)));
        assert_eq!(config.colors.icmp, Some(Color::Rgb(243, 139, 168)));
        assert_eq!(config.colors.info, Some(Color::Rgb(124, 170, 131)));
        assert_eq!(config.colors.danger, Some(Color::Rgb(243, 139, 168)));
        assert!(config.colors.text.is_none());
    }

    #[test]
    fn empty_toml_uses_defaults() {
        let toml = "";
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.poll_interval, 1);
        assert!(config.interface.is_none());
        assert!(config.colors.tcp.is_none());
    }

    #[test]
    fn partial_toml_merges_with_defaults() {
        let toml = r##"
            poll_interval = 3

            [colors]
            tcp = "#7CAA83"
            "##;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.poll_interval, 3);
        assert_eq!(config.colors.tcp, Some(Color::Rgb(124, 170, 131)));
        assert!(config.colors.udp.is_none());
        assert!(config.interface.is_none());
    }

    #[test]
    fn parses_hex_color_without_hash_prefix() {
        let toml = r##"
            [colors]
            tcp = "7CAA83"
            "##;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.colors.tcp, Some(Color::Rgb(124, 170, 131)));
    }

    #[test]
    fn rejects_invalid_hex_chars() {
        let toml = r##"
            [colors]
            tcp = "#GGGGGG"
            "##;
        let result = toml::from_str::<Config>(toml);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_hex_length() {
        let toml_short = r##"
            [colors]
            tcp = "#7CAA"
            "##;
        assert!(toml::from_str::<Config>(toml_short).is_err());

        let toml_long = r##"
            [colors]
            tcp = "#7CAA8300"
            "##;
        assert!(toml::from_str::<Config>(toml_long).is_err());
    }
}
