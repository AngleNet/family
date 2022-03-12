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
		let num = self.len();
		assert!((num as usize) < MAX_RECORDS_PER_PAGE);
		if self.find_record(name).is_some() {
			fail!("record '{}' has already existed in header page", name);
		}
		let offset = num * RECORD_LEN + RECORD_NUMS_LEN;
		self.page.data[..].copy_from_slice(name.as_bytes())
	}

	pub fn fetch_record(&self, name: &str) -> Option<PageIdType> {
		todo!()
	}

	#[inline]
	pub fn len(&self) -> usize {
		u32::from_be_bytes(self.page.data[..4].try_into().unwrap()) as usize
	}

	pub fn write_record_count(&mut self, count: u32) {}

	fn find_record(&mut self, name: &str) -> Option<u32> {
		todo!()
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn test() {
		let mut x = [0_u8; 10];
		let y = "x".as_bytes();
	}
}
