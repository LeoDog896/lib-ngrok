use std::{io, path::Path, process::Command};
use thiserror::Error;

pub mod url;

#[cfg(feature = "cache")]
pub mod cache;

#[cfg(feature = "download")]
pub mod download;

#[derive(Error, Debug)]
pub enum RunErorr {
    #[error("could not run ngrok")]
    CouldNotRun,
    #[error("could not find ngrok binary")]
    CouldNotFindBinary,
    #[error("io")]
    IO(#[from] io::Error),
}

pub fn run(bin: &Path, auth_token: &str, args: &[&str]) -> Result<(), RunErorr> {
    let mut child = Command::new(bin)
        .env("NGROK_AUTHTOKEN", auth_token)
        .args(args)
        .spawn()?;
    let status = child.wait()?;

    if !status.success() {
        return Err(RunErorr::CouldNotRun);
    }
    Ok(())
}
