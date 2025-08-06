extern crate walkdir;
extern crate zip;
use super::core::Zip;
use super::errors::BridgeError;
use crate::BridgeResult;
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};
use std::str;
use walkdir::WalkDir;
use zip::read::ZipArchive;
use zip::result::ZipError;
use zip::unstable::write::FileOptionsExt;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub struct Unzip {
  pub zip_pwd: Option<String>,
}

impl Unzip {
  pub fn new(zip_pwd: Option<String>) -> Self {
    Self { zip_pwd: zip_pwd }
  }
}

impl Zip for Unzip {
  /// extract
  fn extract<P>(&self, input_path: P, output_dir: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>,
  {
    if !input_path.as_ref().exists() {
      return Err(BridgeError::WithMsg(format!(
        "{} is not exist",
        input_path.as_ref().display()
      )));
    }

    // check output parent is exist
    if let Some(parent) = output_dir.as_ref().parent() {
      if !parent.exists() {
        fs::create_dir_all(parent)
          .map_err(|err| BridgeError::WithMsg(format!("{}", err.to_string())))?;
      }
    }

    let file = File::open(input_path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    let mut archive =
      ZipArchive::new(BufReader::new(file)).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
    match &self.zip_pwd {
      Some(pwd) => {
        for i in 0..archive.len() {
          let mut file = match archive.by_index_decrypt(i, pwd.as_bytes()) {
            Ok(f) => f,
            Err(ZipError::InvalidPassword) => {
              return Err(BridgeError::WithMsg(
                "Password error, unable to decompress file!".to_string(),
              ))
            }
            Err(e) => return Err(BridgeError::WithMsg(e.to_string())),
          };
          let outpath = output_dir.as_ref().join(file.name());
          if file.is_dir() {
            std::fs::create_dir_all(&outpath).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
          } else {
            if let Some(parent) = outpath.parent() {
              if !parent.exists() {
                std::fs::create_dir_all(parent)
                  .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
              }
            }
            let mut outfile =
              File::create(&outpath).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
            io::copy(&mut file, &mut outfile).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
          }
        }
      }
      None => {
        for i in 0..archive.len() {
          let mut file = archive
            .by_index(i)
            .map_err(|e| {
              let msg = e.to_string();
              if msg.contains("Password required") {
                return BridgeError::WithMsg("Password must be used for decryption!".to_string());
              }
              return BridgeError::WithMsg(e.to_string());
            })
            .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
          let outpath = output_dir.as_ref().join(file.name());
          if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
          } else {
            if let Some(parent) = outpath.parent() {
              if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
              }
            }
            let mut outfile =
              File::create(&outpath).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
            io::copy(&mut file, &mut outfile).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
          }
        }
      }
    }
    Ok(true)
  }

  /// Pay special attention to compressing files in zip format, as it is a single file, not a folder
  fn compress_file<P: AsRef<Path>>(&self, input_path: P, out_path: P) -> BridgeResult<bool> {
    //  check if the path exists
    let new_src_path = input_path.as_ref();
    if !new_src_path.is_file() {
      return Err(BridgeError::WithMsg(format!(
        "{} is not a file",
        new_src_path.display()
      )));
    }
    // check if the destination path exists
    let new_dst_path = out_path.as_ref();
    if !new_dst_path.exists() {
      match new_dst_path.parent() {
        Some(p) => {
          fs::create_dir_all(p).map_err(|err| {
            BridgeError::WithMsg(format!("Failed to create directory: {}", err.to_string()))
          })?;
        }
        None => {}
      }
    }
    // write file
    let file = fs::File::create(new_dst_path).map_err(|e| BridgeError::WithMsg(e.to_string()))?;
    let mut zip = ZipWriter::new(file);
    let mut options: FileOptions<'_, ()> =
      FileOptions::default().compression_method(CompressionMethod::DEFLATE);
    if self.zip_pwd.is_some() {
      options = options.with_deprecated_encryption(
        &self
          .zip_pwd
          .clone()
          .unwrap_or(Default::default())
          .to_string()
          .as_bytes()
          .to_vec(),
      );
    }
    match new_src_path.file_name() {
      Some(src_file_name) => {
        if let Some(s) = src_file_name.to_os_string().to_str() {
          zip
            .start_file(s, options)
            .map_err(|e| BridgeError::WithMsg(e.to_string()))?;
          let src_file_content = fs::read(input_path)
            .map_err(|err| {
              BridgeError::WithMsg(format!(
                "file read failure, error info: {}",
                err.to_string()
              ))
            })
            .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
          zip
            .write_all(&src_file_content)
            .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
          zip
            .finish()
            .map_err(|e| BridgeError::WithMsg(e.to_string()))?;
        } else {
          return Err(BridgeError::WithMsg("file name does not exist".to_string()));
        }
      }
      None => return Err(BridgeError::WithMsg("file name does not exist".to_string())),
    }
    Ok(true)
  }

