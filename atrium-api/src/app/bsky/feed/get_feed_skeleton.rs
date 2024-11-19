// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.getFeedSkeleton` namespace.
pub const NSID: &str = "app.bsky.feed.getFeedSkeleton";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    ///Reference to feed generator record describing the specific feed being requested.
    pub feed: String,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub limit: core::option::Option<crate::types::LimitedNonZeroU8<100u8>>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    pub feed: Vec<crate::app::bsky::feed::defs::SkeletonFeedPost>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    UnknownFeed(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownFeed(msg) => {
                write!(_f, "UnknownFeed")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
