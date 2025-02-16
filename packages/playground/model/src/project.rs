use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::{ApiClient, GetSharedProjectRes},
    AppError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub description: Option<String>,
    pub path: Option<String>,
    pub contents: String,
    pub prebuilt: bool,
    id: Uuid,
    shared_id: Option<String>,
}

impl Project {
    pub fn new(contents: impl ToString, description: Option<String>, path: Option<String>) -> Self {
        let contents = contents.to_string();

        // Generate a unique id for the example.
        let id = Uuid::new_v3(&Uuid::NAMESPACE_URL, contents.as_bytes());
        Self {
            prebuilt: false,
            contents,
            description,
            path,
            id,
            shared_id: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Retrieve the shared project information from a share code.
    pub async fn from_share_code(client: &ApiClient, share_code: String) -> Result<Self, AppError> {
        let url = format!("{}/shared/{}", client.server_url, share_code);

        let res = client.get(url).send().await?;
        if res.status() == StatusCode::NOT_FOUND {
            return Err(AppError::ResourceNotFound);
        }

        // Decode
        let shared = res.json::<GetSharedProjectRes>().await?;
        let id = Uuid::new_v3(&Uuid::NAMESPACE_URL, shared.code.as_bytes());

        Ok(Self {
            description: None,
            path: None,
            contents: shared.code,
            prebuilt: false,
            id,
            shared_id: Some(shared.id),
        })
    }
}
