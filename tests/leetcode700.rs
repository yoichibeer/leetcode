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

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // pub fn search_bst(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     val: i32,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     match root {
    //         None => None,
    //         Some(node) => {
    //             if node.borrow().val == val {
    //                 Some(node)
    //             } else {
    //                 match Self::search_bst(node.borrow().left.clone(), val) {
    //                     Some(left) => Some(left),
    //                     None => Self::search_bst(node.borrow().right.clone(), val),
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub fn search_bst(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     val: i32,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     let mut stack = vec![root];
    //     while let Some(node) = stack.pop() {
    //         if let Some(node) = node {
    //             if node.borrow().val == val {
    //                 return Some(node);
    //             }
    //             stack.push(node.borrow().left.clone());
    //             stack.push(node.borrow().right.clone());
    //         }
    //     }
    //     None
    // }

    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                if node.borrow().val == val {
                    return Some(node);
                }
                stack.push(node.borrow_mut().left.take());
                stack.push(node.borrow_mut().right.take());
            }
        }
        None
    }
}

#[cfg(test)]
mod leetcode700 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        assert_eq!(None, Solution::search_bst(None, 0));
    }

    #[test]
    fn test1() {
        let node10 = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        })));
        assert_eq!(node10.clone(), Solution::search_bst(node10.clone(), 10));
    }

    #[test]
    fn test2() {
        let node10 = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        })));
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: node10.clone(),
        })));
        assert_eq!(node5.clone(), Solution::search_bst(node5.clone(), 5));
        assert_eq!(node10.clone(), Solution::search_bst(node5.clone(), 10));
    }
}
