use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::{ApiClient, GetSharedProjectRes, ShareProjectReq, ShareProjectRes},
    AppError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub description: Option<String>,
    pub path: String,
    contents: String,
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
            path: path.unwrap_or("main.rs".to_string()),
            id,
            shared_id: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn contents(&self) -> String {
        self.contents.clone()
    }

    pub fn set_contents(&mut self, new_contents: impl ToString) {
        self.contents = new_contents.to_string();
        self.shared_id = None;
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
            path: "main.rs".to_string(),
            contents: shared.code.clone(),
            prebuilt: false,
            id,
            shared_id: Some(shared.id),
        })
    }

    pub async fn share_project(&mut self, client: &ApiClient) -> Result<String, AppError> {
        // If the project has already been shared, return the share code.
        // We remove the shared id if the content changes.
        if let Some(share_code) = &self.shared_id {
            return Ok(share_code.clone());
        }

        let url = format!("{}/shared", client.server_url);
        let res = client
            .post(url)
            .json(&ShareProjectReq {
                code: self.contents.clone(),
            })
            .send()
            .await?;

        let res = res.json::<ShareProjectRes>().await?;
        self.shared_id = Some(res.id.clone());

        Ok(res.id)
    }
}
