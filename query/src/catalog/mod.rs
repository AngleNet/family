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
use crate::buffer::replacer::Replacer;

pub type TableOidType = i64;

pub type IndexOidType = i64;

pub type ColumnOidType = i64;

pub struct Catalog<R: Replacer> {
	buffer_pool_manager: BufferPoolManagerRef<R>,
	tables: HashMap<TableOidType, TableRef>,
	table_names: HashMap<String, TableOidType>,
}

impl<R: Replacer> Catalog<R> {
	pub fn new(bfm: BufferPoolManager<R>) -> Catalog<R> {
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
