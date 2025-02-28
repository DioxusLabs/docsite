use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// API request to share a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShareProjectReq {
    pub code: String,
}

/// API response for sharing a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShareProjectRes {
    pub id: String,
}

/// API response for requesting a shared project.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSharedProjectRes {
    pub id: String,
    pub code: String,
}

/// An api client for the Dioxus Playground server.
#[derive(Clone)]
pub struct ApiClient {
    pub server_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(server_url: impl ToString) -> Self {
        let client = reqwest::Client::new();

        Self {
            server_url: server_url.to_string(),
            client,
        }
    }
}

impl Deref for ApiClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for ApiClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
