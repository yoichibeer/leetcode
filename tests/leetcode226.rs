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
    // fn recursive(node: &mut Option<Rc<RefCell<TreeNode>>>) {
    //     match node {
    //         None => (),
    //         Some(node) => {
    //             let left = node.borrow_mut().left.take();
    //             let right = node.borrow_mut().right.take();
    //             node.borrow_mut().left = right;
    //             node.borrow_mut().right = left;
    //             Self::recursive(&mut node.borrow().left.clone());
    //             Self::recursive(&mut node.borrow().right.clone());
    //         }
    //     }
    // }
    // pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     let mut answer = root.clone();
    //     Self::recursive(&mut answer);
    //     answer
    // }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let answer = root;
        let mut stack = vec![answer.clone()];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                stack.push(left.clone());
                stack.push(right.clone());
                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
            }
        }
        answer
    }
}

#[cfg(test)]
mod leetcode226 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        assert_eq!(None, Solution::invert_tree(None));
    }

    #[test]
    fn test1() {
        // [1]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(node, Solution::invert_tree(node.clone()));
    }

    #[test]
    fn test2_1() {
        // [1,2]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node_input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node.clone(),
            right: None,
        })));
        let node_expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: node.clone(),
        })));
        assert_eq!(node_expected, Solution::invert_tree(node_input));
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
        let node_input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node2.clone(),
            right: node3.clone(),
        })));
        let node_expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node3.clone(),
            right: node2.clone(),
        })));
        assert_eq!(node_expected, Solution::invert_tree(node_input));
    }

    #[test]
    fn test4() {
        // [1,null,2,3]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node2a = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node.clone(),
            right: None,
        })));
        let node2b = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: node.clone(),
        })));
        let node_input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: node2a,
        })));
        let node_expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node2b,
            right: None,
        })));
        assert_eq!(node_expected, Solution::invert_tree(node_input));
    }

    #[test]
    fn test7() {
        // [4,2,7,1,3,6,9]
        let node9 = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })));
        let node6 = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        })));
        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node7a = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: node6.clone(),
            right: node9.clone(),
        })));
        let node7b = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: node9.clone(),
            right: node6.clone(),
        })));
        let node2a = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: node1.clone(),
            right: node3.clone(),
        })));
        let node2b = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: node3.clone(),
            right: node1.clone(),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: node2a,
            right: node7a,
        })));
        let root_expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: node7b,
            right: node2b,
        })));
        assert_eq!(root_expected, Solution::invert_tree(root));
    }
}
