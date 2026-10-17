use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use validator::ValidationErrors;

#[derive(Serialize, Debug)]
pub struct AppErrorData {
    pub code: String,
    pub message: String,
    pub errors: Option<HashMap<String, Vec<String>>>,
    pub is_error: bool,
}

impl Default for AppErrorData {
    fn default() -> Self {
        Self::new()
    }
}

impl AppErrorData {
    pub fn new() -> Self {
        Self {
            is_error: true,
            code: "INTERNAL_SERVER_ERROR".to_string(),
            message: "An internal server error occurred".to_string(),
            errors: None,
        }
    }

    pub fn code(mut self, code: String) -> Self {
        self.code = code;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn errors(mut self, errors: HashMap<String, Vec<String>>) -> Self {
        self.errors = Some(errors);
        self
    }

    pub fn validation_error(mut self, errors: ValidationErrors) -> Self {
        let mut err_map: HashMap<String, Vec<String>> = HashMap::new();

        for (field, field_errors) in errors.field_errors() {
            let messages: Vec<String> = field_errors
                .iter()
                .map(|e| {
                    // Use custom message if provided, otherwise default to the rule code (e.g., "email")
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| e.code.to_string())
                })
                .collect();

            err_map.insert(field.to_string(), messages);
        }
        self.errors = Some(err_map);
        self
    }

    pub fn to_json(self) -> serde_json::Value {
        json!(self)
    }
}
