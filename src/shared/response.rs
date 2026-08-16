use axum::{
    Json,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode, header::InvalidHeaderValue},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub msg: String,
    pub data: Option<T>,

    #[serde(skip)]
    pub status_code: StatusCode,

    #[serde(skip)]
    pub headers: HeaderMap,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new() -> Self {
        Self {
            msg: String::new(),
            data: None,
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }
    pub fn message(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }

    pub fn body(mut self, data: Option<T>) -> Self {
        self.data = data;
        self
    }

    pub fn header(mut self, key: HeaderName, value: &str) -> Self {
        self.headers.insert(
            key,
            HeaderValue::from_str(value).expect("must be a valid header value"),
        );
        self
    }
    pub fn try_header(mut self, key: HeaderName, value: &str) -> Result<Self, InvalidHeaderValue> {
        self.headers.insert(key, HeaderValue::from_str(value)?);
        Ok(self)
    }

    pub fn headers(mut self, header_map: HeaderMap) -> Self {
        self.headers.extend(header_map);
        self
    }
}

impl<T: Serialize> Default for ApiResponse<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = self.status_code;
        let body = Json(json!({
          "message": self.msg,
          "data": self.data
        }));
        let mut res = (status, body).into_response();
        res.headers_mut().extend(self.headers);
        res
    }
}
