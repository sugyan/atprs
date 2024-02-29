// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.updateAccountPassword` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub did: crate::types::string::Did,
    pub password: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
