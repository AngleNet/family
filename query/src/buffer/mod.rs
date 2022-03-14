pub mod replacer;

use std::borrow::Borrow;
use std::collections::{HashMap, LinkedList};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};
use crate::buffer::replacer::{LRUReplacer, Replacer};
use crate::store::page::{Page, PageRef};
use crate::config::{FrameIdType, PageIdType};
use crate::store::disk::{DiskManager};

pub struct BufferPoolManager<R: Replacer> {
	disk_manager: DiskManager,
	replacer: R,
	frames: Vec<PageRef>,
	page_table: HashMap<PageIdType, FrameIdType>,
	free_frames: LinkedList<FrameIdType>,
	next_page_id: u32,
}

pub type BufferPoolManagerRef<R> = Arc<RwLock<BufferPoolManager<R>>>;

impl<R: Replacer> BufferPoolManager<R> {
	pub fn new(size: usize, disk_manager: DiskManager, replacer: R) -> BufferPoolManager<R> {
		let mut frees = LinkedList::new();
		for i in 0..size {
			frees.push_back(i as FrameIdType);
		}
		BufferPoolManager {
			disk_manager,
			replacer,
			frames: Vec::with_capacity(size),
			page_table: HashMap::with_capacity(size),
			free_frames: frees,
			next_page_id: 0,
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
		{
			let mut page = frame.write().unwrap();
			if page.dirty() {
				self.disk_manager.write_page(page.id(), page.data());
			}
			page.reset(self.next_page_id());
			if new {
				self.page_table.insert(page.id(), free.unwrap());
			}
		}
		return Some(frame);
	}

	pub fn fetch_page(&mut self, page_id: PageIdType) -> Option<PageRef> {
		if let Some(found) = self.page_table.get(&page_id) {
			// the page is already in mem, pin it again
			let frame = Arc::clone(&self.frames[*found as usize]);
			self.replacer.pin(*found);
			return Some(frame);
		}
		let free = self.fetch_free_frame();
		if free.is_none() {
			return None;
		}
		// page is not in memory, flush the evicted frame and load new page from disk and pin it
		let frame = Arc::clone(&self.frames[free.unwrap() as usize]);
		{
			let mut page = frame.write().unwrap();
			if page.dirty() {
				self.disk_manager.write_page(page.id(), page.data());
			}
			self.page_table.remove(&page.id());
			self.page_table.insert(page_id, free.unwrap());
			page.reset(page_id);
			self.replacer.pin(free.unwrap());
			self.disk_manager.read_page(page_id, page.data_mut());
		}
		return Some(frame);
	}

	pub fn unpin_page(&mut self, page_id: PageIdType) {
		let found = self.page_table.get(&page_id).unwrap();
		self.replacer.unpin(*found);
	}

	#[inline]
	fn next_page_id(&mut self) -> PageIdType {
		self.next_page_id += 1;
		return self.next_page_id as PageIdType;
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
	use rand::Fill;
	use rand::prelude::*;
	use crate::buffer::BufferPoolManager;
	use crate::buffer::replacer::{LRUReplacer, Replacer};
	use crate::config::{FrameIdType, PAGE_SIZE, PageIdType};
	use crate::store::disk::DiskManager;

	struct Holder {
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

	struct DummyReplacer;

	impl Replacer for DummyReplacer {
		fn victim(&self) -> Option<FrameIdType> {
			return None;
		}
		fn pin(&mut self, frame_id: FrameIdType) {}

		fn unpin(&mut self, frame_id: FrameIdType) {}
	}

	#[test]
	fn binary_data_test() {
		let holder = Holder::new();
		let size = 10_usize;
		let mut bm = BufferPoolManager::new(size, DiskManager::new(&holder.db), DummyReplacer {});
		let p1 = bm.new_page();
		assert!(p1.is_some());
		let mut bytes = [0_u8; PAGE_SIZE];
		let mut rng = rand::thread_rng();
		bytes.try_fill(&mut rng).unwrap();
		// insert terminal chars both in the middle and at end
		bytes[PAGE_SIZE / 2] = b'\0';
		bytes[PAGE_SIZE - 1] = b'\0';
		let p1 = p1.unwrap();
		let mut lock = p1.write().unwrap();
		lock.data_mut().copy_from_slice(&bytes);
		lock.mark_dirty();
		for i in 1..size {
			assert!(bm.new_page().is_some());
		}
		assert!(bm.new_page().is_none());
		// after we unpin [0, 1, 2, 3, 4], we are able to create another 5 pages
		for i in 0..5 {
			bm.unpin_page(i as PageIdType);
		}
		for i in 0..5 {
			let page = bm.new_page();
			assert!(page.is_some());
			let page = page.unwrap();
			let lock = page.read().unwrap();
			bm.unpin_page(lock.id());
		}
		let page = bm.fetch_page(0 as PageIdType);
		assert!(page.is_some());
		let page = page.unwrap();
		let lock = page.read().unwrap();
		assert_eq!(bytes, lock.data());
	}
}
