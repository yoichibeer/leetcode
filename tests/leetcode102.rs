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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    // fn recursive(node: Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<Vec<i32>>, layer: usize) {
    //     if let Some(node) = node {
    //         if answer.len() == layer {
    //             answer.push(vec![]);
    //         }
    //         answer.get_mut(layer).unwrap().push(node.borrow().val);
    //         let layer = layer + 1;
    //         Self::recursive(node.borrow_mut().left.take(), answer, layer);
    //         Self::recursive(node.borrow_mut().right.take(), answer, layer);
    //     }
    // }
    // pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //     let mut answer = vec![];
    //     Self::recursive(root, &mut answer, 0);
    //     answer
    // }

    // pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //     let mut answer = vec![];
    //     let mut stack = vec![];
    //     stack.push((root, 0));
    //     while let Some((node, layer)) = stack.pop() {
    //         if let Some(node) = node {
    //             if answer.len() == layer {
    //                 answer.push(vec![]);
    //             }
    //             answer[layer].push(node.borrow().val);
    //             stack.push((node.borrow().right.clone(), layer + 1));
    //             stack.push((node.borrow().left.clone(), layer + 1));
    //         }
    //     }
    //     answer
    // }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let mut len = queue.len();
            let mut list = vec![];
            while len > 0 {
                len -= 1;
                if let Some(node) = queue.pop_front() {
                    if let Some(node) = node {
                        list.push(node.borrow().val);
                        queue.push_back(node.borrow_mut().left.take());
                        queue.push_back(node.borrow_mut().right.take());
                    }
                }
            }
            if !list.is_empty() {
                answer.push(list);
            }
        }
        answer
    }
}

#[cfg(test)]
mod leetcode102 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        let v: Vec<Vec<i32>> = vec![];
        assert_eq!(v, Solution::level_order(None));
    }

    #[test]
    fn test1() {
        // [1]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(vec![vec![1]], Solution::level_order(node));
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
        assert_eq!(vec![vec![1], vec![2]], Solution::level_order(node));
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
        assert_eq!(vec![vec![1], vec![2]], Solution::level_order(node));
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
        assert_eq!(vec![vec![1], vec![2, 3]], Solution::level_order(node));
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
        assert_eq!(vec![vec![3], vec![1, 2]], Solution::level_order(node));
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
        assert_eq!(vec![vec![1], vec![2], vec![3]], Solution::level_order(node));
    }

    #[test]
    fn test7_5() {
        // [3,9,20,null,null,15,7]
        let node_lhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: None,
            right: None,
        })));
        let node_rhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));
        let node_rhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: node_lhs,
            right: node_rhs,
        })));
        let node_lhs = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node_lhs,
            right: node_rhs,
        })));
        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(node)
        );
    }

    #[test]
    fn test7() {
        // [1,2,3,4,5,6,7]
        let node3_1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let node3_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let node3_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        })));
        let node3_4 = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));
        let node2_1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node3_1,
            right: node3_2,
        })));
        let node2_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node3_3,
            right: node3_4,
        })));
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node2_1,
            right: node2_2,
        })));
        assert_eq!(
            vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]],
            Solution::level_order(node)
        );
    }
}
