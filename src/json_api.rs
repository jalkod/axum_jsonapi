use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

/// Generic JSON API response struct
#[derive(Serialize, Deserialize, Default)]
#[allow(non_camel_case_types)]
pub struct Output<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) errors: Vec<JsonError>,
}

impl<T> Output<T>
where
    T: Serialize,
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data: None,
            errors: Vec::new(),
        }
    }

    pub fn add(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn respond(self) -> impl IntoResponse {
        self.respond_with_code(StatusCode::OK)
    }

    pub fn respond_with_code(self, mut status_code: StatusCode) -> impl IntoResponse {
        if !self.errors.is_empty() {
            let is_internal_error = self.errors.iter().any(|x| x.status >= 500);
            status_code = if is_internal_error {
                StatusCode::INTERNAL_SERVER_ERROR
            } else {
                StatusCode::BAD_REQUEST
            };
        }
        (
            status_code,
            [(header::CONTENT_TYPE, "application/vnd.api+json")],
            Json(self),
        )
    }

    pub fn add_error(&mut self, status_code: StatusCode) -> &mut Self {
        let title = status_code
            .canonical_reason()
            .map_or_else(String::new, std::string::ToString::to_string);
        self.errors.push(JsonError {
            status: status_code.as_u16(),
            title,
            source: None,
            detail: None,
        });
        self
    }

    pub fn err_source(&mut self, source: String) -> &mut Self {
        let len = self.errors.len();
        if len > 0 {
            self.errors[len - 1].source = Some(source);
        }
        self
    }
    pub fn err_title(&mut self, title: String) -> &mut Self {
        let len = self.errors.len();
        if len > 0 {
            self.errors[len - 1].title = title;
        }
        self
    }

    pub fn err_detail(&mut self, detail: String) -> &mut Self {
        let len = self.errors.len();
        if len > 0 {
            self.errors[len - 1].detail = Some(detail);
        }
        self
    }
}

/// Generic JSON API error response struct
#[derive(Serialize, Deserialize)]
pub struct JsonError {
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[cfg(test)]
mod test_json_api {
    use super::*;

    #[test]
    fn test_error_builder() {
        let mut builder = Output::<u8>::new();
        builder
            .add_error(StatusCode::CONFLICT)
            .err_source("test_error_builder".into())
            .err_detail("detail".into());

        assert_eq!(builder.data, None);
        assert_eq!(builder.errors.len(), 1);
        let errors = &builder.errors[0];
        assert_eq!(errors.detail, Some("detail".into()));
        assert_eq!(errors.source, Some("test_error_builder".into()));
        assert_eq!(errors.status, 409);
        assert_eq!(errors.title, "Conflict");
    }
}
