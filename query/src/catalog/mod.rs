mod table;
mod column;
mod index;
mod scheme;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
pub use table::*;
pub use column::*;
pub use index::*;
pub use scheme::*;
use crate::buffer::{BufferPoolManager, BufferPoolManagerRef};

pub type TableOidType = i64;

pub type IndexOidType = i64;

pub type ColumnOidType = i64;

pub struct Catalog {
    buffer_pool_manager: BufferPoolManagerRef,
    tables: HashMap<TableOidType, TableRef>,
    table_names: HashMap<String, TableOidType>,
}

impl Catalog {
    pub fn new(bfm: BufferPoolManager) -> Catalog {
        Catalog {
            buffer_pool_manager: Arc::new(RwLock::new(bfm)),
            tables: HashMap::new(),
            table_names: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        unimplemented!()
    }

    pub fn resolve_table(&self, oid: TableOidType) -> Option<TableRef> {
        self.tables.get(&oid).cloned()
    }
}
