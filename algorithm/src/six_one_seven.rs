use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root1 {
            Some(ref a) => {
                match root2 {
                    Some(ref b) => {
                        let a = a.borrow_mut();
                        let b = b.borrow_mut();
                        a.val = a.val + b.val;
                        a.left = Solution::merge_trees(a.left.as_ref().cloned(), b.left.as_ref().cloned());
                        a.right = Solution::merge_trees(a.right.as_ref().cloned(), b.right.as_ref().cloned());
                        return root1;
                    }
                    None => { root1 }
                }
            }
            None => root2
        }
    }
}
