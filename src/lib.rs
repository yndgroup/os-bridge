pub mod common;

use common::errors::BridgeError;
type BridgeResult<T> = std::result::Result<T, BridgeError>;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod linux;