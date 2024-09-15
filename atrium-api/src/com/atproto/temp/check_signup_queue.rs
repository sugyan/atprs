// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.temp.checkSignupQueue` namespace.
pub const NSID: &str = "com.atproto.temp.checkSignupQueue";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub activated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_in_queue: Option<i64>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
