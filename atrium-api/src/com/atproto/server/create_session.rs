// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.server.createSession` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_factor_token: Option<String>,
    ///Handle or other identifier supported by the server for the authenticating user.
    pub identifier: String,
    pub password: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub access_jwt: String,
    pub did: crate::types::string::Did,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<crate::did_doc::DidDocument>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_auth_factor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_confirmed: Option<bool>,
    pub handle: crate::types::string::Handle,
    pub refresh_jwt: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    AccountTakedown(Option<String>),
    AuthFactorTokenRequired(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AccountTakedown(msg) => {
                write!(_f, "AccountTakedown")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::AuthFactorTokenRequired(msg) => {
                write!(_f, "AuthFactorTokenRequired")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
