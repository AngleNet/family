mod expression;
mod logical;
mod physical;
mod stats;
mod util;

pub use util::*;

//The SQL string is converted to AST by the parser, every node in the AST is specific to sql parser.
//The AST will be converted to statement composed of a bunch of plan components. There are two types
//of a plan component: logical or physical. A physical plan component could an implementation of a
//logical plan component or could be an add-on of a query plan.
