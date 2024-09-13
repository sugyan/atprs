// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.embed.record` namespace.
//!A representation of a record embedded in a Bluesky record (eg, a post). For example, a quote-post, or sharing a feed generator record.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MainData {
    pub record: crate::com::atproto::repo::strong_ref::Main,
}
pub type Main = crate::types::Object<MainData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewData {
    pub record: crate::types::Union<ViewRecordRefs>,
}
pub type View = crate::types::Object<ViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewBlockedData {
    pub author: crate::app::bsky::feed::defs::BlockedAuthor,
    pub blocked: bool,
    pub uri: String,
}
pub type ViewBlocked = crate::types::Object<ViewBlockedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewDetachedData {
    pub detached: bool,
    pub uri: String,
}
pub type ViewDetached = crate::types::Object<ViewDetachedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewNotFoundData {
    pub not_found: bool,
    pub uri: String,
}
pub type ViewNotFound = crate::types::Object<ViewNotFoundData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewRecordData {
    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub cid: crate::types::string::Cid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<crate::types::Union<ViewRecordEmbedsItem>>>,
    pub indexed_at: crate::types::string::Datetime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repost_count: Option<i64>,
    pub uri: String,
    ///The record data itself.
    pub value: crate::types::Unknown,
}
pub type ViewRecord = crate::types::Object<ViewRecordData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ViewRecordEmbedsItem {
    #[serde(rename = "app.bsky.embed.images#view")]
    AppBskyEmbedImagesView(Box<crate::app::bsky::embed::images::View>),
    #[serde(rename = "app.bsky.embed.video#view")]
    AppBskyEmbedVideoView(Box<crate::app::bsky::embed::video::View>),
    #[serde(rename = "app.bsky.embed.external#view")]
    AppBskyEmbedExternalView(Box<crate::app::bsky::embed::external::View>),
    #[serde(rename = "app.bsky.embed.record#view")]
    AppBskyEmbedRecordView(Box<crate::app::bsky::embed::record::View>),
    #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
    AppBskyEmbedRecordWithMediaView(
        Box<crate::app::bsky::embed::record_with_media::View>,
    ),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ViewRecordRefs {
    #[serde(rename = "app.bsky.embed.record#viewRecord")]
    ViewRecord(Box<ViewRecord>),
    #[serde(rename = "app.bsky.embed.record#viewNotFound")]
    ViewNotFound(Box<ViewNotFound>),
    #[serde(rename = "app.bsky.embed.record#viewBlocked")]
    ViewBlocked(Box<ViewBlocked>),
    #[serde(rename = "app.bsky.embed.record#viewDetached")]
    ViewDetached(Box<ViewDetached>),
    #[serde(rename = "app.bsky.feed.defs#generatorView")]
    AppBskyFeedDefsGeneratorView(Box<crate::app::bsky::feed::defs::GeneratorView>),
    #[serde(rename = "app.bsky.graph.defs#listView")]
    AppBskyGraphDefsListView(Box<crate::app::bsky::graph::defs::ListView>),
    #[serde(rename = "app.bsky.labeler.defs#labelerView")]
    AppBskyLabelerDefsLabelerView(Box<crate::app::bsky::labeler::defs::LabelerView>),
    #[serde(rename = "app.bsky.graph.defs#starterPackViewBasic")]
    AppBskyGraphDefsStarterPackViewBasic(
        Box<crate::app::bsky::graph::defs::StarterPackViewBasic>,
    ),
}
