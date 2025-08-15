//! Deploy key related data structures

use serde::{Deserialize, Serialize};

/// Response from GitHub API when creating or fetching a deploy key
#[derive(Serialize, Deserialize, Debug)]
pub struct DeployKeyResponse {
    pub id: u64,
    pub key: String,
    pub url: String,
    pub title: String,
    pub verified: bool,
    pub created_at: String,
    pub read_only: bool,
}

/// Represents a list of deploy keys
#[derive(Serialize, Deserialize, Debug)]
pub struct DeployKeyList(pub Vec<DeployKeyResponse>);

/// Represents a list of repository secrets
#[derive(Serialize, Deserialize, Debug)]
pub struct SecretList {
    pub total_count: u64,
    pub secrets: Vec<Secret>,
}

/// Represents a repository secret
#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}