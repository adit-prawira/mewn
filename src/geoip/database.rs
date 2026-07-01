use crate::config::Config;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

use super::resource::IpRange;

const DATABASE_FILENAME: &str = "IP2LOCATION-LITE-DB1.CSV";

pub struct GeoIpDatabase {
    pub ip_ranges: Vec<IpRange>,
}

impl GeoIpDatabase {
    pub fn load() -> Option<Self> {
        let path = Self::database_path()?;
        let content = fs::read_to_string(&path).ok()?;
        let ip_ranges = Self::parse_csv(&content);
        if ip_ranges.is_empty() {
            return None;
        }
        Some(Self { ip_ranges })
    }

    pub fn lookup(&self, ip: IpAddr) -> Option<String> {
        let ipv4 = match ip {
            IpAddr::V4(ipv4_addr) => ipv4_addr,
            IpAddr::V6(_) => return None,
        };
        let target = ipv4.to_bits();
        let index = self.ip_ranges.partition_point(|range| range.start <= target).saturating_sub(1);
        let candidate = self.ip_ranges.get(index)?;
        let in_range = target >= candidate.start && target <= candidate.end;
        let is_reserved = candidate.code == "ZZ";
        let has_country = in_range && !is_reserved;

        if has_country { Some(candidate.code.clone()) } else { None }
    }

    fn database_path() -> Option<PathBuf> {
        let directory = Config::directory();
        let path = directory.join(DATABASE_FILENAME);
        path.exists().then_some(path)
    }

    fn parse_csv(content: &str) -> Vec<IpRange> {
        let mut ranges = Vec::new();
        let trimmed_content = content.lines().filter(|line| !line.trim().is_empty());

        for line in trimmed_content {
            let Some(ip_range) = Self::parse_line(line) else {
                continue;
            };
            ranges.push(ip_range);
        }
        ranges
    }

    fn parse_line(line: &str) -> Option<IpRange> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for character in line.chars() {
            match character {
                '"' => in_quotes = !in_quotes,
                ',' if !in_quotes => fields.push(std::mem::take(&mut current)),
                _ => current.push(character),
            };
        }

        fields.push(current);

        if fields.len() < 3 {
            return None;
        }
        let start: Ipv4Addr = fields[0].parse().ok()?;
        let end: Ipv4Addr = fields[1].parse().ok()?;
        let code = fields[2].to_string();

        Some(IpRange {
            start: start.to_bits(),
            end: end.to_bits(),
            code,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_csv_line_then_parse_line_returns_ip_range() {
        let result = GeoIpDatabase::parse_line("\"1.0.0.0\",\"1.0.0.255\",\"AU\",\"Australia\"");
        assert!(result.is_some());
        let ip_range = result.unwrap();
        assert_eq!(ip_range.code, "AU");
        assert_eq!(ip_range.start, Ipv4Addr::new(1, 0, 0, 0).to_bits());
        assert_eq!(ip_range.end, Ipv4Addr::new(1, 0, 0, 255).to_bits());
    }

    #[test]
    fn given_csv_line_too_short_then_parse_line_returns_none() {
        let result = GeoIpDatabase::parse_line("\"1.0.0.0\",\"1.0.0.255\"");
        assert!(result.is_none());
    }

    #[test]
    fn given_valid_csv_then_parse_csv_returns_ranges() {
        let csv = "\"1.0.0.0\",\"1.0.0.255\",\"AU\",\"Australia\"\n\"1.0.1.0\",\"1.0.3.255\",\"CN\",\"China\"";
        let ranges = GeoIpDatabase::parse_csv(csv);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].code, "AU");
        assert_eq!(ranges[1].code, "CN");
    }

    #[test]
    fn given_empty_csv_then_parse_csv_returns_empty_vec() {
        let ranges = GeoIpDatabase::parse_csv("");
        assert!(ranges.is_empty());
    }

    #[test]
    fn given_ip_in_range_then_lookup_returns_code() {
        let csv = "\"1.0.0.0\",\"1.0.0.255\",\"AU\",\"Australia\"";
        let ranges = GeoIpDatabase::parse_csv(csv);
        let db = GeoIpDatabase { ip_ranges: ranges };
        let ip: IpAddr = Ipv4Addr::new(1, 0, 0, 50).into();
        assert_eq!(db.lookup(ip), Some("AU".into()));
    }

    #[test]
    fn given_reserved_code_then_lookup_returns_none() {
        let csv = "\"0.0.0.0\",\"0.0.255.255\",\"ZZ\",\"Reserved\"";
        let ranges = GeoIpDatabase::parse_csv(csv);
        let db = GeoIpDatabase { ip_ranges: ranges };
        let ip: IpAddr = Ipv4Addr::new(0, 0, 0, 1).into();
        assert_eq!(db.lookup(ip), None);
    }

    #[test]
    fn given_ipv6_then_lookup_returns_none() {
        let csv = "\"1.0.0.0\",\"1.0.0.255\",\"AU\",\"Australia\"";
        let ranges = GeoIpDatabase::parse_csv(csv);
        let db = GeoIpDatabase { ip_ranges: ranges };
        let ip: IpAddr = "::1".parse().unwrap();
        assert_eq!(db.lookup(ip), None);
    }
}
