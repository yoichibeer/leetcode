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
    // pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    //     fn recursive(
    //         node: Option<Rc<RefCell<TreeNode>>>,
    //         target_sum: i32,
    //         current_sum: i32,
    //     ) -> bool {
    //         match node {
    //             None => false,
    //             Some(node) => {
    //                 let current_sum = current_sum + node.borrow().val;
    //                 if node.borrow().left.is_none()
    //                     && node.borrow().right.is_none()
    //                     && (target_sum == current_sum)
    //                 {
    //                     true
    //                 } else {
    //                     recursive(node.borrow().left.clone(), target_sum, current_sum)
    //                         || recursive(node.borrow().right.clone(), target_sum, current_sum)
    //                 }
    //             }
    //         }
    //     }
    //     recursive(root, target_sum, 0)
    // }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut stack = vec![(root, 0)];
        let mut answer = false;
        while let Some((node, mut current_sum)) = stack.pop() {
            match node {
                None => continue,
                Some(node) => {
                    current_sum += node.borrow().val;
                    if (current_sum == target_sum)
                        && node.borrow().left.is_none()
                        && node.borrow().right.is_none()
                    {
                        answer = true;
                        break;
                    } else {
                        stack.push((node.borrow().left.clone(), current_sum));
                        stack.push((node.borrow().right.clone(), current_sum));
                    }
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod leetcode112 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        assert_eq!(false, Solution::has_path_sum(None, 0));
    }

    #[test]
    fn test1() {
        // [1]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 0));
        assert_eq!(true, Solution::has_path_sum(node, 1));
    }

    #[test]
    fn test2() {
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
        assert_eq!(false, Solution::has_path_sum(node.clone(), 0));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 1));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 2));
        assert_eq!(true, Solution::has_path_sum(node, 3));
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
        assert_eq!(false, Solution::has_path_sum(node.clone(), 1));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 2));
        assert_eq!(true, Solution::has_path_sum(node.clone(), 3));
        assert_eq!(true, Solution::has_path_sum(node.clone(), 4));
        assert_eq!(false, Solution::has_path_sum(node, 5));
    }

    #[test]
    fn test4() {
        // [1,null,2,3]
        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node3,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: node2,
        })));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 1));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 2));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 3));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 4));
        assert_eq!(false, Solution::has_path_sum(node.clone(), 5));
        assert_eq!(true, Solution::has_path_sum(node, 6));
    }
}
