#[cfg(test)]
mod tests {

  use std::time::Instant;

  use crate::{core::Zip, Unzip};

  #[test]
  fn test_get_size() {
    let zip = Unzip::new(Some("123456".to_string()));
    let ret = zip.calculate_size("addon");
    println!("ret = {:?}", ret);
    // let _ = zip.extract("zip/compress_folder.zip", "zip/compress_folder");
  }

  #[test]
  fn test_compress_folder() {
    let start = Instant::now();
    let zip = Unzip::new(Some("123456".to_string()));
    let ret = zip.single_folder_compression(
      "/Users/yndgroup/2025/code/bak/docs-server",
      "zip/compress_folder.zip",
    );
    println!("ret = {:?}", ret);
    let duration = start.elapsed();
    println!("执行耗时: {:?}", duration);
    // let _ = zip.extract("zip/compress_folder.zip", "zip/compress_folder");
  }

  #[test]
  fn test_compress_multiple() {
    let start = Instant::now();
    let zip = Unzip::new(Some("123456".to_string()));
    let ret = zip.batch_compression(
      vec![
        "build.rs",
        "addon",
        "/Users/yndgroup/2025/code/bak/docs-server",
      ],
      "zip/multiple.zip",
    );
    println!("ret = {:?}", ret);
    let duration = start.elapsed();
    println!("执行耗时: {:?}", duration);
    // let _ = zip.extract("zip/multiple.zip", "zip/multiple");
  }

  #[test]
  fn test_compress_file() {
    // let start = Instant::now();
    // let zip = Unzip::new(Some("123456".to_string()));
    // let ret = zip.compress_file(
    //   "zip-resources/通过例子学Rust中文版.pdf",
    //   "zip/通过例子学Rust中文版.zip",
    // );
    // println!("ret = {:?}", ret);
    // let duration = start.elapsed();
    // println!("执行耗时: {:?}", duration);
    // let _ = zip.extract("zip/通过例子学Rust中文版.zip", "zip/通过例子学Rust中文版");

    let start = Instant::now();
    let zip = Unzip::new(Some("123456".to_string()));
    let ret = zip.file_compression("zip-resources/三皇五帝的传说.mp4", "zip/三皇五帝的传说.zip");
    println!("ret = {:?}", ret);
    let duration = start.elapsed();
    println!("执行耗时: {:?}", duration);
    let _ = zip.extract("zip/三皇五帝的传说.zip", "zip/三皇五帝的传说");
  }
}
