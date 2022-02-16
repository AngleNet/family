use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use crate::Symbol;
use super::*;

pub type PlanNodeRef = Rc<RefCell<PlanNode>>;

pub enum PlanNode {
    Limit(LimitNode)
}

pub trait LogicalPlan {
    fn source(&self) -> LinkedList<Symbol>;

    fn outputSymbols(&self) -> LinkedList<Symbol>;

    fn replaceChildren(&mut self, children: LinkedList<PlanNodeRef>);
}

impl LogicalPlan for PlanNodeRef {
    fn source(&self) -> LinkedList<PlanNodeRef> {
        todo!()
    }

    fn outputSymbols(&self) -> LinkedList<Symbol> {
        todo!()
    }

    fn replaceChildren(&mut self, children: LinkedList<PlanNodeRef>) {
        todo!()
    }
}

pub struct LimitNode {
    pub source: PlanNodeRef,
    pub count: usize,
}

impl LogicalPlan for LimitNode {
    fn source(&self) -> LinkedList<PlanNodeRef> {
        let mut s = LinkedList::new();
        s.push_back(self.source.clone());
        return s;
    }

    fn outputSymbols(&self) -> LinkedList<Symbol> {
        self.source.outputSymbols()
    }

    fn replaceChildren(&mut self, mut children: LinkedList<PlanNodeRef>) {
        self.source = children.pop_back().unwrap();
    }
}
