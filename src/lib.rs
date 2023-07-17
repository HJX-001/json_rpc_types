pub mod error;
pub mod request;
pub mod response;

use serde::{Deserialize, Serialize};

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

// re exports
pub use error::Error;
pub use request::Request;
pub use response::Response;

#[cfg(test)]
mod tests {

    use crate::{error::Error, request::Request, response::Response};

    use super::*;

    #[test]
    fn version() {
        assert_eq!(&serde_json::to_string(&Version2).unwrap(), "\"2.0\"");
        assert_eq!(
            serde_json::from_str::<Version2>("\"2.0\"").unwrap(),
            Version2
        );
    }

    #[test]
    fn request() {
        let req = Request {
            jsonrpc: Version2,
            method: "method".to_string(),
            params: Some("params"),
            id: Some("id".to_string()),
        };

        let req_str = r#"{"jsonrpc":"2.0","method":"method","params":"params","id":"id"}"#;

        assert_eq!(&serde_json::to_string(&req).unwrap(), req_str);
        assert_eq!(serde_json::from_str::<Request<_>>(req_str).unwrap(), req);
    }

    #[test]
    fn error() {
        let error = Error::from(-32000);
        let error_str = r#"{"code":-32000,"message":"Server error"}"#;

        assert_eq!(&serde_json::to_string(&error).unwrap(), error_str);
        assert_eq!(serde_json::from_str::<Error>(error_str).unwrap(), error);
    }

    #[test]
    fn response() {
        let result_res = Response::Result {
            jsonrpc: crate::Version2,
            result: "result",
            id: Some("id".to_string()),
        };

        let error_res: Response<()> = Response::Error {
            jsonrpc: crate::Version2,
            error: Error::from(-32000),
            id: Some("id".to_string()),
        };

        let result_res_str = r#"{"jsonrpc":"2.0","result":"result","id":"id"}"#;
        let error_res_str =
            r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"Server error"},"id":"id"}"#;

        assert_eq!(&serde_json::to_string(&result_res).unwrap(), result_res_str);
        assert_eq!(
            serde_json::from_str::<Response<_>>(result_res_str).unwrap(),
            result_res
        );

        assert_eq!(&serde_json::to_string(&error_res).unwrap(), error_res_str);
        assert_eq!(
            serde_json::from_str::<Response<_>>(error_res_str).unwrap(),
            error_res
        );
    }
}
