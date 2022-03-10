mod table;
mod column;
mod index;
mod scheme;

pub use table::*;
pub use column::*;
pub use index::*;
pub use scheme::*;

pub type TableOidType = i64;

pub type IndexOidType = i64;

pub type ColumnOidType = i64;

pub struct Catalog {
}

impl Catalog {

    pub fn resolve_table(oid: TableOidType) -> Option<Table> {
        todo!()
    }

    pub fn resolve_index(oid: IndexOidType) -> Option<Index> {
        todo!()
    }

}
