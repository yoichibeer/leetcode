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
    // fn dfs(node: Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
    //     if let Some(n) = node {
    //         Self::dfs(n.borrow_mut().left.take(), answer);
    //         answer.push(n.borrow().val);
    //         Self::dfs(n.borrow_mut().right.take(), answer);
    //     }
    //     // if let Some(n) = node {
    //     //     Self::dfs(n.borrow().left.clone(), answer);
    //     //     answer.push(n.borrow().val);
    //     //     Self::dfs(n.borrow().right.clone(), answer);
    //     // }
    // }
    // pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut answer: Vec<i32> = vec![];
    //     Self::dfs(root, &mut answer);
    //     answer
    // }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer = vec![];
        let mut stack = vec![];
        let mut cur = root;
        while cur.is_some() || !stack.is_empty() {
            while let Some(some) = cur {
                cur = some.borrow_mut().left.take();
                stack.push(some);
            }
            let some = stack.pop().unwrap();
            answer.push(some.borrow().val);
            cur = some.borrow_mut().right.take();
        }
        answer
    }
}

#[cfg(test)]
mod leetcode094 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        let v: Vec<i32> = vec![];
        assert_eq!(v, Solution::inorder_traversal(None));
    }

    #[test]
    fn test1() {
        // [1]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(vec![1], Solution::inorder_traversal(node));
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
        assert_eq!(vec![2, 1], Solution::inorder_traversal(node));
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
        assert_eq!(vec![1, 2], Solution::inorder_traversal(node));
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
        assert_eq!(vec![2, 1, 3], Solution::inorder_traversal(node));
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
        assert_eq!(vec![1, 3, 2], Solution::inorder_traversal(node));
    }
}
