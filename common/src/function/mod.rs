use crate::catalog::TableCatalogEntryRef;

pub mod table;

pub enum FunctionData {
    TableScan()
}

pub struct TableScanBindData {
    pub table: TableCatalogEntryRef,
}
