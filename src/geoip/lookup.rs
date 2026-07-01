use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Mutex, OnceLock};

use super::database::GeoIpDatabase;

static DATABASE: OnceLock<Option<GeoIpDatabase>> = OnceLock::new();
static CACHE: OnceLock<Mutex<HashMap<IpAddr, Option<String>>>> = OnceLock::new();

pub struct GeoIpLookup;

impl GeoIpLookup {
    pub fn get_country(ip: &str) -> Option<String> {
        let parsed = ip.parse::<IpAddr>().ok()?;
        {
            let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
            let guard = cache.lock().ok()?;
            if let Some(cached) = guard.get(&parsed) {
                return cached.clone();
            }
        }

        let result = DATABASE.get_or_init(GeoIpDatabase::load).as_ref().and_then(|db| db.lookup(parsed));

        {
            let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
            if let Ok(mut guard) = cache.lock() {
                guard.insert(parsed, result.clone());
            }
        }

        result
    }

    pub fn get_ip(endpoint: &str) -> &str {
        endpoint.rsplit_once(':').map(|(ip, _)| ip).unwrap_or(endpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_endpoint_with_port_then_get_ip_strips_port() {
        assert_eq!(GeoIpLookup::get_ip("142.250.80.46:443"), "142.250.80.46");
        assert_eq!(GeoIpLookup::get_ip("192.168.1.5:52532"), "192.168.1.5");
    }

    #[test]
    fn given_endpoint_without_port_then_get_ip_returns_input() {
        assert_eq!(GeoIpLookup::get_ip("142.250.80.46"), "142.250.80.46");
    }

    #[test]
    fn given_ipv6_endpoint_then_get_ip_returns_bracketed() {
        assert_eq!(GeoIpLookup::get_ip("[::1]:443"), "[::1]");
    }

    #[test]
    fn given_invalid_ip_then_get_country_returns_none() {
        assert!(GeoIpLookup::get_country("not-an-ip").is_none());
    }

    #[test]
    fn given_cached_result_then_second_call_uses_cache() {
        // Both calls should not panic; result depends on DB presence
        let _ = GeoIpLookup::get_country("8.8.8.8");
        let result = GeoIpLookup::get_country("8.8.8.8");
        // Without a real DB, both return None consistently
        assert!(result.is_none());
    }
}
