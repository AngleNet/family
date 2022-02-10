use crate::TableRef;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Database {
    pub tables: HashMap<String, TableRef>,
}

pub type DatabaseRef = Arc<RwLock<Database>>;
