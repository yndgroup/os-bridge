#[cfg(test)]
mod tests {

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
    let zip = Unzip::new(Some("123456".to_string()));
    let ret = zip.compress_folder("addon", "zip/compress_folder.zip");
    println!("ret = {:?}", ret);
    // let _ = zip.extract("zip/compress_folder.zip", "zip/compress_folder");
  }

  #[test]
  fn test_compress_multiple() {
    let zip = Unzip::new(Some("123456".to_string()));
    let ret = zip.compress_multiple(vec!["empty", "build.rs", "addon"], "zip/multiple.zip");
    println!("ret = {:?}", ret);

    // let _ = zip.extract("zip/multiple.zip", "zip/multiple");
  }

  #[test]
  fn test_compress_file() {
    let zip = Unzip::new(Some("123456".to_string()));
    let ret = zip.compress_file("build.rs", "zip/zip_file.zip");
    println!("ret = {:?}", ret);
    // let _ = zip.extract("zip/zip_file.zip", "zip/zip_file");
  }
}
