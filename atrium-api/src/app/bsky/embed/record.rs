// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `app.bsky.embed.record` namespace."]
#![doc = "A representation of a record embedded in another form of content"]
#[doc = "`app.bsky.embed.record`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub record: crate::com::atproto::repo::strong_ref::Main,
}
#[doc = "`app.bsky.embed.record#view`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub record: ViewRecordEnum,
}
#[doc = "`app.bsky.embed.record#viewBlocked`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewBlocked {
    pub uri: String,
}
#[doc = "`app.bsky.embed.record#viewNotFound`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewNotFound {
    pub uri: String,
}
#[doc = "`app.bsky.embed.record#viewRecord`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewRecord {
    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub cid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<ViewRecordEmbedsItem>>,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    pub uri: String,
    pub value: crate::records::Record,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ViewRecordEmbedsItem {
    #[serde(rename = "app.bsky.embed.images#view")]
    AppBskyEmbedImagesView(Box<crate::app::bsky::embed::images::View>),
    #[serde(rename = "app.bsky.embed.external#view")]
    AppBskyEmbedExternalView(Box<crate::app::bsky::embed::external::View>),
    #[serde(rename = "app.bsky.embed.record#view")]
    AppBskyEmbedRecordView(Box<crate::app::bsky::embed::record::View>),
    #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
    AppBskyEmbedRecordWithMediaView(Box<crate::app::bsky::embed::record_with_media::View>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ViewRecordEnum {
    #[serde(rename = "app.bsky.embed.record#viewRecord")]
    ViewRecord(Box<ViewRecord>),
    #[serde(rename = "app.bsky.embed.record#viewNotFound")]
    ViewNotFound(Box<ViewNotFound>),
    #[serde(rename = "app.bsky.embed.record#viewBlocked")]
    ViewBlocked(Box<ViewBlocked>),
}
