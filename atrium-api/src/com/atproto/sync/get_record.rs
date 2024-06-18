// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.sync.getRecord` namespace.
pub const NSID: &str = "com.atproto.sync.getRecord";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub collection: crate::types::string::Nsid,
    ///DEPRECATED: referenced a repo commit by CID, and retrieved record as of that commit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<crate::types::string::Cid>,
    ///The DID of the repo.
    pub did: crate::types::string::Did,
    ///Record Key
    pub rkey: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    RecordNotFound(Option<String>),
    RepoNotFound(Option<String>),
    RepoTakendown(Option<String>),
    RepoSuspended(Option<String>),
    RepoDeactivated(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RecordNotFound(msg) => {
                write!(_f, "RecordNotFound")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::RepoNotFound(msg) => {
                write!(_f, "RepoNotFound")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::RepoTakendown(msg) => {
                write!(_f, "RepoTakendown")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::RepoSuspended(msg) => {
                write!(_f, "RepoSuspended")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::RepoDeactivated(msg) => {
                write!(_f, "RepoDeactivated")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
