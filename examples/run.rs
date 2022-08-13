use std::path::Path;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use lib_ngrok::{run, download};

#[tokio::main]
async fn main() -> Result<()> {

    let ngrok_path = &Path::new("ngrok");

    download::download(ngrok_path).await?;

    let auth_token: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter NGROK auth token")
        .interact()?;

    run(ngrok_path, &auth_token, &std::env::args().skip(1).collect::<Vec<String>>())?;

    Ok(())

}