use crate::commands::Command;
use crate::store::SimpleJsonFileSessionStore;
use anyhow::{Context, Result};
use atrium_api::agent::bluesky::{AtprotoServiceType, BSKY_CHAT_DID};
use atrium_api::agent::{store::SessionStore, AtpAgent};
use atrium_api::records::{KnownRecord, Record};
use atrium_api::types::string::{AtIdentifier, Datetime, Handle};
use atrium_api::types::LimitedNonZeroU8;
use atrium_xrpc_client::reqwest::ReqwestClient;
use serde::Serialize;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncReadExt;

pub struct Runner {
    agent: AtpAgent<SimpleJsonFileSessionStore, ReqwestClient>,
    limit: LimitedNonZeroU8<100>,
    debug: bool,
    session_path: PathBuf,
    handle: Option<Handle>,
}

impl Runner {
    pub async fn new(pds_host: String, limit: LimitedNonZeroU8<100>, debug: bool) -> Result<Self> {
        let config_dir = dirs::config_dir()
            .with_context(|| format!("No config dir: {:?}", dirs::config_dir()))?;
        let dir = config_dir.join("atrium-cli");
        create_dir_all(&dir).await?;
        let session_path = dir.join("session.json");
        let store = SimpleJsonFileSessionStore::new(session_path.clone());
        let session = store.get_session().await;
        let handle = session.as_ref().map(|s| s.handle.clone());
        let agent = AtpAgent::new(ReqwestClient::new(pds_host), store);
        if let Some(s) = &session {
            agent.resume_session(s.clone()).await?;
        }
        Ok(Self {
            agent,
            limit,
            debug,
            session_path,
            handle,
        })
    }
    pub async fn run(&self, command: Command) -> Result<()> {
        let limit = self.limit;
        match command {
            Command::Login(args) => {
                self.agent.login(args.identifier, args.password).await?;
                println!("Login successful! Saved session to {:?}", self.session_path);
                Ok(())
            }
            Command::GetTimeline => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .feed
                    .get_timeline(
                        atrium_api::app::bsky::feed::get_timeline::ParametersData {
                            algorithm: None,
                            cursor: None,
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetAuthorFeed(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .feed
                    .get_author_feed(
                        atrium_api::app::bsky::feed::get_author_feed::ParametersData {
                            actor: args
                                .actor
                                .or(self.handle.clone().map(AtIdentifier::Handle))
                                .with_context(|| "Not logged in")?,
                            cursor: None,
                            filter: None,
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetLikes(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .feed
                    .get_likes(
                        atrium_api::app::bsky::feed::get_likes::ParametersData {
                            cid: None,
                            cursor: None,
                            limit: Some(limit),
                            uri: args.uri.to_string(),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetRepostedBy(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .feed
                    .get_reposted_by(
                        atrium_api::app::bsky::feed::get_reposted_by::ParametersData {
                            cid: None,
                            cursor: None,
                            limit: Some(limit),
                            uri: args.uri.to_string(),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetActorFeeds(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .feed
                    .get_actor_feeds(
                        atrium_api::app::bsky::feed::get_actor_feeds::ParametersData {
                            actor: args
                                .actor
                                .or(self.handle.clone().map(AtIdentifier::Handle))
                                .with_context(|| "Not logged in")?,
                            cursor: None,
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetFeed(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .feed
                    .get_feed(
                        atrium_api::app::bsky::feed::get_feed::ParametersData {
                            cursor: None,
                            feed: args.uri.to_string(),
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetListFeed(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .feed
                    .get_list_feed(
                        atrium_api::app::bsky::feed::get_list_feed::ParametersData {
                            cursor: None,
                            limit: Some(limit),
                            list: args.uri.to_string(),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetFollows(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .graph
                    .get_follows(
                        atrium_api::app::bsky::graph::get_follows::ParametersData {
                            actor: args
                                .actor
                                .or(self.handle.clone().map(AtIdentifier::Handle))
                                .with_context(|| "Not logged in")?,
                            cursor: None,
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetFollowers(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .graph
                    .get_followers(
                        atrium_api::app::bsky::graph::get_followers::ParametersData {
                            actor: args
                                .actor
                                .or(self.handle.clone().map(AtIdentifier::Handle))
                                .with_context(|| "Not logged in")?,
                            cursor: None,
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetLists(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .graph
                    .get_lists(
                        atrium_api::app::bsky::graph::get_lists::ParametersData {
                            actor: args
                                .actor
                                .or(self.handle.clone().map(AtIdentifier::Handle))
                                .with_context(|| "Not logged in")?,
                            cursor: None,
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetList(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .graph
                    .get_list(
                        atrium_api::app::bsky::graph::get_list::ParametersData {
                            cursor: None,
                            limit: Some(limit),
                            list: args.uri.to_string(),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetProfile(args) => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .actor
                    .get_profile(
                        atrium_api::app::bsky::actor::get_profile::ParametersData {
                            actor: args
                                .actor
                                .or(self.handle.clone().map(AtIdentifier::Handle))
                                .with_context(|| "Not logged in")?,
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::GetPreferences => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .actor
                    .get_preferences(
                        atrium_api::app::bsky::actor::get_preferences::ParametersData {}.into(),
                    )
                    .await?,
            ),
            Command::ListNotifications => self.print(
                &self
                    .agent
                    .api
                    .app
                    .bsky
                    .notification
                    .list_notifications(
                        atrium_api::app::bsky::notification::list_notifications::ParametersData {
                            cursor: None,
                            limit: Some(limit),
                            seen_at: None,
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::ListConvos => self.print(
                &self
                    .agent
                    .api_with_proxy(
                        BSKY_CHAT_DID.parse().expect("valid DID"),
                        AtprotoServiceType::BskyChat,
                    )
                    .chat
                    .bsky
                    .convo
                    .list_convos(
                        atrium_api::chat::bsky::convo::list_convos::ParametersData {
                            cursor: None,
                            limit: Some(limit),
                        }
                        .into(),
                    )
                    .await?,
            ),
            Command::SendConvoMessage(args) => {
                let did = match args.actor {
                    AtIdentifier::Handle(handle) => self
                        .agent
                        .api
                        .com
                        .atproto
                        .identity
                        .resolve_handle(
                            atrium_api::com::atproto::identity::resolve_handle::ParametersData {
                                handle: handle.clone(),
                            }
                            .into(),
                        )
                        .await?
                        .data
                        .did,
                    AtIdentifier::Did(did) => did,
                };
                let chat = &self
                    .agent
                    .api_with_proxy(
                        BSKY_CHAT_DID.parse().expect("valid DID"),
                        AtprotoServiceType::BskyChat,
                    )
                    .chat;
                let convo = chat
                    .bsky
                    .convo
                    .get_convo_for_members(
                        atrium_api::chat::bsky::convo::get_convo_for_members::ParametersData {
                            members: vec![did],
                        }
                        .into(),
                    )
                    .await?;
                self.print(
                    &chat
                        .bsky
                        .convo
                        .send_message(
                            atrium_api::chat::bsky::convo::send_message::InputData {
                                convo_id: convo.data.convo.data.id,
                                message: atrium_api::chat::bsky::convo::defs::MessageInputData {
                                    embed: None,
                                    facets: None,
                                    text: args.text,
                                }
                                .into(),
                            }
                            .into(),
                        )
                        .await?,
                )
            }
            Command::CreatePost(args) => {
                let mut images = Vec::new();
                for image in &args.images {
                    if let Ok(mut file) = File::open(image).await {
                        let mut buf = Vec::new();
                        file.read_to_end(&mut buf).await.expect("read image file");
                        let output = self
                            .agent
                            .api
                            .com
                            .atproto
                            .repo
                            .upload_blob(buf)
                            .await
                            .expect("upload blob");
                        images.push(
                            atrium_api::app::bsky::embed::images::ImageData {
                                alt: image
                                    .file_name()
                                    .map(OsStr::to_string_lossy)
                                    .unwrap_or_default()
                                    .into(),
                                aspect_ratio: None,
                                image: output.data.blob,
                            }
                            .into(),
                        )
                    }
                }
                let embed = Some(atrium_api::types::Union::Refs(
                    atrium_api::app::bsky::feed::post::RecordEmbedRefs::AppBskyEmbedImagesMain(
                        Box::new(atrium_api::app::bsky::embed::images::MainData { images }.into()),
                    ),
                ));
                self.print(
                    &self
                        .agent
                        .api
                        .com
                        .atproto
                        .repo
                        .create_record(
                            atrium_api::com::atproto::repo::create_record::InputData {
                                collection: "app.bsky.feed.post".parse().expect("valid"),
                                record: Record::Known(KnownRecord::AppBskyFeedPost(Box::new(
                                    atrium_api::app::bsky::feed::post::RecordData {
                                        created_at: Datetime::now(),
                                        embed,
                                        entities: None,
                                        facets: None,
                                        labels: None,
                                        langs: None,
                                        reply: None,
                                        tags: None,
                                        text: args.text,
                                    }
                                    .into(),
                                ))),
                                repo: self.handle.clone().with_context(|| "Not logged in")?.into(),
                                rkey: None,
                                swap_commit: None,
                                validate: None,
                            }
                            .into(),
                        )
                        .await?,
                )
            }
            Command::DeletePost(args) => self.print(
                &self
                    .agent
                    .api
                    .com
                    .atproto
                    .repo
                    .delete_record(
                        atrium_api::com::atproto::repo::delete_record::InputData {
                            collection: "app.bsky.feed.post".parse().expect("valid"),
                            repo: self.handle.clone().with_context(|| "Not logged in")?.into(),
                            rkey: args.uri.rkey,
                            swap_commit: None,
                            swap_record: None,
                        }
                        .into(),
                    )
                    .await?,
            ),
        }
    }
    fn print<T: std::fmt::Debug + Serialize>(&self, result: &T) -> Result<()> {
        if self.debug {
            println!("{:#?}", result);
        } else {
            println!("{}", serde_json::to_string_pretty(result)?);
        }
        Ok(())
    }
}
