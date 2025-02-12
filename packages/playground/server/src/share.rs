use std::collections::HashMap;

use crate::app::{AppState, Error};
use axum::{
    extract::{Path, State},
    Json,
};
use dioxus_logger::tracing::trace;
use gists::{GistFile, NewGist};
use model::share::{GetGistRes, SaveToGistReq, SaveToGistRes};

const PRIMARY_GIST_FILE_NAME: &str = "dxp.rs";

pub async fn get_gist(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetGistRes>, Error> {
    trace!(id = ?id, "get_gist request");
    let gist = gists::get(&state, id).await?;

    let Some(file) = gist.files.get(PRIMARY_GIST_FILE_NAME) else {
        return Err(Error::InternalServerError);
    };

    Ok(Json(GetGistRes {
        id: gist.id,
        code: file.content.clone(),
    }))
}

pub async fn save_to_gist(
    State(state): State<AppState>,
    Json(payload): Json<SaveToGistReq>,
) -> Result<Json<SaveToGistRes>, Error> {
    trace!(payload = ?payload, "save_to_gist request");
    let mut files = HashMap::new();
    files.insert(
        PRIMARY_GIST_FILE_NAME.to_string(),
        GistFile {
            content: payload.code,
        },
    );

    let new_gist = NewGist {
        description: "A user-saved Dioxus Playground snippet.".to_string(),
        public: false,
        files,
    };

    let new_gist = gists::create(&state, new_gist).await?;
    Ok(Json(SaveToGistRes { id: new_gist.id }))
}

pub mod gists {
    use crate::app::{AppState, Error};
    use reqwest::{header, StatusCode};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    const GISTS_URL_PREFIX: &str = "https://api.github.com/gists";
    const GITHUB_USER_AGENT: &str = "Dioxus Playground";

    pub async fn get(state: &AppState, id: String) -> Result<Gist, Error> {
        let res = state
            .reqwest_client
            .get(format!("{GISTS_URL_PREFIX}/{id}"))
            .bearer_auth(&state.env.gist_auth_token)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header(header::USER_AGENT, GITHUB_USER_AGENT)
            .send()
            .await?;

        // Was the gist found?
        if res.status() == StatusCode::NOT_FOUND {
            return Err(Error::ResourceNotFound);
        }

        Ok(res.json::<Gist>().await?)
    }

    pub async fn create(state: &AppState, gist: NewGist) -> Result<Gist, Error> {
        let res = state
            .reqwest_client
            .post(GISTS_URL_PREFIX)
            .bearer_auth(&state.env.gist_auth_token)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header(header::USER_AGENT, GITHUB_USER_AGENT)
            .json(&gist)
            .send()
            .await?;

        Ok(res.json::<Gist>().await?)
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Gist {
        pub id: String,
        pub files: HashMap<String, GistFile>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GistFile {
        pub content: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NewGist {
        pub description: String,
        pub public: bool,
        pub files: HashMap<String, GistFile>,
    }
}