  // Compressed Folder
  fn compress_folder<P: AsRef<Path>>(
    &self,
    zip_input_path: P,
    zip_out_path: P,
  ) -> BridgeResult<bool> {
    if !zip_input_path.as_ref().exists() {
      return Err(BridgeError::WithMsg("folder is not exist".to_string()));
    }
    if !zip_input_path.as_ref().is_dir() {
      return Err(BridgeError::WithMsg(format!(
        "{} is not a folder",
        zip_input_path.as_ref().display()
      )));
    }
    if let Some(zip_out_path) = zip_out_path.as_ref().parent() {
      if !zip_out_path.exists() {
        std::fs::create_dir_all(zip_out_path)
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      }
    }
    let zip_file =
      File::create(zip_out_path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    let mut zip_writer = ZipWriter::new(zip_file);
    match zip_input_path.as_ref().parent() {
      Some(dir) => {
        self
          .add_dir_to_zip(dir, zip_input_path.as_ref(), &mut zip_writer)
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        zip_writer
          .finish()
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        return Ok(true);
      }
      None => {
        return Err(BridgeError::WithMsg("file name does not exist".to_string()));
      }
    }
  }

  /// Compress multiple files
  fn compress_multiple<P>(&self, zip_input_paths: Vec<P>, zip_out_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>,
  {
    if zip_input_paths.is_empty() {
      return Err(BridgeError::WithMsg(
        "The input_paths cannot be empty".to_string(),
      ));
    }
    if let Some(out_path) = zip_out_path.as_ref().parent() {
      if !out_path.exists() {
        std::fs::create_dir_all(out_path)?;
      }
    }
    // let paths_to_zip: Vec<PathBuf> = zip_input_paths.iter().map(|s| PathBuf::from(s)).collect();
    let mut buffer = io::Cursor::new(Vec::new());
    let mut zip_writer = ZipWriter::new(&mut buffer);
    for path in zip_input_paths {
      self
        .add_path_to_zip(&mut zip_writer, &path)
        .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    }
    zip_writer
      .finish()
      .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    let compressed_data = buffer.into_inner();
    fs::write(zip_out_path, &compressed_data)
      .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    Ok(true)
  }

  /// Recursive compression folder
  fn add_dir_to_zip<P: AsRef<Path>>(
    &self,
    base_dir: P,
    current_dir: P,
    zip_writer: &mut ZipWriter<File>,
  ) -> BridgeResult<bool> {
    let current_dir = current_dir.as_ref();
    let mut options: FileOptions<'_, ()> = FileOptions::default()
      .compression_method(CompressionMethod::Stored)
      .unix_permissions(0o755);
    if self.zip_pwd.is_some() {
      options = options.with_deprecated_encryption(
        &self
          .zip_pwd
          .clone()
          .unwrap_or(Default::default())
          .to_string()
          .as_bytes()
          .to_vec(),
      );
    }
    for entry in fs::read_dir(current_dir).map_err(|err| BridgeError::WithMsg(err.to_string()))? {
      let entry = entry.map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      let entry_path = entry.path();
      let relative_path = entry_path
        .strip_prefix(base_dir.as_ref())
        .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      if entry_path.is_dir() {
        zip_writer
          .add_directory(
            relative_path.to_str().unwrap_or(Default::default()),
            options,
          )
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        self
          .add_dir_to_zip(base_dir.as_ref(), &entry_path, zip_writer)
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      } else {
        let mut file =
          File::open(&entry_path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        zip_writer
          .start_file(
            relative_path.to_str().unwrap_or(Default::default()),
            options,
          )
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        io::copy(&mut file, zip_writer).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      }
    }
    Ok(true)
  }

  /// Determine if the directory is empty
  fn is_empty_directory<P: AsRef<Path>>(&self, path: P) -> BridgeResult<bool> {
    let path = path.as_ref();
    if path.is_dir() {
      let entries = fs::read_dir(path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      for entry in entries {
        let entry = entry.map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        if entry
          .file_type()
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?
          .is_file()
          || entry
            .file_type()
            .map_err(|err| BridgeError::WithMsg(err.to_string()))?
            .is_dir()
        {
          return Ok(false);
        }
      }
      Ok(true)
    } else {
      Err(BridgeError::WithMsg(
        "Provided path is not a directory".to_string(),
      ))
    }
  }

  /// Add path to zip file
  fn add_path_to_zip<P: AsRef<Path>>(
    &self,
    zip_writer: &mut ZipWriter<&mut io::Cursor<Vec<u8>>>,
    path: P,
  ) -> BridgeResult<bool> {
    let path = path.as_ref();
    if path.is_file() {
      let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(Default::default());
      let mut options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);
      if self.zip_pwd.is_some() {
        options = options.with_deprecated_encryption(
          &self
            .zip_pwd
            .clone()
            .unwrap_or(Default::default())
            .to_string()
            .as_bytes()
            .to_vec(),
        );
      }
      zip_writer
        .start_file(file_name, options)
        .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      let mut file = fs::File::open(path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      io::copy(&mut file, zip_writer).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    } else if path.is_dir() {
      let prefix = path.parent().unwrap_or(Path::new(""));
      self
        .add_absolute_dir_to_zip(zip_writer, path, prefix)
        .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    }
    Ok(true)
  }

  // Recursively add directory to zip file
  fn add_absolute_dir_to_zip<P: AsRef<Path>>(
    &self,
    zip_writer: &mut ZipWriter<&mut io::Cursor<Vec<u8>>>,
    path: P,
    prefix: &Path,
  ) -> BridgeResult<bool> {
    let path = path.as_ref();
    let mut options: FileOptions<'_, ()> = FileOptions::default()
      .compression_method(CompressionMethod::Stored)
      .unix_permissions(0o755);
    if self.zip_pwd.is_some() {
      options = options.with_deprecated_encryption(
        &self
          .zip_pwd
          .clone()
          .unwrap_or(Default::default())
          .to_string()
          .as_bytes()
          .to_vec(),
      );
    }
    let entries = fs::read_dir(path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    for entry in entries {
      let entry = entry.map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      let path = entry.path();
      let rel_path = path
        .strip_prefix(prefix)
        .unwrap()
        .to_str()
        .unwrap_or(Default::default())
        .replace("\\", "/");
      if path.is_file() {
        zip_writer
          .start_file(rel_path, options)
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        let mut file = fs::File::open(path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        io::copy(&mut file, zip_writer).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      } else if path.is_dir() {
        match self.is_empty_directory(path.clone()) {
          Ok(bool) => {
            if bool {
              zip_writer
                .add_directory(rel_path, options)
                .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
              return Ok(true);
            }
          }
          Err(_) => (),
        };
        self
          .add_absolute_dir_to_zip(zip_writer, path, prefix)
          .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      }
    }
    if path.is_dir() {
      match self.is_empty_directory(path) {
        Ok(bool) => {
          if bool {
            let rel_path = path
              .strip_prefix(prefix)
              .unwrap()
              .to_str()
              .unwrap_or(Default::default())
              .replace("\\", "/");
            zip_writer
              .add_directory(rel_path, options)
              .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
          }
        }
        _ => (),
      }
    }
    Ok(true)
  }

  ///  Calculate file size
  fn calculate_size<P>(&self, path: P) -> BridgeResult<f64>
  where
    P: AsRef<Path>,
  {
    let mut paths = vec![path.as_ref().to_path_buf()]; // 改为 PathBuf
    let mut res_size = 0u64;
    while let Some(path) = paths.pop() {
      let meta =
        std::fs::symlink_metadata(&path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      let file_type = meta.file_type();
      if file_type.is_dir() {
        let entries =
          std::fs::read_dir(path).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
        for entry in entries {
          let entry = entry.map_err(|err| BridgeError::WithMsg(err.to_string()))?;
          let path = entry.path();
          paths.push(path);
        }
      }
      if file_type.is_file() {
        res_size += meta.len();
      }
    }
    Ok(res_size as f64)
  }
  /// Read file size
  fn get_size<P>(&self, path: P) -> u64
  where
    P: AsRef<Path>,
  {
    let total_size = WalkDir::new(path)
      .min_depth(1)
      .max_depth(3)
      .into_iter()
      .filter_map(|entry| entry.ok())
      .filter_map(|entry| entry.metadata().ok())
      .filter(|metadata| metadata.is_file())
      .fold(0, |acc, m| acc + m.len());
    total_size
  }
}
