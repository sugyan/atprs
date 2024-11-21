// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.sync.subscribeRepos` namespace.
pub const NSID: &str = "com.atproto.sync.subscribeRepos";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    ///The last known event seq number to backfill from.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<i64>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    FutureCursor(Option<String>),
    ///If the consumer of the stream can not keep up with events, and a backlog gets too large, the server will drop the connection.
    ConsumerTooSlow(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FutureCursor(msg) => {
                write!(_f, "FutureCursor")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::ConsumerTooSlow(msg) => {
                write!(_f, "ConsumerTooSlow")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
///Represents a change to an account's status on a host (eg, PDS or Relay). The semantics of this event are that the status is at the host which emitted the event, not necessarily that at the currently active PDS. Eg, a Relay takedown would emit a takedown with active=false, even if the PDS is still active.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    ///Indicates that the account has a repository which can be fetched from the host that emitted this event.
    pub active: bool,
    pub did: crate::types::string::Did,
    pub seq: i64,
    ///If active=false, this optional field indicates a reason for why the account is not active.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub status: core::option::Option<String>,
    pub time: crate::types::string::Datetime,
}
pub type Account = crate::types::Object<AccountData>;
///Represents an update of repository state. Note that empty commits are allowed, which include no repo data changes, but an update to rev and signature.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommitData {
    pub blobs: Vec<crate::types::CidLink>,
    ///CAR file containing relevant blocks, as a diff since the previous repo state.
    #[serde(with = "serde_bytes")]
    pub blocks: Vec<u8>,
    ///Repo commit object CID.
    pub commit: crate::types::CidLink,
    pub ops: Vec<RepoOp>,
    ///DEPRECATED -- unused. WARNING -- nullable and optional; stick with optional to ensure golang interoperability.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub prev: core::option::Option<crate::types::CidLink>,
    ///DEPRECATED -- unused
    pub rebase: bool,
    ///The repo this event comes from.
    pub repo: crate::types::string::Did,
    ///The rev of the emitted commit. Note that this information is also in the commit object included in blocks, unless this is a tooBig event.
    pub rev: String,
    ///The stream sequence number of this message.
    pub seq: i64,
    ///The rev of the last emitted commit from this repo (if any).
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub since: core::option::Option<String>,
    ///Timestamp of when this message was originally broadcast.
    pub time: crate::types::string::Datetime,
    ///Indicates that this commit contained too many ops, or data size was too large. Consumers will need to make a separate request to get missing data.
    pub too_big: bool,
}
pub type Commit = crate::types::Object<CommitData>;
///DEPRECATED -- Use #identity event instead
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HandleData {
    pub did: crate::types::string::Did,
    pub handle: crate::types::string::Handle,
    pub seq: i64,
    pub time: crate::types::string::Datetime,
}
pub type Handle = crate::types::Object<HandleData>;
///Represents a change to an account's identity. Could be an updated handle, signing key, or pds hosting endpoint. Serves as a prod to all downstream services to refresh their identity cache.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IdentityData {
    pub did: crate::types::string::Did,
    ///The current handle for the account, or 'handle.invalid' if validation fails. This field is optional, might have been validated or passed-through from an upstream source. Semantics and behaviors for PDS vs Relay may evolve in the future; see atproto specs for more details.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub handle: core::option::Option<crate::types::string::Handle>,
    pub seq: i64,
    pub time: crate::types::string::Datetime,
}
pub type Identity = crate::types::Object<IdentityData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InfoData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub message: core::option::Option<String>,
    pub name: String,
}
pub type Info = crate::types::Object<InfoData>;
///DEPRECATED -- Use #account event instead
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MigrateData {
    pub did: crate::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub migrate_to: core::option::Option<String>,
    pub seq: i64,
    pub time: crate::types::string::Datetime,
}
pub type Migrate = crate::types::Object<MigrateData>;
///A repo operation, ie a mutation of a single record.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RepoOpData {
    pub action: String,
    ///For creates and updates, the new record CID. For deletions, null.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cid: core::option::Option<crate::types::CidLink>,
    pub path: String,
}
pub type RepoOp = crate::types::Object<RepoOpData>;
///DEPRECATED -- Use #account event instead
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TombstoneData {
    pub did: crate::types::string::Did,
    pub seq: i64,
    pub time: crate::types::string::Datetime,
}
pub type Tombstone = crate::types::Object<TombstoneData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum Message {
    #[serde(rename = "com.atproto.sync.subscribeRepos#commit")]
    Commit(Box<Commit>),
    #[serde(rename = "com.atproto.sync.subscribeRepos#identity")]
    Identity(Box<Identity>),
    #[serde(rename = "com.atproto.sync.subscribeRepos#account")]
    Account(Box<Account>),
    #[serde(rename = "com.atproto.sync.subscribeRepos#handle")]
    Handle(Box<Handle>),
    #[serde(rename = "com.atproto.sync.subscribeRepos#migrate")]
    Migrate(Box<Migrate>),
    #[serde(rename = "com.atproto.sync.subscribeRepos#tombstone")]
    Tombstone(Box<Tombstone>),
    #[serde(rename = "com.atproto.sync.subscribeRepos#info")]
    Info(Box<Info>),
}
