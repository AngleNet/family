use std::collections::HashMap;

pub enum QueryNode {
    Select(SelectNode)
}

pub struct SelectNode {}

/// Represents a generic expression that returns a table
pub enum TableRef {
    BaseTable()
}

pub struct BaseTableRef {
    pub schema: String,
    pub table: String,
    pub column_name_alias: Vec<String>,
    pub alias: String,
}
