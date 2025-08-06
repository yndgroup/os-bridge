extern crate walkdir;
extern crate zip;

use std::{fs::File, io, path::Path};
use zip::ZipWriter;

use crate::BridgeResult;

/// OsBridge
pub trait OsBridge {
  fn get_pid(&self) -> BridgeResult<u32>;
}

/// zip
pub trait Zip {
  ///  Calculate file size
  fn calculate_size<P>(&self, path: P) -> BridgeResult<f64>
  where
    P: AsRef<Path>;

  /// Read file size
  fn get_size<P>(&self, path: P) -> u64
  where
    P: AsRef<Path>;

  /// extract
  fn extract<P>(&self, zip_path: P, zip_output_dir: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// Pay special attention to compressing files in zip format, as it is a single file, not a folder
  fn compress_file<P>(&self, zip_input_path: P, zip_out_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// Compressed Folder
  fn compress_folder<P>(&self, zip_input_paths: P, zip_out_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// Recursive compression folder
  fn add_dir_to_zip<P>(
    &self,
    base_dir: P,
    current_dir: P,
    zip_writer: &mut ZipWriter<File>,
  ) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// Compress multiple files
  fn compress_multiple<P>(&self, input_paths: Vec<P>, out_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// Determine if the directory is empty
  fn is_empty_directory<P>(&self, path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// Add path to zip file
  fn add_path_to_zip<P>(
    &self,
    zip_writer: &mut ZipWriter<&mut io::Cursor<Vec<u8>>>,
    path: P,
  ) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  // Recursively add directory to zip file
  fn add_absolute_dir_to_zip<P>(
    &self,
    zip_writer: &mut ZipWriter<&mut io::Cursor<Vec<u8>>>,
    path: P,
    prefix: &Path,
  ) -> BridgeResult<bool>
  where
    P: AsRef<Path>;
}
