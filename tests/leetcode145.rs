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
use std::rc::Rc;

pub struct Solution2 {
    pub answer: Vec<i32>,
}
impl Solution2 {
    pub fn postorder_traversal(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            self.postorder_traversal(node.borrow_mut().left.take());
            self.postorder_traversal(node.borrow_mut().right.take());
            self.answer.push(node.borrow().val);
        }
    }
}

pub struct Solution {}
impl Solution {
    // fn recursive(node: Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
    //     if let Some(n) = node {
    //         Self::recursive(n.borrow().left.clone(), answer);
    //         Self::recursive(n.borrow().right.clone(), answer);
    //         answer.push(n.borrow().val);
    //     }
    // }
    // pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut answer = vec![];
    //     Self::recursive(root, &mut answer);
    //     answer
    // }

    // pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut answer = vec![];
    //     let mut stack = vec![];
    //     stack.push(root);
    //     while let Some(node) = stack.pop() {
    //         if let Some(node) = node {
    //             stack.push(node.borrow().left.clone());
    //             stack.push(node.borrow().right.clone());
    //             answer.push(node.borrow().val);
    //         }
    //     }
    //     answer.reverse();
    //     answer
    // }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut solution2 = Solution2 { answer: vec![] };
        solution2.postorder_traversal(root);
        solution2.answer
    }
}

#[cfg(test)]
mod leetcode145 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        let v: Vec<i32> = vec![];
        assert_eq!(v, Solution::postorder_traversal(None));
    }

    #[test]
    fn test1() {
        // [1]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(vec![1], Solution::postorder_traversal(node));
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
        assert_eq!(vec![2, 1], Solution::postorder_traversal(node));
    }

    #[test]
    fn test2_2() {
        // [1,null,2]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: node,
        })));
        assert_eq!(vec![2, 1], Solution::postorder_traversal(node));
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
        assert_eq!(vec![2, 3, 1], Solution::postorder_traversal(node));
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
        assert_eq!(vec![1, 2, 3], Solution::postorder_traversal(node));
    }

    #[test]
    fn test5() {
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
        assert_eq!(vec![3, 2, 1], Solution::postorder_traversal(node));
    }
}
