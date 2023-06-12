use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryMeta {
    pub cab_name: String,
    pub cab_file_name: String,
}
