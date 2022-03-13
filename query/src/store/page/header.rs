use crate::config::{INVALID_PAGE_ID, PAGE_SIZE, PageIdType};
use crate::fail;
use crate::store::page::Page;

const RECORD_NUMS_LEN: usize = 4;
const RECORD_NAME_LEN: usize = 32;
const ROOT_ID_LEN: usize = 4;
const RECORD_LEN: usize = RECORD_NAME_LEN + 4;
const MAX_RECORDS_PER_PAGE: usize = PAGE_SIZE - 4 / RECORD_LEN;

/**
 * Database use the first page (page_id = 0) as header page to store metadata, in
 * our case, we will contain information about table/index name (length less than
 * 32 bytes) and their corresponding root_id
 *
 * Format (size in byte):
 *  -----------------------------------------------------------------
 * | RecordCount (4) | Entry_1 name (32) | Entry_1 root_id (4) | ... |
 *  -----------------------------------------------------------------
 */
pub struct HeaderPage {
	page: Page,
}

impl HeaderPage {
	pub fn new(page: &Page) -> &HeaderPage {
		unsafe {
			&*(page as *const Page as *const HeaderPage)
		}
	}

	pub fn new_mut(page: &mut Page) -> &mut HeaderPage {
		unsafe {
			&mut *(page as *mut Page as *mut HeaderPage)
		}
	}

	/// Inserts an header record into the page. If the record already exists, it will panics.
	pub fn insert_record(&mut self, name: &str, root_it: PageIdType) {
		assert!(name.len() < RECORD_NAME_LEN);
		assert!(root_it > INVALID_PAGE_ID);
		let num = self.num_records();
		assert!((num as usize) < MAX_RECORDS_PER_PAGE);
		if self.find_record(name).is_some() {
			fail!("record '{}' has already existed in header page", name);
		}
		let mut offset = num * RECORD_LEN + RECORD_NUMS_LEN;
		self.page.data[offset..(offset + name.len())].copy_from_slice(name.as_bytes());
		offset += RECORD_NAME_LEN;
		self.page.data[offset..(offset + ROOT_ID_LEN)].copy_from_slice(&u32::to_be_bytes(root_it as u32));
		self.write_record_count(num + 1);
	}

	pub fn update_record(&mut self, name: &str, root_id: PageIdType) {
		assert!(root_id > INVALID_PAGE_ID);
		if let Some(offset) = self.find_record(name) {
			self.page.write_u32(offset + RECORD_NAME_LEN, root_id as u32);
			return;
		}
		fail!("record named '{}' does not exist", name);
	}

	pub fn fetch_record(&self, name: &str) -> Option<PageIdType> {
		if let Some(offset) = self.find_record(name) {
			return Some(self.page.read_u32(offset + RECORD_NAME_LEN) as PageIdType);
		}
		return None;
	}

	#[inline]
	pub fn num_records(&self) -> usize {
		self.page.read_u32(0) as usize
	}

	pub fn write_record_count(&mut self, count: usize) {
		self.page.write_u32(0, count as u32);
	}

	fn find_record(&self, name: &str) -> Option<usize> {
		assert!(name.len() <= RECORD_NAME_LEN);
		let num = self.num_records();
		let mut offset = RECORD_NUMS_LEN;
		for i in 0..num {
			if name.eq(self.page.read_str(offset)) {
				return Some(offset);
			}
			offset += RECORD_LEN;
		}
		return None;
	}
}

#[cfg(test)]
mod test {
	use crate::store::page::{Page, PageRef};

	#[test]
	fn read_write_records() {
		let page: PageRef = Page::new(1).into();
		let mut lock = page.write().unwrap();
		let header = lock.as_mut();
		assert_eq!(0, header.num_records());
		header.insert_record("r1", 1);
		assert_eq!(1, header.num_records());
		header.insert_record("r2", 2);
		assert_eq!(2, header.num_records());
		assert_eq!(1, header.fetch_record("r1").unwrap());
		assert_eq!(2, header.fetch_record("r2").unwrap());
	}
}
