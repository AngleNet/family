use std::sync::{Arc, RwLock};
use crate::catalog::{Scheme, TableOidType};

pub struct Table {
    oid: TableOidType,
    scheme: Scheme,
}

pub type TableRef = Arc<RwLock<Table>>;

impl Table {
    pub fn scheme_mut(&mut self) -> &mut Scheme {
        &mut self.scheme
    }

    pub fn scheme(&self) -> &Scheme {
        &self.scheme
    }
}

impl Clone for Table {
    fn clone(&self) -> Self {
        todo!()
    }
}
