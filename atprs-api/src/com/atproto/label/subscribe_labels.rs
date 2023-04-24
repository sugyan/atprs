// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.label.subscribeLabels` namespace.

// com.atproto.label.subscribeLabels
/// Subscribe to label updates
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Main {}

// com.atproto.label.subscribeLabels#info
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Info {
    pub message: Option<String>,
    pub name: String,
}

// com.atproto.label.subscribeLabels#labels
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Labels {
    pub labels: Vec<crate::com::atproto::label::defs::Label>,
    pub seq: i32,
}
