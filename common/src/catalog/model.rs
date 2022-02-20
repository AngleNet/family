use crate::catalog::IdxType;
use crate::types::LogicalType;

pub struct ColumnDefinition {
    pub name: String,
    pub oid: IdxType,
    pub column_type: LogicalType,
}
