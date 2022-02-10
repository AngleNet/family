use common::types::DataBox;
use parser::ast::DataType;

/// Constant stands for a constant value.
pub struct Constant {
    pub return_type: DataType,
    pub value: DataBox,
}
