use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogSearchQuery {
    pub q: String,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_search_query_deserialization() {
        let json = r#"{"q": "error", "page": 1}"#;
        let query: LogSearchQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.q, "error");
        assert_eq!(query.page, Some(1));
    }
}
