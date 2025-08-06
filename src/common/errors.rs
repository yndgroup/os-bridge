use std::error::Error;
use std::fmt::{self};
use std::fmt::{Debug, Display};

#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
  // IO error
  #[error(transparent)]
  Io(#[from] std::io::Error),

  // custom error
  #[allow(unused)]
  #[error("{0}")]
  WithMsg(String),
}

#[allow(unused)]
impl BridgeError {
  pub fn new(msg: &str) -> Self {
    BridgeError::WithMsg(msg.to_string())
  }

  pub fn from(msg: &str) {
    BridgeError::new(msg);
  }
}

#[derive(Debug)]
pub struct ErrMsg {
  pub code: u32,
  pub msg: String,
}

//  AppError ro std::fmt::Display traits
impl fmt::Display for ErrMsg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ErrMsg: {{code:{}，msg :{} }}", self.code, self.msg) // user-facing output
  }
}

#[allow(unused)]
pub fn print_err<E>(e: E)
where
  E: Display + Debug + Error,
{
  print!("{}", e);
}
