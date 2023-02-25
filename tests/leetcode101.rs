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
    fn recursive(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                if left.borrow().val != right.borrow().val {
                    return false;
                }
                let outside =
                    Self::recursive(left.borrow().left.clone(), right.borrow().right.clone());
                let inside =
                    Self::recursive(left.borrow().right.clone(), right.borrow().left.clone());
                outside && inside
            }
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => Self::recursive(node.borrow().left.clone(), node.borrow().right.clone()),
            // Some(node) => Self::recursive(
            //     node.borrow_mut().left.take(),
            //     node.borrow_mut().right.take(),
            // ),
            None => true,
        }
    }

    // pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     if root.is_none() {
    //         return true;
    //     }
    //     assert!(root.is_some());
    //     let root = root.unwrap();
    //     let mut stack_left: Vec<Option<Rc<RefCell<TreeNode>>>> =
    //         vec![root.borrow_mut().left.take()];
    //     let mut stack_right: Vec<Option<Rc<RefCell<TreeNode>>>> =
    //         vec![root.borrow_mut().right.take()];
    //     loop {
    //         let left = stack_left.pop();
    //         let right = stack_right.pop();
    //         match (left, right) {
    //             (Some(left), Some(right)) => match (left, right) {
    //                 (Some(left), Some(right)) => {
    //                     if left.borrow().val != right.borrow().val {
    //                         break false;
    //                     }
    //                     stack_left.push(left.borrow_mut().right.take());
    //                     stack_left.push(left.borrow_mut().left.take());
    //                     stack_right.push(right.borrow_mut().left.take());
    //                     stack_right.push(right.borrow_mut().right.take());
    //                 }
    //                 (None, None) => {}
    //                 _ => break false,
    //             },
    //             (None, None) => break true,
    //             _ => break false,
    //         }
    //     }
    // }

    // pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     if root.is_none() {
    //         return true;
    //     }
    //     assert!(root.is_some());
    //     let root = root.unwrap();
    //     let mut stack_left: Vec<Option<Rc<RefCell<TreeNode>>>> =
    //         vec![root.borrow_mut().left.take()];
    //     let mut stack_right: Vec<Option<Rc<RefCell<TreeNode>>>> =
    //         vec![root.borrow_mut().right.take()];
    //     let mut count = 0;
    //     loop {
    //         println!("count={}", count);
    //         count += 1;
    //         let left = stack_left.pop();
    //         let right = stack_right.pop();
    //         match (left, right) {
    //             (Some(left), Some(right)) => match (left, right) {
    //                 (Some(left), Some(right)) => {
    //                     if left.borrow().val != right.borrow().val {
    //                         println!(
    //                             "break false; Some(left={}), Some(right={})",
    //                             left.borrow().val,
    //                             right.borrow().val
    //                         );
    //                         break false;
    //                     }
    //                     println!(
    //                         "continue; Some(left={}), Some(right={})",
    //                         left.borrow().val,
    //                         right.borrow().val
    //                     );
    //                     stack_left.push(left.borrow_mut().right.take());
    //                     stack_left.push(left.borrow_mut().left.take());
    //                     stack_right.push(right.borrow_mut().left.take());
    //                     stack_right.push(right.borrow_mut().right.take());
    //                 }
    //                 (Some(left), None) => {
    //                     println!("break false; Some(left={}), None", left.borrow().val);
    //                     break false;
    //                 }
    //                 (None, Some(right)) => {
    //                     println!("break false; None, Some(right={})", right.borrow().val);
    //                     break false;
    //                 }
    //                 (None, None) => {
    //                     println!("continue None, None");
    //                 }
    //             },
    //             (None, None) => {
    //                 println!("break true; (None, None)");
    //                 break true;
    //             }
    //             (Some(left), None) => {
    //                 if let Some(left) = left {
    //                     println!("break false; ({}, None)", left.borrow().val);
    //                 } else {
    //                     println!("break false; ((None), None)",);
    //                 }
    //                 break false;
    //             }
    //             (None, Some(right)) => {
    //                 if let Some(right) = right {
    //                     println!("break false; ({}, None)", right.borrow().val);
    //                 } else {
    //                     println!("break false; ((None), None)",);
    //                 }
    //                 break false;
    //             }
    //         }
    //     }
    // }
}

#[cfg(test)]
mod leetcode101 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        assert!(Solution::is_symmetric(None));
    }

    #[test]
    fn test1() {
        // [1]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert!(Solution::is_symmetric(node));
    }

    #[test]
    fn test2_1() {
        // [1,2]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node,
            right: None,
        })));
        assert!(!Solution::is_symmetric(node));
    }

    #[test]
    fn test2_2() {
        // [1,2,2]
        let node_left = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node_right = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node_left,
            right: node_right,
        })));
        assert!(Solution::is_symmetric(node));
    }

    #[test]
    fn test3() {
        // [1,2,3]
        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node2,
            right: node3,
        })));
        assert!(!Solution::is_symmetric(node));
    }

    #[test]
    fn test3_1() {
        // [3,1,2]
        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node2,
            right: node3,
        })));
        assert!(!Solution::is_symmetric(node));
    }

    #[test]
    fn test4() {
        // [1,null,2,3]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: node,
        })));
        assert!(!Solution::is_symmetric(node));
    }

    #[test]
    fn test7() {
        // [1,2,2,3,4,4,3]
        let node_lhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node_rhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let node_l = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node_lhs,
            right: node_rhs,
        })));
        let node_lhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let node_rhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node_r = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node_lhs,
            right: node_rhs,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node_l,
            right: node_r,
        })));
        assert!(Solution::is_symmetric(node));
    }
}
