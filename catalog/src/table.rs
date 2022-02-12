use parser::ast::Expr;
use std::sync::{Arc, RwLock};

pub struct Table {
    meta: TableInfo,
}

/// There are two types of columns: normal or generated column
pub enum Column {
    Normal,
    Generated(Expr),
}

pub enum TableInfo {
    Normal { columns: Vec<Column> },
    Virtual,
}

impl TableInfo {
    pub fn is_normal(&self) -> bool {
        match self {
            TableInfo::Normal { .. } => true,
            _ => false,
        }
    }

    pub fn is_virtual(&self) -> bool {
        match self {
            TableInfo::Virtual => true,
            _ => false,
        }
    }
}

impl Clone for TableInfo {
    fn clone(&self) -> Self {
        todo!()
    }
}

pub type TableRef = Arc<RwLock<Table>>;

impl Table {
    pub fn meta(&self) -> TableInfo {
        self.meta.clone()
    }
}
