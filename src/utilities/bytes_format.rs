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
