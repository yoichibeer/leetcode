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
    // fn dfs(node: Option<Rc<RefCell<TreeNode>>>, answer: &mut usize, depth: usize) {
    //     if let Some(node) = node {
    //         if *answer < depth {
    //             *answer = depth;
    //         }
    //         //*answer = std::cmp::max(*answer, depth);
    //         Self::dfs(node.borrow_mut().left.take(), answer, depth + 1);
    //         Self::dfs(node.borrow_mut().right.take(), answer, depth + 1);
    //     }
    // }
    // pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut answer = 0;
    //     Self::dfs(root, &mut answer, 1);
    //     answer as i32
    // }

    // pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut answer = 0;
    //     let mut stack = vec![];
    //     stack.push((root, 1));
    //     while let Some((node, depth)) = stack.pop() {
    //         if let Some(node) = node {
    //             let depth = depth;
    //             // if answer < depth {
    //             //     answer = depth;
    //             // }
    //             answer = std::cmp::max(answer, depth);
    //             stack.push((node.borrow().right.clone(), depth + 1));
    //             stack.push((node.borrow().left.clone(), depth + 1));
    //         }
    //     }
    //     answer
    // }

    // https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/1770060/c-recursive-dfs-example-dry-run-well-explained/?envType=study-plan&id=data-structure-i
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let max_left = Self::max_depth(node.borrow_mut().left.take());
            let max_right = Self::max_depth(node.borrow_mut().right.take());
            std::cmp::max(max_left, max_right) + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod leetcode104 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        assert_eq!(0, Solution::max_depth(None));
    }

    #[test]
    fn test1() {
        // [1]
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(1, Solution::max_depth(node));
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
        assert_eq!(2, Solution::max_depth(node));
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
        assert_eq!(2, Solution::max_depth(node));
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
        assert_eq!(2, Solution::max_depth(node));
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
        assert_eq!(2, Solution::max_depth(node));
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
        assert_eq!(3, Solution::max_depth(node));
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
        assert_eq!(3, Solution::max_depth(node));
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
        assert_eq!(3, Solution::max_depth(node));
    }
}
