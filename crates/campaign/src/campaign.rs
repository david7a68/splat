//! The identity and location of a local campaign.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    #[serde(default)]
    pub dndbeyond_campaign_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dndbeyond_campaign_name: Option<String>,
}
