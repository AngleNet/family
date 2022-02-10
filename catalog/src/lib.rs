mod database;
mod table;

pub use database::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
pub use table::*;

/// Catalog is used to retrieve the schema information.
pub struct Catalog {
    database: HashMap<String, DatabaseRef>,
}

pub type CatalogRef = Arc<Catalog>;

impl Catalog {
    pub fn schema_by_name(&self, schema: &str) -> Option<DatabaseRef> {
        self.database.get(schema).cloned()
    }

    pub fn schema_exists(&self, schema: &str) -> bool {
        self.database.contains_key(schema)
    }

    pub fn table_by_name(&self, schema: &str, table: &str) -> Option<TableRef> {
        if let Some(db) = self.database.get(schema) {
            let db = db.read().unwrap();
            return db.tables.get(table).cloned();
        }
        None
    }

    pub fn table_exists(&self, schema: &str, table: &str) -> bool {
        if let Some(db) = self.database.get(schema) {
            let db = db.read().unwrap();
            return db.tables.contains_key(table);
        }
        false
    }
}
