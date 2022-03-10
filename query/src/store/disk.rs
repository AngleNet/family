use std::fs::File;
use crate::fail;

pub type PageIdType = i64;

/// Manages database files including data and logs.
pub struct DiskManager {
    db_file: String,
    database: File,
    log: File,
}

impl DiskManager {
    pub fn new(name: &str) -> DiskManager {
        // name format: {path}/{database name}.db; {path}/{database name}.log
        if name.is_empty() {
            fail!("test");
        }
        todo!()
    }

    pub fn write_page(page_id: PageIdType, data: &[u8]) {}

    pub fn read_page(page_id: PageIdType, data: &mut [u8]) {}

    pub fn write_log(data: &[u8]) {}

    pub fn read_log(offset: usize, data: &mut [u8]) {}
}
