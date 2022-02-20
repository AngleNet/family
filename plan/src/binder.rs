use std::collections::HashSet;
use std::fmt::format;
use common::catalog::{CatalogEntry, CatalogType, IdxType, TableCatalogEntry, TableCatalogEntryRef};
use common::context::ClientContext;
use common::function::table::TableFunction;
use crate::ast::{BaseTableRef, QueryNode, SelectNode};
use common::prelude::*;
use crate::operator::get::LogicalGet;

pub struct Binder {
    ctx: ClientContext,
    idx: IdxType,
}

impl Binder {
    pub fn bind_query(&mut self, node: &QueryNode) {
        todo!()
    }

    pub fn bind_select(&mut self, node: &SelectNode) {}

    pub fn bind_base_table(&mut self, node: &BaseTableRef) -> Result<BoundTableRef> {
        let table_or_view = self.ctx.catalog.borrow().entry(
            CatalogType::TableEntry, node.schema.as_str(),
            node.table.as_str())?;
        let bound = match table_or_view {
            CatalogEntry::TableEntry(e) => {
                let scan = TableFunction {};
                let idx = self.generate_table_index();
                let mut col_types = vec![];
                let mut col_names = vec![];
                for col in &e.columns {
                    col_types.push(col.column_type);
                    col_names.push(col.name.clone());
                }
            }
            CatalogEntry::ViewEntry => {}
            _ => {}
        };
        todo!()
    }

    pub fn generate_table_index(&mut self) -> IdxType {
        self.idx += 1;
        self.idx
    }

    pub fn alias_column_names(table: &str, names: &Vec<String>, column_alias: &Vec<String>) -> Result<()> {
        if column_alias.len() > names.len() {
            return Err(FamilyError::with(format!("table {} has {} columns available but {} columns specified",
                                                 table, names.len(), column_alias.len())));
        }
        todo!()
    }

    // base -> base:1 -> base:2 -> base:3
    pub fn add_column_name_to_binding(base: &str, current_names: &mut HashSet<String>) -> String {
        let mut idx = 1;
        let mut name = base.to_string();
        while current_names.contains(&name) {
            name = format!("{}:{}", base, idx);
        }
        current_names.insert(name.clone());
        return name;
    }
}

pub enum BoundTableRef {
    BaseTable(BoundBaseTableRef)
}

pub struct BoundBaseTableRef {
    pub table: TableCatalogEntryRef,
    pub get: LogicalGet,
}

#[cfg(test)]
pub mod test {

    #[test]
    pub fn test() {
    }
}
