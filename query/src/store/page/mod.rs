use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
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
}
