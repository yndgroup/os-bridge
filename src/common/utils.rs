use crate::{common::errors::BridgeError, BridgeResult};

/// 获取进程的PID 支持windows macos linux
pub fn get_pid() -> BridgeResult<u32> {
    let pid = sysinfo::get_current_pid().map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    Ok(pid.as_u32())
}