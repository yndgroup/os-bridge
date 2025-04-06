#[cfg(test)]
mod tests {
  use crate::common::addon::add;
  // 测试c++中的函数
  #[test]
  fn test_add() {
    unsafe {
      let result = add(3, 4);
      println!("3 + 4 = {}", result); // 输出 7
    }
  }
}
