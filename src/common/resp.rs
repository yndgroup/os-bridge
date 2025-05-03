#[derive(Debug, Clone)]
pub struct Resp<T> {
  pub code: i32,
  pub msg: String,
  pub data: Option<T>,
}

impl<T> Resp<T> {
  pub fn new(code: i32, msg: &str, data: Option<T>) -> Self {
    Resp {
      code: code,
      msg: msg.to_string(),
      data: data,
    }
  }

  pub fn success(msg: &str, data: Option<T>) -> Self {
    Resp {
      code: 1,
      msg: msg.to_string(),
      data: data,
    }
  }

  pub fn success_with_msg(msg: &str) -> Self {
    Resp {
      code: 1,
      msg: msg.to_string(),
      data: None,
    }
  }

  pub fn fail(msg: &str, data: Option<T>) -> Self {
    Resp {
      code: 0,
      msg: msg.to_string(),
      data: data,
    }
  }

  pub fn fail_with_msg(msg: &str) -> Self {
    Resp {
      code: 0,
      msg: msg.to_string(),
      data: None,
    }
  }
}
