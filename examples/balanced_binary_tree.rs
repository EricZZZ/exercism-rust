use std::{cell::RefCell, cmp, rc::Rc};

fn main() {}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_height = max_depth(&node.borrow().left);
                if left_height == -1 {
                    return -1;
                }
                let right_height = max_depth(&node.borrow().right);
                if right_height == -1 {
                    return -1;
                }

                if (left_height - right_height).abs() > 1 {
                    return -1;
                }

                cmp::max(left_height, right_height) + 1
            }
        }
    }

    max_depth(&root) == -1
}

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
