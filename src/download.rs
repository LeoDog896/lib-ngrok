use crate::url::NGROK_PATH;
use bytes::Buf;
use flate2::read::GzDecoder;
use std::{ffi::OsStr, fs::File, io, io::copy, path::Path};
use tar::Archive;

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("io")]
    IO(#[from] io::Error),
    #[error("could not support file extension `{0}`")]
    UnsupportedFileExtension(String),
    #[error("no file extension found")]
    NoFileExtension,
    #[error("no files found in archive")]
    NoFiles,
    #[error("Could not download file")]
    Reqwest(#[from] reqwest::Error),
}

pub async fn bin(output: &Path) -> Result<(), DownloadError> {
    // Download the ngrok binary
    let bytes = reqwest::get(NGROK_PATH).await?.bytes().await?.reader();

    // Unzip it
    return match Path::new(NGROK_PATH).extension().and_then(OsStr::to_str) {
        Some(ext) if ext == "zip" => {
            todo!("Unzip the zip file");
        }
        Some(ext) if ext == "tgz" => {
            let decompress_stream = GzDecoder::new(bytes);
            let mut archive = Archive::new(decompress_stream);
            let mut x = archive
                .entries()?
                .into_iter()
                .next()
                .transpose()?
                .ok_or(DownloadError::NoFiles)?;

            copy(&mut x, &mut File::create(output)?)?;

            Ok(())
        }
        Some(ext) => Err(DownloadError::UnsupportedFileExtension(ext.to_string())),
        None => Err(DownloadError::NoFileExtension),
    };
}
