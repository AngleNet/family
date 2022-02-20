use std::cell::RefCell;
use std::rc::Rc;
use crate::catalog::Catalog;

pub struct ClientContext {
    pub catalog: Rc<RefCell<Catalog>>,
}
