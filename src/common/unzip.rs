use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::read::ZipArchive;
use zip::result::ZipError;
use zip::unstable::write::FileOptionsExt;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::core::Zip;
use crate::errors::BridgeError;
use crate::BridgeResult;

pub struct Unzip {
  pub zip_pwd: Option<String>,
}

impl Unzip {
  pub fn new(zip_pwd: Option<String>) -> Self {
    Self { zip_pwd: zip_pwd }
  }
}

impl Zip for Unzip {
  /// @description:  Decompression extractor
  /// @parama zip_src_path: Decompression target path
  /// @parama zip_output_path: Output Path
  fn extract<P>(&self, zip_input_path: P, zip_output_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>,
  {
    if !zip_input_path.as_ref().exists() {
      return Err(BridgeError::from(format!(
        "The file '{}' does not exist",
        zip_input_path.as_ref().display()
      )));
    }
    if let Some(parent) = zip_output_path.as_ref().parent() {
      if !parent.exists() {
        fs::create_dir_all(parent).map_err(|err| BridgeError::from(err.to_string()))?;
      }
    }
    let file = File::open(zip_input_path)?;
    let mut archive =
      ZipArchive::new(BufReader::new(file)).map_err(|err| BridgeError::from(err.to_string()))?;
    match &self.zip_pwd {
      Some(pwd) => {
        for i in 0..archive.len() {
          let mut file = match archive.by_index_decrypt(i, pwd.as_bytes()) {
            Ok(f) => f,
            Err(ZipError::InvalidPassword) => {
              return Err(BridgeError::from(
                "Password error, unable to decompress the file!",
              ));
            }
            Err(err) => return Err(BridgeError::from(err.to_string())),
          };
          let outpath = Path::new(zip_output_path.as_ref()).join(file.name());
          if file.is_dir() {
            std::fs::create_dir_all(&outpath).map_err(|err| BridgeError::from(err.to_string()))?;
          } else {
            if let Some(parent) = outpath.parent() {
              if !parent.exists() {
                std::fs::create_dir_all(parent)?;
              }
            }
            let mut outfile =
              File::create(&outpath).map_err(|err| BridgeError::from(err.to_string()))?;
            io::copy(&mut file, &mut outfile).map_err(|err| BridgeError::from(err.to_string()))?;
          }
        }
      }
      None => {
        for i in 0..archive.len() {
          let mut file = archive
            .by_index(i)
            .map_err(|err| BridgeError::from(err.to_string()))?;
          let outpath = Path::new(zip_output_path.as_ref()).join(file.name());
          if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|err| BridgeError::from(err.to_string()))?;
          } else {
            if let Some(parent) = outpath.parent() {
              if !parent.exists() {
                std::fs::create_dir_all(parent)
                  .map_err(|err| BridgeError::from(err.to_string()))?;
              }
            }
            let mut outfile =
              File::create(&outpath).map_err(|err| BridgeError::from(err.to_string()))?;
            io::copy(&mut file, &mut outfile).map_err(|err| BridgeError::from(err.to_string()))?;
          }
        }
      }
    }

