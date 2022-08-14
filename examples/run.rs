use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use lib_ngrok::{download, run};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let ngrok_path = &Path::new("ngrok");

    download::to_path(ngrok_path).await?;

    let auth_token: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter NGROK auth token")
        .interact()?;

    run(
        ngrok_path,
        &auth_token,
        &std::env::args().skip(1).collect::<Vec<String>>(),
    )?;

    Ok(())
}
