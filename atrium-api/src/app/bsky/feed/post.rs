// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.post` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordData {
    ///Client-declared timestamp when this post was originally created.
    pub created_at: crate::types::string::Datetime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<crate::types::Union<RecordEmbedRefs>>,
    ///DEPRECATED: replaced by app.bsky.richtext.facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    ///Annotations of text (mentions, URLs, hashtags, etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    ///Self-label values for this post. Effectively content warnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::Union<RecordLabelsRefs>>,
    ///Indicates human language of post primary text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langs: Option<Vec<crate::types::string::Language>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
    ///Additional hashtags, in addition to any included in post text and facets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    ///The primary post content. May be an empty string, if there are embeds.
    pub text: String,
}
pub type Record = crate::types::Object<RecordData>;
///Deprecated: use facets instead.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntityData {
    pub index: TextSlice,
    ///Expected values are 'mention' and 'link'.
    pub r#type: String,
    pub value: String,
}
pub type Entity = crate::types::Object<EntityData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReplyRefData {
    pub parent: crate::com::atproto::repo::strong_ref::Main,
    pub root: crate::com::atproto::repo::strong_ref::Main,
}
pub type ReplyRef = crate::types::Object<ReplyRefData>;
///Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TextSliceData {
    pub end: usize,
    pub start: usize,
}
pub type TextSlice = crate::types::Object<TextSliceData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum RecordEmbedRefs {
    #[serde(rename = "app.bsky.embed.images")]
    AppBskyEmbedImagesMain(Box<crate::app::bsky::embed::images::Main>),
    #[serde(rename = "app.bsky.embed.video")]
    AppBskyEmbedVideoMain(Box<crate::app::bsky::embed::video::Main>),
    #[serde(rename = "app.bsky.embed.external")]
    AppBskyEmbedExternalMain(Box<crate::app::bsky::embed::external::Main>),
    #[serde(rename = "app.bsky.embed.record")]
    AppBskyEmbedRecordMain(Box<crate::app::bsky::embed::record::Main>),
    #[serde(rename = "app.bsky.embed.recordWithMedia")]
    AppBskyEmbedRecordWithMediaMain(
        Box<crate::app::bsky::embed::record_with_media::Main>,
    ),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum RecordLabelsRefs {
    #[serde(rename = "com.atproto.label.defs#selfLabels")]
    ComAtprotoLabelDefsSelfLabels(Box<crate::com::atproto::label::defs::SelfLabels>),
}
