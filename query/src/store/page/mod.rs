use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::str::from_utf8;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::config::PAGE_SIZE;
use crate::store::disk::PageIdType;
use crate::store::page::header::HeaderPage;
use crate::store::page::table::TablePage;

mod header;
mod table;

pub struct Page {
	id: PageIdType,
	data: [u8; PAGE_SIZE],
}

pub type PageRef = Arc<RwLock<Page>>;

impl Page {
	pub fn new(id: PageIdType) -> Page {
		Page {
			id,
			data: [0; PAGE_SIZE],
		}
	}

	pub fn id(&self) -> PageIdType {
		self.id
	}

	pub fn data(&self) -> &[u8] {
		&self.data
	}

	pub fn data_mut(&mut self) -> &mut [u8] {
		&mut self.data
	}

	#[inline]
	pub fn write(&mut self, offset: usize, buf: &[u8]) {
		assert!(self.data.len() - offset >= buf.len());
		self.data[offset..(offset + buf.len())].copy_from_slice(buf);
	}

	#[inline]
	pub fn write_str(&mut self, offset: usize, s: &str) {
		assert!(s.len() < self.data.len() - offset);
		self.write(offset, s.as_bytes());
		self.data[offset + s.len()] = b'\0';
	}

	pub fn read_str(&self, offset: usize) -> &str {
		let mut end = offset;
		for i in offset..self.data.len() {
			end = i;
			if self.data[i] == b'\0' {
				break;
			}
		}
		from_utf8(&self.data[offset..end]).unwrap()
	}

	#[inline]
	pub fn read_u32(&self, offset: usize) -> u32 {
		assert!(self.data.len() - offset >= 4);
		u32::from_be_bytes(self.data[offset..(offset + 4)].try_into().unwrap())
	}

	#[inline]
	pub fn write_u32(&mut self, offset: usize, n: u32) {
		self.write(offset, n.to_be_bytes().as_slice());
	}
}

impl Into<PageRef> for Page {
	#[inline]
	fn into(self) -> PageRef {
		Arc::new(RwLock::new(self))
	}
}

impl<'a> AsRef<HeaderPage> for RwLockReadGuard<'a, Page> {
	#[inline]
	fn as_ref(&self) -> &HeaderPage {
		HeaderPage::new(self.deref())
	}
}

impl<'a> AsMut<HeaderPage> for RwLockWriteGuard<'a, Page> {
	#[inline]
	fn as_mut(&mut self) -> &mut HeaderPage {
		HeaderPage::new_mut(self.deref_mut())
	}
}


#[cfg(test)]
mod test {
	use std::sync::{Arc, RwLock};
	use crate::store::page::{Page, PageRef};
	use std::thread;
	use std::time::Duration;
	use crate::store::page::header::HeaderPage;

	#[test]
	fn test_concurrent_access_page_ref() {
		let mut p1: PageRef = Page::new(0).into();
		let mut write_p1 = Arc::clone(&p1);
		let write = thread::spawn(move || {
			let mut lock = write_p1.write().unwrap();
			let h = lock.as_mut();
			thread::sleep(Duration::from_secs(8));
		});
		let read_p1 = Arc::clone(&p1);
		let read = thread::spawn(move || {
			thread::sleep(Duration::from_secs(2));
			let lock = read_p1.read().unwrap();
			let h = lock.as_ref();
		});
		thread::sleep(Duration::from_secs(3));
		assert!(read.is_running());
		write.join().unwrap();
		thread::sleep(Duration::from_secs(1));
		assert!(!read.is_running());
		read.join().unwrap();
	}

	#[test]
	fn test_read_str() {
		let mut page = Page::new(10);
		assert_eq!("", page.read_str(0));
		page.write_str(1, "hello");
		assert_eq!("hello", page.read_str(1));
		assert_eq!("ello", page.read_str(2));
	}
}
