use lib_ngrok::download::{self, DownloadError};
use std::io::stdout;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    download::to_write(&mut stdout()).await?;

    Ok(())
}
