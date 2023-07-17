use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    code: i64,
    message: String,
    #[serde(skip_serializing_if = "Value::is_null")]
    #[serde(default)]
    data: Value,
}

impl Error {
    pub fn parse_error() -> Self {
        Self {
            code: -32700,
            message: "Parse error".to_owned(),
            data: Value::Null,
        }
    }

    pub fn invalid_request() -> Self {
        Self {
            code: -32600,
            message: "Parse error".to_owned(),
            data: Value::Null,
        }
    }

    pub fn method_not_found() -> Self {
        Self {
            code: -32601,
            message: "Parse error".to_owned(),
            data: Value::Null,
        }
    }

    pub fn invalid_params() -> Self {
        Self {
            code: -32602,
            message: "Parse error".to_owned(),
            data: Value::Null,
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: -32603,
            message: "Parse error".to_owned(),
            data: Value::Null,
        }
    }

    /// panic on invalid server code (-32099..=-32000)
    pub fn server_error(code: i64) -> Self {
        match code {
            -32099..=-32000 => Error {
                code,
                message: "Server error".to_owned(),
                data: Value::Null,
            },
            _ => panic!("No matching error found for given code"),
        }
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.data = data;
        self
    }
}

impl From<i64> for Error {
    fn from(code: i64) -> Self {
        match code {
            -32700 => Error::parse_error(),
            -32600 => Error::invalid_request(),
            -32601 => Error::method_not_found(),
            -32602 => Error::invalid_params(),
            -32603 => Error::internal_error(),
            _ => Error::server_error(code),
        }
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.code, self.message, self.data)
    }
}
