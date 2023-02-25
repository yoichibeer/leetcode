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
    // // (https://leetcode.com/problems/binary-tree-preorder-traversal/solutions/2918429/binary-tree-preorder-traversal/#approach-2-iteration)
    // pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut answer = vec![];
    //     let mut stack = vec![];
    //     stack.push(root);
    //     while let Some(node) = stack.pop() {
    //         if let Some(node) = node {
    //             answer.push(node.borrow().val);
    //             stack.push(node.borrow_mut().right.take());
    //             stack.push(node.borrow_mut().left.take());
    //         }
    //     }
    //     answer
    // }

    // (https://leetcode.com/problems/binary-tree-preorder-traversal/solutions/2918429/binary-tree-preorder-traversal/)
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
        match node {
            Some(n) => {
                let ptr = Rc::as_ptr(&n);
                println!("{:p} n ptr", ptr);

                answer.push(n.borrow().val);
                println!("{:p} n.borrow_mut().left", &n.borrow_mut().left);
                let clone = n.borrow_mut().left.clone();
                println!("{:p} clone", &clone);
                let mut take = n.borrow_mut().left.take();
                println!("{:p} take", &take);
                if clone.is_some() && take.is_some() {
                    let clone_unwrap = clone.unwrap();
                    println!("{:p} clone unwrap", &clone_unwrap);

                    let ptr = Rc::as_ptr(&clone_unwrap);
                    println!("{:p} clone_unwrap ptr", ptr);

                    let take_unwrap = take.unwrap();
                    println!("{:p} take unwrap", &take_unwrap);

                    let ptr = Rc::as_ptr(&take_unwrap);
                    println!("{:p} take_unwrap ptr", ptr);

                    take = Some(take_unwrap);
                }
                Self::dfs(take, answer);
                Self::dfs(n.borrow_mut().right.take(), answer);
            }
            None => (),
        }
    }

    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![];
        Self::dfs(root, &mut answer);
        answer
    }
}

#[cfg(test)]
mod leetcode144 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    // #[test]
    // fn test1() {
    //     // [1]
    //     let node = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 1,
    //         left: None,
    //         right: None,
    //     })));
    //     assert_eq!(vec![1], Solution::preorder_traversal(node));
    // }

    #[test]
    fn test5() {
        // [1,null,2,3]
        let t = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        println!("{:p} 3 t", &t);
        let refcel = RefCell::new(t);
        println!("{:p} 3 refcel", &refcel);
        let rc = Rc::new(refcel);
        println!("{:p} 3 rc", &rc);
        let node = Some(rc);
        println!("{:p} 3 node", &node);

        let t = TreeNode {
            val: 2,
            left: node,
            right: None,
        };
        println!("{:p} 2 t", &t);
        let refcel = RefCell::new(t);
        println!("{:p} 2 refcel", &refcel);
        let rc = Rc::new(refcel);
        println!("{:p} 2 rc", &rc);
        let node = Some(rc);
        println!("{:p} 2 node", &node);

        let t = TreeNode {
            val: 1,
            left: None,
            right: node,
        };
        println!("{:p} 1 t", &t);
        let refcel = RefCell::new(t);
        println!("{:p} 1 refcel", &refcel);
        let rc = Rc::new(refcel);
        println!("{:p} 1 rc", &rc);
        let rc_clone = rc.clone();

        let ptr = Rc::as_ptr(&rc);
        println!("{:p} 1 ptr", ptr);
        let ptr = Rc::as_ptr(&rc_clone);
        println!("{:p} 1 ptr clone", ptr);

        let node = Some(rc);
        println!("{:p} 1 node", &node);

        assert_eq!(vec![1, 2, 3], Solution::preorder_traversal(node));
    }
}
