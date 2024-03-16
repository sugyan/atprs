// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.getAccountInfo` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub did: crate::types::string::Did,
}
pub type Output = crate::com::atproto::admin::defs::AccountView;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            _ => {}
        }
        Ok(())
    }
}
