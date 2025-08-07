extern crate walkdir;
extern crate zip;

use std::{fs::File, path::Path};
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

  /// @description:  Decompression extractor
  /// @parama zip_src_path: Decompression target path
  /// @parama zip_output_path: Output Path
  fn extract<P>(&self, zip_src_path: P, zip_output_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// @description:  Single file compression
  /// @parama zip_input_path: Compression target path
  /// @parama zip_output_path: Output Path
  fn file_compression<P>(&self, zip_input_path: P, zip_output_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// @description:  Single folder compression
  /// @parama zip_input_path: Compression target path
  /// @parama zip_output_path: Output Path
  fn single_folder_compression<P>(
    &self,
    zip_input_path: P,
    zip_output_path: P,
  ) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// @description:  Add directory
  /// @parama zip_writer: ZipWriter
  /// @parama base_dir: Directory path
  /// @parama current_dir: Base directory
  fn add_dir<P>(
    &self,
    zip_writer: &mut ZipWriter<File>,
    base_dir: P,
    current_dir: P,
  ) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// batch compression
  /// Please provide a list path
  /// @parama zip_input_paths: [zip_input_path1, zip_input_path2, ...]
  /// @parama zip_output_path: Output Path
  fn batch_compression<P>(&self, zip_input_paths: Vec<P>, zip_output_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// @description Check whether the folder is empty
  /// @parama path: Path
  fn check_folder_is_empty<P>(&self, path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;

  /// @description: Add a path to the zip file
  /// @parama zip_writer: zip_writer
  /// @parama path: Path
  fn add_path<P>(&self, zip_writer: &mut ZipWriter<File>, path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>;
}
