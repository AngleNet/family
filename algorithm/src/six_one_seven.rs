use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution {}

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root1 {
            Some(ref a) => match root2 {
                Some(ref b) => {
                    let mut a = a.borrow_mut();
                    let b = b.borrow_mut();
                    a.val = a.val + b.val;
                    a.left =
                        Solution::merge_trees(a.left.as_ref().cloned(), b.left.as_ref().cloned());
                    a.right =
                        Solution::merge_trees(a.right.as_ref().cloned(), b.right.as_ref().cloned());
                    return root1.clone();
                }
                None => root1,
            },
            None => root2,
        }
    }
}