    Ok(true)
  }

  /// @description:  Single file compression
  /// @parama zip_input_path: Compression target path
  /// @parama zip_output_path: Output Path
  fn file_compression<P>(&self, zip_input_path: P, zip_output_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>,
  {
    if !zip_input_path.as_ref().is_file() {
      return Err(BridgeError::from("Please input the file path".to_string()));
    }
    if let Some(parent) = zip_output_path.as_ref().parent() {
      if !parent.exists() {
        fs::create_dir_all(parent).map_err(|err| BridgeError::from(err.to_string()))?;
      }
    }
    let file = fs::File::create(zip_output_path.as_ref())
      .map_err(|err| BridgeError::from(err.to_string()))?;
    let mut zip = ZipWriter::new(file);
    let mut options: FileOptions<'_, ()> = FileOptions::default()
      .compression_method(CompressionMethod::Deflated)
      .compression_level(Some(9))
      .large_file(true)
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
    match zip_input_path.as_ref().file_name() {
      Some(file_name) => {
        if let Some(s) = file_name.to_os_string().to_str() {
          zip
            .start_file(s, options)
            .map_err(|e| BridgeError::from(format!("{}", e.to_string())))?;
          let src_file_content = fs::read(zip_input_path.as_ref())
            .map_err(|err| BridgeError::from(format!("File read error: {}", err.to_string())))?;
          zip.write_all(&src_file_content)?;
          zip
            .finish()
            .map_err(|err| BridgeError::from(err.to_string()))?;
        } else {
          return Err(BridgeError::from("The file does not exist"));
        }
      }
      None => return Err(BridgeError::from("The file does not exist")),
    }
    Ok(true)
  }

  /// @description:  Single folder compression
  /// @parama zip_input_path: Compression target path
  /// @parama zip_output_path: Output Path
  fn single_folder_compression<P: AsRef<Path>>(
    &self,
    zip_input_path: P,
    zip_output_path: P,
  ) -> BridgeResult<bool> {
    if !zip_input_path.as_ref().exists() {
      return Err(BridgeError::from(format!(
        "The folder path '{}' does not exist",
        zip_input_path.as_ref().display()
      )));
    }
    if let Some(parent) = zip_output_path.as_ref().parent() {
      if !parent.exists() {
        fs::create_dir_all(parent).map_err(|err| BridgeError::from(err.to_string()))?;
      }
    }
    let zip_file = File::create(zip_output_path)?;
    let mut zip_writer = ZipWriter::new(zip_file);
    let base_dir = zip_input_path.as_ref().parent().unwrap_or(Path::new(""));
    self
      .add_dir(&mut zip_writer, base_dir, zip_input_path.as_ref())
      .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    zip_writer
      .finish()
      .map_err(|err| BridgeError::WithMsg(err.to_string()))?;
    Ok(true)
  }

  /// batch compression
  /// Please provide a list path
  /// @parama zip_input_paths: [zip_input_path1, zip_input_path2, ...]
  /// @parama zip_output_path: Output Path
  fn batch_compression<P>(&self, zip_input_paths: Vec<P>, zip_output_path: P) -> BridgeResult<bool>
  where
    P: AsRef<Path>,
  {
    if zip_input_paths.is_empty() {
      return Err(BridgeError::from("The file path cannot be empty"));
    }
    if let Some(parent) = zip_output_path.as_ref().parent() {
      if !parent.exists() {
        std::fs::create_dir_all(parent).map_err(|err| BridgeError::WithMsg(err.to_string()))?;
      }
    }
    for p in &zip_input_paths {
      if !p.as_ref().exists() {
        return Err(BridgeError::from(format!(
          "The path '{}' does not exist",
          p.as_ref().display()
        )));
      }
    }
    let zip_file = File::create(zip_output_path)?;
    let mut zip_writer = ZipWriter::new(zip_file);
    for path in zip_input_paths {
      self
        .add_path(&mut zip_writer, &path)
        .map_err(|err| BridgeError::from(err.to_string()))?;
    }
    zip_writer
      .finish()
      .map_err(|err| BridgeError::from(err.to_string()))?;
    Ok(true)
  }

  /// @description:  Add directory
  /// @parama zip_writer: ZipWriter
  /// @parama base_dir: Directory path
  /// @parama current_dir: Base directory
  fn add_dir<P: AsRef<Path>>(
    &self,
    zip_writer: &mut ZipWriter<File>,
    base_dir: P,
    current_dir: P,
  ) -> BridgeResult<bool> {
    if !current_dir.as_ref().exists() {
      return Err(BridgeError::from(format!(
        "The path '{}' does not exist",
        current_dir.as_ref().display()
      )));
    }
    for entry in fs::read_dir(current_dir).map_err(|err| BridgeError::from(err.to_string()))? {
      let entry = entry.map_err(|err| BridgeError::from(err.to_string()))?;
      let entry_path = entry.path();
      let relative_path = entry_path
        .strip_prefix(base_dir.as_ref())
        .map_err(|err| BridgeError::from(err.to_string()))?;
      let zip_path = format!("{}", relative_path.display()).replace("\\", "/");
      if entry_path.is_dir() {
        let mut options: FileOptions<'_, ()> = FileOptions::default()
          .compression_method(CompressionMethod::Deflated)
          .large_file(true)
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
          .add_directory(zip_path, options)
          .map_err(|err| BridgeError::from(err.to_string()))?;
        self
          .add_dir(zip_writer, base_dir.as_ref(), &entry_path)
          .map_err(|err| BridgeError::from(err.to_string()))?;
      } else {
        // 添加文件
        let mut options: FileOptions<'_, ()> = FileOptions::default()
          .compression_method(CompressionMethod::Deflated)
          .large_file(true)
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
        let mut file = File::open(&entry_path)?;
        zip_writer
          .start_file(zip_path, options)
          .map_err(|err| BridgeError::from(err.to_string()))?;
        io::copy(&mut file, zip_writer)?;
      }
    }
    Ok(true)
  }

  /// @description: Add a path to the zip file
  /// @parama zip_writer: zip_writer
  /// @parama path: Path
  fn add_path<P: AsRef<Path>>(
    &self,
    zip_writer: &mut ZipWriter<File>,
    path: P,
  ) -> BridgeResult<bool> {
    if path.as_ref().is_file() {
      let file_name = path
        .as_ref()
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(Default::default());
      let mut options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .large_file(true)
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
        .map_err(|err| BridgeError::new(&err.to_string()))?;
      let mut file = fs::File::open(path).map_err(|err| BridgeError::new(&err.to_string()))?;
      io::copy(&mut file, zip_writer).map_err(|err| BridgeError::new(&err.to_string()))?;
    } else if path.as_ref().is_dir() {
      let path_ref = path.as_ref();
      if self
        .check_folder_is_empty(path_ref)
        .map_err(|err| BridgeError::new(&err.to_string()))?
      {
        let create_path = path_ref
          .strip_prefix(path.as_ref().parent().unwrap_or(Path::new("")))
          .unwrap_or(Path::new(""))
          .to_str()
          .unwrap_or_default()
          .replace("\\", "/");
        let mut options: FileOptions<'_, ()> = FileOptions::default()
          .compression_method(CompressionMethod::Deflated)
          .large_file(true)
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
        let out_path = create_path.replace("\\", "/");
        zip_writer
          .add_directory(out_path, options)
          .map_err(|err| BridgeError::new(&err.to_string()))?;
      } else {
        let prefix = path_ref.parent().unwrap_or(Path::new(""));
        self.add_dir(zip_writer, prefix, path.as_ref())?;
      }
    }
    Ok(true)
  }

  /// @description Check whether the folder is empty
  /// @parama path: Path
  fn check_folder_is_empty<P: AsRef<Path>>(&self, path: P) -> BridgeResult<bool> {
    let path = path.as_ref();
    if path.is_dir() {
      let entries = fs::read_dir(path)?;
      for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_file() || entry.file_type()?.is_dir() {
          return Ok(false);
        }
      }
      Ok(true)
    } else {
      return Err(BridgeError::new("Path is not a directory"));
    }
  }

  fn calculate_size<P>(&self, path: P) -> BridgeResult<f64>
  where
    P: AsRef<Path>,
  {
    let mut paths = vec![path.as_ref().to_path_buf()];
    let mut res_size = 0u64;
    while let Some(path) = paths.pop() {
      let meta =
        std::fs::symlink_metadata(&path).map_err(|err| BridgeError::from(err.to_string()))?;
      let file_type = meta.file_type();
      if file_type.is_dir() {
        let entries = std::fs::read_dir(path).map_err(|err| BridgeError::from(err.to_string()))?;
        for entry in entries {
          let entry = entry.map_err(|err| BridgeError::from(err.to_string()))?;
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
