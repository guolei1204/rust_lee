use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn preoder_tranversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if root.is_none {
            return result;
        }

        preorder_recursive(root, &mut result);
        result
    }

    pub fn preoder_tranversal_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none {
            return result;
        }

        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut r = root.clone();

        while r.is_some || !stack.is_empty() {
            while let Some(node) = r {
                result.push(node.borrow().val);
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            r = stack.pop();

            if let Some(node) = r {
                r = node.borrow().right.clone();
            }
        }
        result
    }
}
fn preorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            result.push(node.borrow.val);
            preorder_recursive(node.borrow().left.clone(), result);
            preorder_recursive(node.borrow().right.clone(), result);
        }
        None => {
            return;
        }
    }
}
