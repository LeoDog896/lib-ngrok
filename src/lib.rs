use std::{io, path::Path, process::Command, ffi::OsStr};
use thiserror::Error;

pub mod url;

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

pub fn run<T: AsRef<OsStr>>(bin: &Path, auth_token: &str, args: &[T]) -> Result<(), RunErorr> {
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
