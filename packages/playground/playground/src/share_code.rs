use dioxus::prelude::*;
use dioxus_document::eval;
use model::{api::ApiClient, AppError, Project};

/// Share a project and copy the link to the clipboard.
pub async fn copy_share_link(
    api_client: &ApiClient,
    mut project: Signal<Project>,
    location: &str,
) -> Result<(), AppError> {
    let read_project = project.read();
    let shared_id = read_project.shared_id();
    let code = read_project.contents();
    // Drop the lock before running any async code.
    drop(read_project);
    let share_code = Project::share_project(shared_id, code, api_client).await?;

    project.write().set_shared_id(share_code);

    let formatted = format!("{}/shared/{}", location, share_code.as_simple());
    let e = eval(
        r#"
        const data = await dioxus.recv();
        navigator.clipboard.writeText(data);
        "#,
    );

    e.send(&formatted)?;
    router().push(formatted);

    Ok(())
}
