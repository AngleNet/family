use crate::config::PageIdType;

pub struct RID {
    page: PageIdType,
    slot: u32,
}

impl RID {
    pub fn new(page: PageIdType, slot: u32) -> RID {
        RID {
            page,
            slot,
        }
    }

    pub fn fetch(&self) -> i64 {
        // high 32 bits are page number, low 32 bits are slot number
        todo!()
    }
}
