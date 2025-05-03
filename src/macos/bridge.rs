use crate::{common::{core::OsBridge, utils}, BridgeResult};

pub struct Bridge {}

impl Bridge {
    pub fn new() -> Self {
        Self {}
    }

}

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