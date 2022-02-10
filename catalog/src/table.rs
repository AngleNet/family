use std::sync::{Arc, RwLock};

pub struct Table {
    meta: TableInfo,
}

pub struct TableInfo {}

impl Clone for TableInfo {
    fn clone(&self) -> Self {
        todo!()
    }
}

pub type TableRef = Arc<RwLock<Table>>;

impl Table {
    pub fn meta(&self) -> TableInfo {
        self.meta.clone()
    }
}
