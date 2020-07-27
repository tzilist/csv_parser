use std::fmt;

use actix_http::ResponseBuilder;
use actix_web::{
    http::{header, StatusCode},
    HttpResponse, ResponseError,
};
use serde::Serialize;
use thiserror::Error;

/// General error message, all errors should eventually convert to this
#[derive(Serialize, Debug, Clone)]
pub(crate) struct ErrorMessage {
    msg: String,
}

/// Error types for server and controllers
#[derive(Debug, Error)]
pub(crate) enum ServerErrors {
    CsvParseFailure {
        #[from]
        source: csv::Error,
    },
}

impl<'a> From<&'a ServerErrors> for ErrorMessage {
    fn from(e: &ServerErrors) -> Self {
        Self { msg: e.to_string() }
    }
}

impl fmt::Display for ServerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl ResponseError for ServerErrors {
    fn error_response(&self) -> HttpResponse {
        let msg: ErrorMessage = self.into();

        let body = serde_json::to_string(&msg).unwrap();

        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .body(body)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
