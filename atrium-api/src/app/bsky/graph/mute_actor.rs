// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.graph.muteActor` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub actor: crate::types::string::AtIdentifier,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
