use parser::ast::DataType;

/// ScalarFunc is the function that returns a value
pub struct ScalarFunc {
    pub return_type: DataType,
    pub func: BuiltinFunc,
}

pub enum BuiltinFunc {
    IsNull(IsNullFunc),
}

impl ScalarFunc {
    pub fn new(return_type: DataType, func: BuiltinFunc) -> Self {
        ScalarFunc { return_type, func }
    }
}

pub struct IsNullFunc {}
