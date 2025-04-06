use crate::BridgeResult;

pub trait OsBridge {
  fn get_pid(&self) -> BridgeResult<u32>;
}
