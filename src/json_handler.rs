use std::path::PathBuf;
use crate::{meta_structure::*, io_helper::*};

pub fn encode_json(entry_data: Vec<EntryMeta>) -> std::result::Result<String, Box<dyn std::error::Error>> {
  let encode_data = entry_data;
  let json_str: String = serde_json::to_string(&encode_data).unwrap();
  write_to_file(&data_dir().join("entry_meta.json"), json_str.as_bytes())?;
  return Ok(json_str);
}

pub fn decode_json(path: &PathBuf) -> std::result::Result<Vec<EntryMeta>, serde_json::Error> {
  let json_str: String = String::from_utf8_lossy(&read_from_file(path).unwrap()).to_string(); 
  let decode_data: Vec<EntryMeta> = serde_json::from_str(&json_str)?;
  return Ok(decode_data)
}
