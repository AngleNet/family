use common::catalog::{ColumnIdxType, IdxType};
use common::function::FunctionData;
use common::function::table::TableFunction;
use common::types::LogicalType;

pub struct LogicalGet {
    // the table index in the current bind context
    pub table_index: IdxType,
    pub function: TableFunction,
    pub bind_data: FunctionData,
    pub returned_types: Vec<LogicalType>,
    pub names: Vec<String>,
    pub column_ids: Vec<ColumnIdxType>
}
