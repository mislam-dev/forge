use serde::{Deserialize, Serialize};

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,

    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            per_page: default_per_page(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaginatedResponse<T> {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
    pub data: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u64, per_page: u64, total: u64) -> Self {
        let total_pages = if per_page == 0 {
            0
        } else {
            (total + per_page - 1) / per_page
        };
        Self {
            page,
            per_page,
            total,
            total_pages,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params_defaults() {
        let params: PaginationParams = serde_json::from_str("{}").unwrap();
        assert_eq!(params.page, 1);
        assert_eq!(params.per_page, 20);
    }

    #[test]
    fn test_pagination_params_custom() {
        let json = r#"{"page": 2, "per_page": 50}"#;
        let params: PaginationParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.page, 2);
        assert_eq!(params.per_page, 50);
    }

    #[test]
    fn test_paginated_response_calculation() {
        let data = vec!["item1", "item2"];
        let res = PaginatedResponse::new(data.clone(), 1, 10, 25);
        assert_eq!(res.page, 1);
        assert_eq!(res.per_page, 10);
        assert_eq!(res.total, 25);
        assert_eq!(res.total_pages, 3);
        assert_eq!(res.data, data);
    }

    #[test]
    fn test_paginated_response_exact_pages() {
        let data = vec![1, 2, 3];
        let res = PaginatedResponse::new(data, 1, 10, 30);
        assert_eq!(res.total_pages, 3);
    }
}
