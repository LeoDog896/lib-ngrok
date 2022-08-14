use anyhow::Result;
use lib_ngrok::download;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let ngrok_path = &Path::new("ngrok");

    download::to_path(ngrok_path).await?;

    println!("Binary downloaded!");

    Ok(())
}
