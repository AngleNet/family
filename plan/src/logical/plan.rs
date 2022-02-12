use super::*;
use crate::expression::{AggFuncDesc, Column, Expression, FieldName, Schema};
use crate::AccessPath;
use catalog::TableRef;
use parser::ast::Expr;

/// Every logical plan should have a schema which indicates how its output looks like.
pub enum LogicalPlan {
    DataSource(DataSource),
    TableDual {
        schema: Schema,
        row_count: i32,
    },
    Projection {
        schema: Schema,
        /// What happens when we have both aggregations and simple projections in the same query?
        /// Maybe this is only the simple projection.
        exprs: Vec<Expression>,
    },
    Aggregation {
        schema: Schema,
        funcs: Vec<AggFuncDesc>,
        group_by_items: Vec<Expression>,
        group_by_columns: Vec<Column>,
    },
    Sort {
        schema: Schema,
        by_items: Expression,
        desc: bool,
    },
    Limit {
        schema: Schema,
        offset: u64,
        count: u64,
    },
    Selection {
        schema: Schema,
        conditions: Vec<Expression>,
    },
    TableScan {
        schema: Schema,
        source: DataSource,
    },
    IndexScan {
        schema: Schema,
        source: DataSource,
    },
}

pub struct DataSource {
    schema: Schema,
    database: String,
    table: String,
    instance: TableRef,
    push_down_expr: Vec<Expression>,
    filter_expr: Vec<Expression>,
    possible_access_paths: Vec<AccessPath>,
    fields: Vec<FieldName>,
}
