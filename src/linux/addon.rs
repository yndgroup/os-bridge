#[cfg(test)]
mod tests {
  use crate::common::addon::add;
  // test c++ function
  #[test]
  fn test_add() {
    unsafe {
      let result = add(3, 4);
      println!("3 + 4 = {}", result); // out 7
    }
  }
}