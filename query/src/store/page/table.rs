use crate::store::table::Tuple;


/// Slotted page format:
/// ---------------------------------------------------------
/// | HEADER | ... FREE SPACE ... | ... INSERTED TUPLES ... |
/// ---------------------------------------------------------
///
/// Header format (size in bytes):
/// ----------------------------------------------------------------------------
/// | PageId (4)| LSN (4)| PrevPageId (4)| NextPageId (4)| FreeSpacePointer(4) |
/// ----------------------------------------------------------------------------
/// ----------------------------------------------------------------
/// | TupleCount (4) | Tuple_1 offset (4) | Tuple_1 size (4) | ... |
/// ----------------------------------------------------------------
///
/// free space pointer points to where the last inserted tuple ends
pub struct TablePage {}

impl TablePage {
    pub fn insert_tuple(&mut self, tuple: &Tuple) {}
}
