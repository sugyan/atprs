// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `app.bsky.feed.describeFeedGenerator` namespace."]
#[doc = "`app.bsky.feed.describeFeedGenerator`"]
#[doc = "Returns information about a given feed generator including TOS & offered feed URIs"]
#[async_trait::async_trait]
pub trait DescribeFeedGenerator: crate::xrpc::XrpcClient {
    async fn describe_feed_generator(&self) -> Result<Output, Box<dyn std::error::Error>> {
        let body = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::GET,
            "app.bsky.feed.describeFeedGenerator",
            None,
            None,
            None,
        )
        .await?;
        serde_json::from_slice(&body).map_err(|e| e.into())
    }
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub did: String,
    pub feeds: Vec<Feed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
#[doc = "`app.bsky.feed.describeFeedGenerator#feed`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub uri: String,
}
#[doc = "`app.bsky.feed.describeFeedGenerator#links`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
}