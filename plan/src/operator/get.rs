use common::catalog::{ColumnIdxType, IdxType};
use common::function::{FunctionData, TableScanBindData};
use common::function::table::TableFunction;
use common::types::LogicalType;

pub struct LogicalGet {
    // the table index in the current bind context
    pub table_index: IdxType,
    pub function: TableFunction,
    pub bind_data: TableScanBindData,
    pub returned_types: Vec<LogicalType>,
    pub names: Vec<String>,
    pub column_ids: Vec<ColumnIdxType>,
}

impl LogicalGet {
    pub fn new(table_index: IdxType, function: TableFunction, bind_data: TableScanBindData,
               types: Vec<LogicalType>, names: Vec<String>) -> Self {
        LogicalGet {
            table_index,
            function,
            bind_data,
            returned_types: types,
            names,
            column_ids: vec![],
        }
    }
}
