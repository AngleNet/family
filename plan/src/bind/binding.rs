/// Represents a binding to a table, table-producing function or subquery with a specified table index
pub enum Binding {
    Normal,
    Table,
    Macro,
}

pub struct TableBinding {}

pub struct NormalBinging {}

pub struct MacroBinding {}
