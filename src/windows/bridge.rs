use crate::{common::{core::OsBridge, utils}, BridgeResult};

pub struct Bridge {}

impl Bridge {

    // 关联函数
    #[allow(unused)]
    pub fn new() -> Self {
        Self {}
    }

}

// 实现 OsBridge trait
impl OsBridge for Bridge {
    fn get_pid(&self) -> BridgeResult<u32> {
       Ok(utils::get_pid()?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pid() {
        let bd = Bridge::new();
        println!("{:?}", bd.get_pid());
    }
}