// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.identity.submitPlcOperation` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub operation: crate::records::Record,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            _ => {}
        }
        Ok(())
    }
}
