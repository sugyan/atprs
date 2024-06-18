// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `tools.ozone.server.getConfig` namespace.
pub const NSID: &str = "tools.ozone.server.getConfig";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appview: Option<ServiceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_divert: Option<ServiceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<ServiceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pds: Option<ServiceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerConfig>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}