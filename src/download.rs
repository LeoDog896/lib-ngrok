use crate::url::NGROK_PATH;
use bytes::Buf;
use flate2::read::GzDecoder;
use std::{
    ffi::OsStr,
    fs::File,
    io::copy,
    io::{self, Write},
    path::Path,
};
use tar::Archive;
use zip::ZipArchive;

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
    #[error("Could not unzip file")]
    Unzip(#[from] zip::result::ZipError),
}

/// Downloads the ngrok binary to a path.
pub async fn to_path(output: &Path) -> Result<(), DownloadError> {
    to_write(&mut File::create(output)?).await
}

/// Downloads the ngrok binary to a writable.
///
/// There are no cache options as ngrok doesn't have any versioning. If you're bundling this with an application, it's reccomended to download this whenever needed.
pub async fn to_write(output: &mut impl Write) -> Result<(), DownloadError> {
    // Download the ngrok binary
    let mut bytes = reqwest::get(NGROK_PATH).await?.bytes().await?.reader();

    // Unzip it
    return match Path::new(NGROK_PATH).extension().and_then(OsStr::to_str) {
        Some(ext) if ext == "zip" => {
            let mut tempfile = tempfile::tempfile()?;
            copy(&mut bytes, &mut tempfile)?;

            let mut archive = ZipArchive::new(tempfile)?;

            copy(&mut archive.by_index(0)?, output)?;

            Ok(())
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

            copy(&mut x, output)?;

            Ok(())
        }
        Some(ext) => Err(DownloadError::UnsupportedFileExtension(ext.to_string())),
        None => Err(DownloadError::NoFileExtension),
    };
}
