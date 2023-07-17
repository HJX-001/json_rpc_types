use serde::{Deserialize, Serialize};

use crate::Version2;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request<P> {
    pub jsonrpc: Version2,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl<P> Request<P> {
    pub const fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}
