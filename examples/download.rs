use std::path::Path;
use anyhow::Result;
use lib_ngrok::download;

#[tokio::main]
async fn main() -> Result<()> {

    let ngrok_path = &Path::new("ngrok");

    download::bin(ngrok_path).await?;

    println!("Binary downloaded!");

    Ok(())
}