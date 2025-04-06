#[allow(unused)]
pub struct RegisterTableParams {
  pub key_path: String,
  pub key_name: String,
}

impl RegisterTableParams {
  #[allow(unused)]
  pub fn new(key_path: &str, key_name: &str) -> Self {
    RegisterTableParams {
      key_path: key_path.to_string(),
      key_name: key_name.to_string(),
    }
  }
}