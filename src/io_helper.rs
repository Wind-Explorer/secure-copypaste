use std::path::PathBuf;
use std::fs::{self, File, ReadDir};
use std::io::{self, Error, BufWriter, prelude::*};
use zip::{ZipArchive, ZipWriter, write::FileOptions};

pub fn data_dir() -> PathBuf {
  let path: PathBuf = directories::BaseDirs::new().unwrap().config_dir().join(env!("CARGO_PKG_NAME"));
  make_new_directory(&path);
  return path;
}

pub fn make_new_directory(path: &PathBuf) {
  if path.exists() {
    return;
  }
  match fs::DirBuilder::new().recursive(true).create(&path) {
    Ok(_) => (),
    Err(reason) => panic!("Could not create directory at {}: {}", path.to_str().unwrap(), reason),
  };
  return;
}

pub fn make_new_file(path: &PathBuf) {
  if path.exists() {
    return;
  }
  match fs::File::create(&path) {
    Ok(_) => (),
    Err(reason) => panic!("Could not create file at {}: {}", path.to_str().unwrap(), reason),
  };
  return;
}

pub fn delete_file(path: &PathBuf) {
  if !path.exists() {
    return;
  }
  match fs::remove_file(path) {
    Ok(()) => return,
    Err(err) => panic!("Error deleting file: {}", err),
  };
}

pub fn write_to_file(path: &PathBuf, contents: &[u8]) -> std::result::Result<(), Error> {
  fs::write(path, contents)?;
  return Ok(());
}

pub fn read_from_file(path: &PathBuf) -> std::result::Result<Vec<u8>, Error> {
  let vec_bytes = fs::read(path)?;
  return Ok(vec_bytes);
}

pub fn files_in_directory(path: &PathBuf) -> ReadDir  {
  return fs::read_dir(path).unwrap();
}

pub fn file_count_in_directory(path: &PathBuf) -> i32 {
  let entries: ReadDir = fs::read_dir(path).expect("Failed to read directory");
  let mut count: i32 = 0;
  for entry in entries {
      if let Ok(entry) = entry {
          let file_type: fs::FileType = entry.file_type().expect("Failed to retrieve file type");
          if file_type.is_file() {
              count += 1;
          }
      }
  }
  return count;
}

pub fn generate_archive(file1_path: &PathBuf, file2_path: &PathBuf, file3_path: &PathBuf, output_file: &PathBuf) -> zip::result::ZipResult<()> {
  let file: File = fs::File::create(output_file)?;
  let mut zip: ZipWriter<BufWriter<File>> = ZipWriter::new(BufWriter::new(file));

  // Compression method: Stored (No compression)
  let options: FileOptions = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
  
  // Add the first file to the archive
  zip.start_file(file1_path.file_name().and_then(|os_str: &std::ffi::OsStr| os_str.to_str()).unwrap(), options)?;
  let mut file1: File = fs::File::open(file1_path)?;
  let mut buffer: Vec<u8> = Vec::new();
  file1.read_to_end(&mut buffer)?;
  zip.write_all(&buffer)?;
  
  // Add the second file to the archive
  zip.start_file(file2_path.file_name().and_then(|os_str: &std::ffi::OsStr| os_str.to_str()).unwrap(), options)?;
  let mut file2: File = fs::File::open(file2_path)?;
  let mut buffer: Vec<u8> = Vec::new();
  file2.read_to_end(&mut buffer)?;
  zip.write_all(&buffer)?;
  
  // Add the third file to the archive
  zip.start_file(file3_path.file_name().and_then(|os_str: &std::ffi::OsStr| os_str.to_str()).unwrap(), options)?;
  let mut file3: File = fs::File::open(file3_path)?;
  let mut buffer: Vec<u8> = Vec::new();
  file3.read_to_end(&mut buffer)?;
  zip.write_all(&buffer)?;
  
  // Finishing up
  zip.finish()?;
  delete_file(file1_path);
  delete_file(file2_path);
  delete_file(file3_path);
  Ok(())
}

pub fn read_from_archive(archive_path: &PathBuf, file_to_read: &str) -> Result<Vec<u8>, Error> {
  let file: File = match File::open(archive_path) {
    Ok(e) => e,
    Err(why) => panic!("Failed to open archive file at \"{}\" ({})", archive_path.display(), why)
  };
  let mut zip: ZipArchive<File> = match ZipArchive::new(file) {
    Ok(e) => e,
    Err(why) => panic!("Failed to open archive ({})", why)
  };
  
  let file_index: usize = zip
      .file_names()
      .position(|name: &str| name == file_to_read)
      .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found in cabinet."))?;
  
  let mut file_in_archive: zip::read::ZipFile = zip.by_index(file_index)?;
  let mut contents: Vec<u8> = Vec::new();
  match file_in_archive.read_to_end(&mut contents) {
    Ok(e) => e,
    Err(why) => panic!("Failed to read file in archive ({})", why),
  };
  Ok(contents)
}
