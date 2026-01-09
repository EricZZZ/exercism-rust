fn main() {}

// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(curr) => {
                let left = dfs(curr.borrow().left.clone());
                let right = dfs(curr.borrow().right.clone());

                let d1 = left.0;
                let d2 = right.0;

                match d1.cmp(&d2) {
                    Ordering::Greater => (d1 + 1, left.1),

                    Ordering::Less => (d2 + 1, right.1),

                    Ordering::Equal => (d1 + 1, Some(curr)),
                }
            }
        }
    }

    dfs(root).1
}
