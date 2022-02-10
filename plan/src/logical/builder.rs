use super::*;
use catalog::{CatalogRef, TableInfo};
use parser::ast::{Statement, TableFactor};

pub struct PlanBuilder {
    catalog: CatalogRef,
}

impl PlanBuilder {
    pub fn build_data_source(&mut self, table: &TableFactor) -> Option<LogicalPlan> {
        /// 1. find table meta in catalog
        /// 2. check table types. There are two types of table: NormalTable and VirtualTable. NormalTable
        /// stores data in tikv while VirtualTable stores data in memory structs.
        /// 3. get all of possible access paths
        /// 4. fetch the table statistics
        /// 5. convert the table column to expression column
        None
    }

    pub fn build_mem_table(&mut self, database: &str, table: &TableInfo) -> Option<LogicalPlan> {
        None
    }
}
