use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use ureq::{Error, get};
use zip::ZipArchive;

use crate::config::Config;

pub struct GeoIpManager;

const DOWNLOAD_URL: &str = "https://www.ip2location.com/download?token={key}&file=DB1LITE";
const CSV_FILENAME: &str = "IP2LOCATION-LITE-DB1.CSV";

impl GeoIpManager {
    pub fn download_database(license_key: &str) -> Result<PathBuf> {
        let url = DOWNLOAD_URL.replace("{key}", license_key);
        let mut response = get(&url).call().map_err(|err| match err {
            Error::StatusCode(code) if code == 401 || code == 403 => {
                anyhow!("download failed: HTTP {} - check your IP2Location license key", code)
            }
            _ => anyhow!("download failed: {}", err),
        })?;
        let mut bytes = Vec::new();
        let mut reader = response.body_mut().as_reader();
        reader.read_to_end(&mut bytes).context("failed to read download response")?;

        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).context("downloaded file is not a valid ZIP archive")?;
        let mut file = archive.by_name(CSV_FILENAME).context(format!("{} not found in downloaded archive", CSV_FILENAME))?;
        let config_directory = Config::directory();
        fs::create_dir_all(&config_directory).with_context(|| format!("failed to create config directory: {}", config_directory.display()))?;

        let target = config_directory.join(CSV_FILENAME);
        let mut output = fs::File::create(&target).with_context(|| format!("failed to create file: {}", target.display()))?;
        std::io::copy(&mut file, &mut output).context("failed to write CSV file")?;
        Ok(target)
    }

    pub fn resolve_license_key() -> Option<String> {
        let key = std::env::var("IP2LOCATION_LICENSE_KEY").ok()?;
        if !key.trim().is_empty() {
            return Some(key);
        }
        let config = Config::load();
        config.ip2location_license_key.filter(|key| !key.trim().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_url_template_then_substitution_works() {
        let url = DOWNLOAD_URL.replace("{key}", "abc123");
        assert_eq!(url, "https://www.ip2location.com/download?token=abc123&file=DB1LITE");
    }
}
