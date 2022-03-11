use crate::store::page::header::HeaderPage;
use crate::store::page::table::TablePage;

mod header;
mod table;

pub enum Page {
    Header(HeaderPage),
    Table(TablePage),
}
