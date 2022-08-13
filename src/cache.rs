//! Helper functions for caching the ngrok binary

use bytes::Buf;
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, path::Path};

use crate::{
    download::{bin, DownloadError},
    url::NGROK_PATH,
};

pub async fn download_if_new(output: &Path) -> Result<(), DownloadError> {
    if output.exists() {
        let local_checksum = {
            let mut hasher = Sha256::new();

            let mut file = File::open(output)?;

            let mut buffer = [0; 1024];
            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }

            hasher.finalize()
        };

        let remote_checksum = {
            let mut bytes = reqwest::get(NGROK_PATH).await?.bytes().await?.reader();

            let mut hasher = Sha256::new();

            let mut buffer = [0; 1024];
            loop {
                let bytes_read = bytes.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }

            hasher.finalize()
        };

        if local_checksum == remote_checksum {
            return Ok(());
        }
    }

    bin(output).await?;
    Ok(())
}
