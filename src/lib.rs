//! Easily download and run an ngrok binary.
//!
//! # Examples
//!
//! ```rust
//! use lib_ngrok::download;
//!
//! async fn download() -> Result<(), anyhow::Error> {
//!     let ngrok_path = &Path::new("ngrok");
//!     download::bin(ngrok_path).await?;
//! }
//! ```

use std::{
    ffi::OsStr,
    io,
    path::Path,
    process::{Command, ExitStatus},
};
use thiserror::Error;

pub mod url;

#[cfg(feature = "download")]
pub mod download;

/// Errors that can occur when running an ngrok binary.
#[derive(Error, Debug)]
pub enum RunError {
    #[error("could not run ngrok. status: {0}")]
    CouldNotRun(ExitStatus),
    #[error("could not find ngrok binary")]
    CouldNotFindBinary,
    #[error("io")]
    IO(#[from] io::Error),
}

/// Runs an ngrok binary with an authentication token.
///
/// Make sure that the bin is an ngrok path.
///
/// The `auth_token` parameter sets an enviornemnt variable `NGROK_AUTHTOKEN` to what you specify.
pub fn run<T: AsRef<OsStr>>(bin: &Path, auth_token: &str, args: &[T]) -> Result<(), RunError> {
    let mut child = Command::new(bin)
        .env("NGROK_AUTHTOKEN", auth_token)
        .args(args)
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        return Err(RunError::CouldNotRun(status));
    }

    Ok(())
}
