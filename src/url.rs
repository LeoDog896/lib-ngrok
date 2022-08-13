//! Download URLs for ngrok per-architecture

// Mac
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub static NGROK_PATH: &str =
    "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-darwin-amd64.zip";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub static NGROK_PATH: &str =
    "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-darwin-arm64.zip";

// Windows
#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
pub static NGROK_PATH: &str =
    "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-windows-amd64.zip";
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
pub static NGROK_PATH: &str =
    "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-windows-386.zip";

// Linux
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub static NGROK_PATH: &str =
    "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-linux-amd64.tgz";

#[cfg(all(target_os = "linux", target_arch = "arrch64"))]
pub static NGROK_PATH: &str =
    "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-linux-arm64.tgz";

#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub static NGROK_PATH: &str = "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-linux-arm.tgz";

#[cfg(all(target_os = "linux", target_arch = "mips"))]
pub static NGROK_PATH: &str = "https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-linux-mips.tgz";
