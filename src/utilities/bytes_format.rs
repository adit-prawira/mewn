const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;

pub struct BytesFormat;

impl BytesFormat {
    pub fn format_bytes(bytes: f64) -> String {
        Self::format_bytes_with_suffix(bytes, "")
    }

    pub fn format_bytes_per_seconds(bytes_per_seconds: f64) -> String {
        Self::format_bytes_with_suffix(bytes_per_seconds, "/s")
    }

    fn format_bytes_with_suffix(bytes: f64, suffix: &str) -> String {
        if bytes >= GB as f64 {
            let gigabytes = bytes / GB as f64;
            format!("{:.2} GB{}", gigabytes, suffix)
        } else if bytes >= MB as f64 {
            let megabytes = bytes / MB as f64;
            format!("{:.2} MB{}", megabytes, suffix)
        } else if bytes >= KB as f64 {
            let kilobytes = bytes / KB as f64;
            format!("{:.2} KB{}", kilobytes, suffix)
        } else {
            format!("{:.2} B{}", bytes, suffix)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_bytes_then_format_bytes_returns_zero_b() {
        assert_eq!(BytesFormat::format_bytes(0.0), "0.00 B");
    }

    #[test]
    fn given_bytes_under_1kb_then_format_bytes_returns_b() {
        assert_eq!(BytesFormat::format_bytes(512.0), "512.00 B");
    }

    #[test]
    fn given_exactly_1kb_then_format_bytes_returns_kb() {
        assert_eq!(BytesFormat::format_bytes(1024.0), "1.00 KB");
    }

    #[test]
    fn given_one_and_half_kb_then_format_bytes_returns_kb() {
        assert_eq!(BytesFormat::format_bytes(1536.0), "1.50 KB");
    }

    #[test]
    fn given_1mb_then_format_bytes_returns_mb() {
        assert_eq!(BytesFormat::format_bytes(1048576.0), "1.00 MB");
    }

    #[test]
    fn given_1gb_then_format_bytes_returns_gb() {
        assert_eq!(BytesFormat::format_bytes(1073741824.0), "1.00 GB");
    }

    #[test]
    fn given_bytes_per_second_then_suffix_is_per_s() {
        assert_eq!(BytesFormat::format_bytes_per_seconds(0.0), "0.00 B/s");
        assert_eq!(BytesFormat::format_bytes_per_seconds(1024.0), "1.00 KB/s");
        assert_eq!(BytesFormat::format_bytes_per_seconds(1048576.0), "1.00 MB/s");
    }
}
