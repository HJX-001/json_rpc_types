use serde::{Deserialize, Serialize};

use crate::{error::Error, Version2};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response<R> {
    Result {
        jsonrpc: Version2,
        result: R,
        id: Option<String>,
    },

    Error {
        jsonrpc: Version2,
        error: Error,
        id: Option<String>,
    },
}
