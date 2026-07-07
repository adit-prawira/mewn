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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Item {
        name: String,
        count: u32,
    }

    #[test]
    fn given_empty_slice_when_jsonized_then_returns_empty_array() {
        let data: &[Item] = &[];
        assert_eq!(Serializer::json(data), "[]");
    }

    #[test]
    fn given_items_when_jsonized_then_returns_pretty_json() {
        let data = vec![Item { name: "foo".into(), count: 1 }];
        let json = Serializer::json(&data);
        assert!(json.contains("\"name\": \"foo\""));
        assert!(json.contains("\"count\": 1"));
    }

    #[test]
    fn given_no_rows_when_csvized_then_returns_header_only() {
        let data: &[Item] = &[];
        let csv = Serializer::csv(data, "name,count\n", |item| format!("{},{}", item.name, item.count));
        assert_eq!(csv, "name,count\n");
    }

    #[test]
    fn given_single_row_when_csvized_then_returns_header_and_row() {
        let data = vec![Item { name: "foo".into(), count: 42 }];
        let csv = Serializer::csv(&data, "name,count\n", |item| format!("{},{}", item.name, item.count));
        assert_eq!(csv, "name,count\nfoo,42\n");
    }

    #[test]
    fn given_multiple_rows_when_csvized_then_returns_all_rows() {
        let data = vec![Item { name: "a".into(), count: 1 }, Item { name: "b".into(), count: 2 }];
        let csv = Serializer::csv(&data, "H\n", |item| format!("{}:{}", item.name, item.count));
        assert_eq!(csv, "H\na:1\nb:2\n");
    }
}
