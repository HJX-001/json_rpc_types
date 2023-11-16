use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct Version2;

impl Serialize for Version2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("2.0")
    }
}

impl<'de> Deserialize<'de> for Version2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let de: String = Deserialize::deserialize(deserializer)?;
        if &de == "2.0" {
            Ok(Version2)
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&de),
                &"2.0",
            ))
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ErrorCode {
    // Predefined by JSON RPC:
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
    ServerErrorStart = -32099,
    ServerErrorEnd = -32000,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str = match self {
            ErrorCode::ParseError => "parse error",
            ErrorCode::InvalidRequest => "invalid request",
            ErrorCode::MethodNotFound => "method not found",
            ErrorCode::InvalidParams => "invalid params",
            ErrorCode::InternalError => "internal error",
            ErrorCode::ServerErrorStart => "server error start",
            ErrorCode::ServerErrorEnd => "server error end",
        };

        f.write_str(self_str)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Value::is_null")]
    #[serde(default)]
    data: Value,
}

impl From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        Self::new(code as i32, &code.to_string())
    }
}

impl Error {
    // not recommended, use from ErrorCode instead
    // msg only set when code is not in predefined json rpc error set
    pub fn new(code: i32, msg: &str) -> Self {
        let message = match code {
            -32700 => "parse error",
            -32600 => "invalid request",
            -32601 => "method not found",
            -32602 => "invalid params",
            -32603 => "internal error",
            -32099 => "server error start",
            -32000 => "server error end",
            _ => msg,
        };

        Self {
            code,
            message: message.to_owned(),
            data: Value::Null,
        }
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.data = data;
        self
    }
}

// in any case reqd
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.code, self.message, self.data)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub jsonrpc: Version2,
    pub method: String,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub params: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Request {
    pub const fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Result {
        jsonrpc: Version2,
        result: Value,
        id: Option<String>,
    },

    Error {
        jsonrpc: Version2,
        error: Error,
        id: Option<String>,
    },
}
