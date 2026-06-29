use serde::Serialize;

pub struct Serializer;

impl Serializer {
    pub fn json<T: Serialize>(data: &[T]) -> String {
        serde_json::to_string_pretty(data).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn csv<T>(data: &[T], header: &str, row_fn: fn(&T) -> String) -> String {
        let mut csv = String::from(header);
        for item in data {
            csv.push_str(&row_fn(item));
            csv.push('\n');
        }
        csv
    }
}
