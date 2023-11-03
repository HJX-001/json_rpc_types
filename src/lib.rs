pub mod types;

// re exports
pub use types::Error;
pub use types::ErrorCode;
pub use types::Request;
pub use types::Response;
pub use types::Version2;

#[cfg(test)]
mod tests {

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
        let error = Error::from(ErrorCode::ServerErrorEnd);
        let error_str = r#"{"code":-32000,"message":"server error end"}"#;

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
            error: Error::from(ErrorCode::ServerErrorEnd),
            id: Some("id".to_string()),
        };

        let result_res_str = r#"{"jsonrpc":"2.0","result":"result","id":"id"}"#;
        let error_res_str =
            r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"server error end"},"id":"id"}"#;

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
