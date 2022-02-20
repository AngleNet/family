mod model;


use std::cell::RefCell;
use std::rc::Rc;
use crate::catalog::model::ColumnDefinition;
use crate::error::Result;
use crate::store::{TableHandle, TableHandleRef};

pub type IdxType = u64;
pub type ColumnIdxType = IdxType;

pub struct Catalog {}

impl Catalog {
    pub fn entry(&self, catalog_type: CatalogType, schema_name: &str,
                 name: &str) -> Result<CatalogEntry> {
        todo!()
    }

    pub fn lookup_entry(&self, catalog_type: CatalogType, schema_name: &str,
                        name: &str) -> Result<CatalogEntry> {
        todo!()
    }
}

pub enum CatalogEntry {
    Schema(SchemaCatalogEntry),
    TableEntry(TableCatalogEntry),
    ViewEntry,
}

pub enum CatalogType {
    TableEntry,
    ViewEntry,
}

pub struct CatalogEntryLookup {
    pub schema: SchemaCatalogEntryRef,
    pub entry: CatalogEntry,
}

pub struct BaseCatalogEntry {
    pub oid: IdxType,
}

pub struct SchemaCatalogEntry {}

pub type SchemaCatalogEntryRef = Rc<RefCell<SchemaCatalogEntry>>;

pub struct TableCatalogEntry {
    pub base: BaseCatalogEntry,
    pub schema: SchemaCatalogEntryRef,
    pub table: TableHandleRef,
    pub columns: Vec<ColumnDefinition>,
}

pub type TableCatalogEntryRef  = Rc<RefCell<TableCatalogEntry>>;

pub struct ViewCatalogEntry {}
