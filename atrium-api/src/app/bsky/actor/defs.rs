// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.actor.defs` namespace.
//!A reference to an actor in the network.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdultContentPref {
    pub enabled: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContentLabelPref {
    pub label: String,
    pub visibility: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedViewPref {
    ///The URI of the feed, or an identifier which describes the feed.
    pub feed: String,
    ///Hide quote posts in the feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_quote_posts: Option<bool>,
    ///Hide replies in the feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_replies: Option<bool>,
    ///Hide replies in the feed if they do not have this number of likes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_replies_by_like_count: Option<i32>,
    ///Hide replies in the feed if they are not by followed users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_replies_by_unfollowed: Option<bool>,
    ///Hide reposts in the feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_reposts: Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonalDetailsPref {
    ///The birth date of account owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
}
pub type Preferences = Vec<PreferencesItem>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub handle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewBasic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub handle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewDetailed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follows_count: Option<i32>,
    pub handle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedFeedsPref {
    pub pinned: Vec<String>,
    pub saved: Vec<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ThreadViewPref {
    ///Show followed users at the top of all replies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritize_followed_users: Option<bool>,
    ///Sorting mode for threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewerState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_by: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking_by_list: Option<crate::app::bsky::graph::defs::ListViewBasic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted_by_list: Option<crate::app::bsky::graph::defs::ListViewBasic>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum PreferencesItem {
    #[serde(rename = "app.bsky.actor.defs#adultContentPref")]
    AdultContentPref(Box<AdultContentPref>),
    #[serde(rename = "app.bsky.actor.defs#contentLabelPref")]
    ContentLabelPref(Box<ContentLabelPref>),
    #[serde(rename = "app.bsky.actor.defs#savedFeedsPref")]
    SavedFeedsPref(Box<SavedFeedsPref>),
    #[serde(rename = "app.bsky.actor.defs#personalDetailsPref")]
    PersonalDetailsPref(Box<PersonalDetailsPref>),
    #[serde(rename = "app.bsky.actor.defs#feedViewPref")]
    FeedViewPref(Box<FeedViewPref>),
    #[serde(rename = "app.bsky.actor.defs#threadViewPref")]
    ThreadViewPref(Box<ThreadViewPref>),
}
