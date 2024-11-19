// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.sendEmail` namespace.
pub const NSID: &str = "com.atproto.admin.sendEmail";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///Additional comment by the sender that won't be used in the email itself but helpful to provide more context for moderators/reviewers
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub comment: core::option::Option<String>,
    pub content: String,
    pub recipient_did: crate::types::string::Did,
    pub sender_did: crate::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub subject: core::option::Option<String>,
}
pub type Input = crate::types::Object<InputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub sent: bool,
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
