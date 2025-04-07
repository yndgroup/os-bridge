mod common;

pub use common::*;
pub use common::core::OsBridge;
pub use common::unzip::Unzip;

type BridgeResult<T> = std::result::Result<T, errors::BridgeError>;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub  use macos::*;


#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod linux;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use linux::*;