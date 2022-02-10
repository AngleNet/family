use super::*;
use crate::expression::{Expression, FieldName, Schema};
use crate::AccessPath;
use catalog::TableRef;
use parser::ast::Expr;

pub enum LogicalPlan {
    DataSource {
        schema: Schema,
        database: String,
        table: String,
        instance: TableRef,
        push_down_expr: Vec<Expression>,
        filter_expr: Vec<Expression>,
        possible_access_paths: Vec<AccessPath>,
        fields: Vec<FieldName>,
    },
}
