use dioxus::signals::{Signal, Writable};
use dioxus_document::eval;
use model::{api::ApiClient, AppError, Project};

/// Share a project and copy the link to the clipboard.
pub async fn copy_share_link(
    api_client: &ApiClient,
    mut project: Signal<Project>,
    location: &str,
) -> Result<(), AppError> {
    let share_code = project.write().share_project(api_client).await?;

    let formatted = format!("{}/shared/{}", location, share_code);
    let e = eval(
        r#"
        const data = await dioxus.recv();
        navigator.clipboard.writeText(data);
        "#,
    );

    e.send(formatted)?;

    Ok(())
}
