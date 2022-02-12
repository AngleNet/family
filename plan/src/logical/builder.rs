use super::*;
use catalog::{CatalogRef, TableInfo};
use parser::ast::{ObjectName, Statement, TableAlias, TableFactor};

pub struct PlanBuilder {
    catalog: CatalogRef,
}

impl PlanBuilder {
    pub fn build_data_source(
        &mut self,
        table: &ObjectName,
        alias: &Option<TableAlias>,
    ) -> Option<LogicalPlan> {
        /// 1. find table meta in catalog
        /// 2. check table types. There are two types of table: NormalTable and VirtualTable. NormalTable
        /// stores data in tikv while VirtualTable stores data in memory structs.
        /// 3. get all of possible access paths
        /// 4. fetch the table statistics
        /// 5. convert the table column to expression column
        if table.0.is_empty() {
            return None;
        }
        let db_name = &table.0[0].value;
        let mut tb_name = &table.0[1].value;
        let table = self
            .catalog
            .table_by_name(db_name.as_str(), tb_name.as_str())?;
        let table_info = table.read().ok()?.meta();
        if table_info.is_virtual() {
            return self.build_mem_table(db_name.as_str(), table_info);
        }
        let x = table_info;
        if let Some(t) = alias {
            // override the table name with alias
            tb_name = &t.name.value;
        }

        None
    }

    pub fn build_mem_table(&mut self, database: &str, table: TableInfo) -> Option<LogicalPlan> {
        None
    }
}
