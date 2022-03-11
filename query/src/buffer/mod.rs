use crate::store::page::Page;
use crate::config::PageIdType;

pub struct BufferPoolManager {}

impl BufferPoolManager {
    pub fn fetch_page(&mut self, page_id: PageIdType) -> Page {
        todo!()
    }

    pub fn unpin_page(&mut self, page_id: PageIdType) {}
}
