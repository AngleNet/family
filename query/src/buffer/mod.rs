use std::sync::{Arc, RwLock};
use crate::store::page::Page;
use crate::config::PageIdType;
use crate::store::disk::{DiskManager, DiskManagerRef};

pub struct BufferPoolManager {
	size: usize,
	disk_manager: DiskManagerRef,
}

pub type BufferPoolManagerRef = Arc<RwLock<BufferPoolManager>>;

impl BufferPoolManager {
	pub fn new(size: usize, disk_manager: DiskManager) -> BufferPoolManager {
		BufferPoolManager {
			size,
			disk_manager: Arc::new(RwLock::new(disk_manager)),
		}
	}

	pub fn new_page(&mut self) -> Option<Page> {
		todo!()
	}

	pub fn fetch_page(&mut self, page_id: PageIdType) -> Page {
		todo!()
	}

	pub fn unpin_page(&mut self, page_id: PageIdType) {}

	pub fn flush_page(&mut self, page_id: PageIdType) {}
}

#[cfg(test)]
mod test {
	use std::fs;
	use crate::buffer::BufferPoolManager;
	use crate::store::disk::DiskManager;

	pub struct Holder {
		pub db: String,
		pub log: String,
	}

	impl Holder {
		pub fn new() -> Holder {
			Holder {
				db: "/tmp/test.db".to_string(),
				log: "/tmp/test.log".to_string(),
			}
		}

		fn remove_file(path: &str) {
			match fs::metadata(path) {
				Ok(meta) => {
					if !meta.is_file() {
						return;
					}
					fs::remove_file(path);
				}
				Err(_) => {}
			}
		}
	}

	impl Drop for Holder {
		fn drop(&mut self) {
			Holder::remove_file(&self.db);
			Holder::remove_file(&self.log);
		}
	}

	#[test]
	fn binary_data_test() {
		let holder = Holder::new();
		let disk_manager = DiskManager::new(&holder.db);
		let size = 10_usize;
		let mut buf_manager = BufferPoolManager::new(size, disk_manager);
		let page = buf_manager.new_page();
	}
}
