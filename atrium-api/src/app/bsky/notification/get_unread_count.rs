// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.notification.getUnreadCount` namespace.
pub const NSID: &str = "app.bsky.notification.getUnreadCount";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub priority: core::option::Option<bool>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub seen_at: core::option::Option<crate::types::string::Datetime>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub count: i64,
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
