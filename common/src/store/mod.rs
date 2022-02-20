use std::cell::RefCell;
use std::rc::Rc;

pub struct TableHandle{}

pub type TableHandleRef = Rc<RefCell<TableHandle>>;
