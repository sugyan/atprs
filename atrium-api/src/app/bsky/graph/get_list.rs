// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.graph.getList` namespace.
pub const NSID: &str = "app.bsky.graph.getList";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub limit: core::option::Option<crate::types::LimitedNonZeroU8<100u8>>,
    ///Reference (AT-URI) of the list record to hydrate.
    pub list: String,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    pub items: Vec<crate::app::bsky::graph::defs::ListItemView>,
    pub list: crate::app::bsky::graph::defs::ListView,
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
