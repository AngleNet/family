pub mod replacer;

use std::borrow::Borrow;
use std::collections::{HashMap, LinkedList};
use std::intrinsics::truncf32;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};
use crate::buffer::replacer::{LRUReplacer, Replacer};
use crate::store::page::{Page, PageRef};
use crate::config::{FrameIdType, PageIdType};
use crate::store::disk::{DiskManager, DiskManagerRef};

pub struct BufferPoolManager<R: Replacer> {
	disk_manager: DiskManagerRef,
	replacer: R,
	frames: Vec<PageRef>,
	page_table: HashMap<PageIdType, FrameIdType>,
	free_frames: LinkedList<FrameIdType>,
	next_page_id: AtomicU32,
}

pub type BufferPoolManagerRef<R> = Arc<RwLock<BufferPoolManager<R>>>;

impl<R: Replacer> BufferPoolManager<R> {
	pub fn new(size: usize, disk_manager: DiskManager, replacer: R) -> BufferPoolManager<R> {
		BufferPoolManager {
			disk_manager: disk_manager.into(),
			replacer,
			frames: Vec::with_capacity(size),
			page_table: HashMap::with_capacity(size),
			free_frames: LinkedList::new(),
			next_page_id: AtomicU32::new(0),
		}
	}

	pub fn new_page(&mut self) -> Option<PageRef> {
		let mut free = self.free_frames.pop_front();
		let mut new = false;
		if free.is_none() {
			// fixme: If we block here with lock held, any thread want to unpin a page will have to
			// wait for this which leads to deadlock
			free = self.replacer.victim();
			new = true;
		}
		if free.is_none() {
			return None;
		}
		let frame = Arc::clone(&self.frames[free.unwrap() as usize]);
		let mut page = frame.write().unwrap();
		if page.is_dirty() {
			let mut disk = self.disk_manager.write().unwrap();
			disk.write_page(page.id(), page.data());
		}
		page.reset(self.next_page_id());
		if new {
			self.page_table.insert(page.id(), free.unwrap());
		}
		return Some(frame);
	}

	pub fn fetch_page(&mut self, page_id: PageIdType) -> Option<PageRef> {
		if let Some(found) = self.page_table.get(&page_id) {
			let frame = Arc::clone(&self.frames[*found as usize]);
			let mut page = frame.write().unwrap();
			page.pin();
			return Some(frame);
		}
		let free = self.fetch_free_frame();
		if free.is_none() {
			return None;
		}
		let frame = Arc::clone(&self.frames[free.unwrap() as usize]);
		let mut page = frame.write().unwrap();
		todo!()
	}

	pub fn unpin_page(&mut self, page_id: PageIdType) {}

	pub fn flush_page(&mut self, page_id: PageIdType) {}

	#[inline]
	fn next_page_id(&mut self) -> PageIdType {
		self.next_page_id.fetch_add(1, Ordering::Relaxed) as PageIdType
	}

	#[inline]
	fn fetch_free_frame(&mut self) -> Option<FrameIdType> {
		let mut free = self.free_frames.pop_front();
		if free.is_none() {
			free = self.replacer.victim();
		}
		return free;
	}
}

#[cfg(test)]
mod test {
	use std::fs;
	use crate::buffer::BufferPoolManager;
	use crate::buffer::replacer::LRUReplacer;
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
		let mut buf_manager = BufferPoolManager::new(size, disk_manager, LRUReplacer {});
		let page = buf_manager.new_page();
	}

	#[derive(Debug)]
	struct A {
		a: u32,
	}

	impl Drop for A {
		fn drop(&mut self) {
			println!("Dropping {}", self.a);
		}
	}

	#[test]
	fn test_scope() {
		let mut a = None;
		{
			a = Some(A { a: 3 });
		}
		println!("{:?}", a)
	}
}
