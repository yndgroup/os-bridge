use rand::Rng;

use crate::{common::errors::BridgeError, BridgeResult};

/// 获取进程的PID 支持windows macos linux
pub fn get_pid() -> BridgeResult<u32> {
    let pid = sysinfo::get_current_pid().map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    Ok(pid.as_u32())
}


// 生成随机字符串
#[allow(unused)]
pub fn random_string(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
  abcdefghijklmnopqrstuvwxyz\
  0123456789)(*&^%$#@!~";
    let mut rng = rand::rng();
    let password: String = (0..len)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    return password;
}

// 生成16位的随机密码
#[allow(unused)]
pub fn get_random_key16() -> BridgeResult<[u8; 16]> {
    let mut arr: [u8; 16] = [0u8; 16];
    rand::rng().fill(&mut arr[..]);
    Ok(arr)
}

// 将数组转换为字符串
#[allow(unused)]
fn array_to_string(arr: [u8; 16]) -> String {
    let mut s = String::new();
    for &byte in &arr {
        if byte == 0 {
            break;
        }
        s.push(byte as char);
    }
    s
}