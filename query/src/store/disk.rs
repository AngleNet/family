use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::sync::{Arc, RwLock};
use crate::fail;

pub type PageIdType = i64;

/// Manages database files including data and logs.
pub struct DiskManager {
	name: String,
	database: File,
	log: File,
}

impl DiskManager {
	pub fn new(name: &str) -> DiskManager {
		// name format: {path}/{database name}.db; {path}/{database name}.log
		if name.is_empty() {
			fail!("Path of database is empty");
		}
		if !name.ends_with(".db") {
			fail!("Path of database should be suffixed with '.db'")
		}
		let database = OpenOptions::new().read(true).write(true).create(true).open(name).unwrap();
		let log = OpenOptions::new().read(true).append(true).create(true).open(
			format!("{}.log", name.strip_suffix(".db").unwrap())).unwrap();
		DiskManager {
			name: name.to_string(),
			database,
			log,
		}
	}

	pub fn write_page(&mut self, page_id: PageIdType, data: &[u8]) {}

	pub fn read_page(&self, page_id: PageIdType, data: &mut [u8]) {}

	pub fn write_log(&mut self, data: &[u8]) {}

	pub fn read_log(&self, offset: usize, data: &mut [u8]) {}
}
