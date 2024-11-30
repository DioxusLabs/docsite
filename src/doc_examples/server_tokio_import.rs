use dioxus::prelude::*;
// ANCHOR: tokio_import
// Since the tokio dependency is only enabled in the server feature,
// we need to only import it when the server feature is enabled.
#[cfg(feature = "server")]
use tokio::fs::File;
#[cfg(feature = "server")]
use tokio::io::AsyncReadExt;
// ANCHOR_END: tokio_import

// ANCHOR: tokio_usage
// Since the tokio dependency is only enabled in the server feature,
// we need to only compile any usage of the dependency when the server feature is enabled.
#[cfg(feature = "server")]
async fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("path/to/file").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

// The bodies of server functions automatically only compile when the server feature is enabled.
#[server]
async fn get_file_contents() -> Result<String, ServerFnError> {
    let mut file = File::open("path/to/file").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
// ANCHOR_END: tokio_usage

// ANCHOR: tokio_module
// Instead of configuring each item that is only used in the server, you can group
// them into a module that is only compiled when the server feature is enabled.
#[cfg(feature = "server")]
mod tokio_utilities {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    pub async fn read_file() -> Result<String, std::io::Error> {
        let mut file = File::open("path/to/file").await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        Ok(contents)
    }
}

// Now you can define a slim server function (wrapper) that is accessible by the
// client.
#[server]
async fn get_file_contents() -> Result<String, ServerFnError> {
    tokio_utilities::read_file().await
}
// ANCHOR_END: tokio_module
