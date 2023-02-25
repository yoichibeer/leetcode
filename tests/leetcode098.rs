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
    //     fn recursive(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    //         match node {
    //             None => true,
    //             Some(node) => {
    //                 let val = node.borrow().val as i64;
    //                 if val <= min || max <= val {
    //                     return false;
    //                 }
    //                 let answer = Self::recursive(node.borrow_mut().left.take(), min, val);
    //                 if !answer {
    //                     return false;
    //                 }
    //                 Self::recursive(node.borrow_mut().right.take(), val, max)
    // // Self::recursive(node.borrow().left.clone(), min, val)
    // //    && Self::recursive(node.borrow().right.clone(), val, max)
    //             }
    //         }
    //     }
    //     pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //         Self::recursive(root, -2147483649, 2147483648)
    //     }

    fn recursive(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(node) => {
                let val = node.borrow().val as i64;
                if val <= min || max <= val {
                    return false;
                }
                let answer = Self::recursive(node.borrow_mut().left.take(), min, val);
                if !answer {
                    return false;
                }
                Self::recursive(node.borrow_mut().right.take(), val, max)
            }
        }
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::recursive(root, -2147483649, 2147483648)
    }
}

#[cfg(test)]
mod leetcode098 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test1() {
        let node10 = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        })));
        assert_eq!(true, Solution::is_valid_bst(node10));
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
            right: node10,
        })));

        assert_eq!(true, Solution::is_valid_bst(node5));
    }
    #[test]
    fn test2b() {
        let node10 = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        })));
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: node10,
            right: None,
        })));

        assert_eq!(false, Solution::is_valid_bst(node5));
    }

    #[test]
    fn test5() {
        // [4,2,7,1,3]
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let node7 = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: node5,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node1,
            right: node3,
        })));
        let node4 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: node2,
            right: node7,
        })));

        assert_eq!(true, Solution::is_valid_bst(node4));
    }

    #[test]
    fn test222() {
        // [2,2,2]
        let node2a = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node2b = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node2a,
            right: node2b,
        })));
        assert_eq!(false, Solution::is_valid_bst(node2));
    }
    #[test]
    fn test2147483647() {
        let node2147483647 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2147483647,
            left: None,
            right: None,
        })));
        assert_eq!(true, Solution::is_valid_bst(node2147483647));
    }
}
