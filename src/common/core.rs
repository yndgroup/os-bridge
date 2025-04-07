extern crate walkdir;
extern crate zip;

use std::{fs::File, io, path::Path};
use zip::ZipWriter;

use crate::BridgeResult;

// os trait
pub trait OsBridge {
  fn get_pid(&self) -> BridgeResult<u32>;
}

pub trait Zip {
  ///  Calculate file size
  fn calculate_size(&self, path: &str) -> BridgeResult<f64>;

  /// Read file size
  fn get_size(&self, path: &str) -> u64;

  /// extract
  fn extract(&self, input_path: &str, output_dir: &str) -> BridgeResult<bool>;

  /// Pay special attention to compressing files in zip format, as it is a single file, not a folder
  fn compress_file(&self, src_path: &str, dst_path: &str) -> BridgeResult<bool>;

  /// Compressed Folder
  fn compress_folder<P: AsRef<Path>>(&self, input_paths: P, zip_file: P) -> BridgeResult<bool>;

  /// Recursive compression folder
  fn add_dir_to_zip<P: AsRef<Path>>(
    &self,
    base_dir: P,
    current_dir: P,
    zip_writer: &mut ZipWriter<File>,
  ) -> BridgeResult<bool>;

  /// Compress multiple files
  /// 
  fn compress_multiple(&self, input_paths: Vec<&str>, out_path: &str) -> BridgeResult<bool>;

   /// Determine if the directory is empty
  fn is_empty_directory<P: AsRef<Path>>(&self, path: P) -> BridgeResult<bool>;

  /// Add path to zip file
  fn add_path_to_zip<P: AsRef<Path>>(
    &self,
    zip_writer: &mut ZipWriter<&mut io::Cursor<Vec<u8>>>,
    path: P,
  ) -> BridgeResult<bool>;

  // Recursively add directory to zip file
  fn add_absolute_dir_to_zip<P: AsRef<Path>>(
    &self,
    zip_writer: &mut ZipWriter<&mut io::Cursor<Vec<u8>>>,
    path: P,
    prefix: &Path,
  ) -> BridgeResult<bool>;
}
