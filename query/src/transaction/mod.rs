use std::sync::{Arc, RwLock};

mod aries;
mod mvcc;

pub struct Transaction {}

pub type TransactionRef = Arc<RwLock<Transaction>>;
