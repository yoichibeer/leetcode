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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let edge = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        let mut stack = vec![root.clone()];
        while let Some(node) = stack.pop() {
            match node {
                None => return edge,
                Some(node) => {
                    if node.borrow().val < val {
                        let right = node.borrow().right.clone();
                        if right.is_none() {
                            node.borrow_mut().right = edge;
                            break;
                        } else {
                            stack.push(right);
                        }
                    } else if node.borrow().val > val {
                        let left = node.borrow().left.clone();
                        if left.is_none() {
                            node.borrow_mut().left = edge;
                            break;
                        } else {
                            stack.push(left);
                        }
                    } else {
                        assert!(false);
                    }
                }
            }
        }
        root
    }

    // pub fn insert_into_bst(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     val: i32,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     match root {
    //         Some(node) => {
    //             let node_val = node.borrow().val;
    //             if node_val < val {
    //                 let right = Self::insert_into_bst(node.borrow_mut().right.take(), val);
    //                 node.borrow_mut().right = right;
    //                 Some(node)
    //             } else if val < node_val {
    //                 let left = Self::insert_into_bst(node.borrow_mut().left.take(), val);
    //                 node.borrow_mut().left = left;
    //                 Some(node)
    //             } else {
    //                 assert!(false);
    //                 None
    //             }
    //         }
    //         None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
    //     }
    // }

    // fn bst(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
    //     if let Some(node) = node {
    //         let cur = node.borrow().val;
    //         // println!("AA{},{}", cur, val);
    //         if cur < val {
    //             if node.borrow().right.is_none() {
    //                 node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    //             } else {
    //                 let right = node.borrow_mut().right.take();
    //                 if right.is_some() {
    //                     let right = right.unwrap();
    //                     let right_val = right.borrow().val;
    //                     if val < right_val {
    //                         node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
    //                             val,
    //                             left: None,
    //                             right: Some(right),
    //                         })));
    //                     } else {
    //                         Self::bst(&mut Some(right), val);
    //                     }
    //                 }
    //             }
    //         } else if cur > val {
    //             if node.borrow().left.is_none() {
    //                 node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    //             } else {
    //                 let left = node.borrow_mut().left.take();
    //                 if left.is_some() {
    //                     let left = left.unwrap();
    //                     let left_val = left.borrow().val;
    //                     if val > left_val {
    //                         node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
    //                             val,
    //                             left: Some(left),
    //                             right: None,
    //                         })));
    //                     } else {
    //                         Self::bst(&mut Some(left), val);
    //                     }
    //                 }
    //             }
    //         } else {
    //             assert!(false);
    //         }
    //     }
    // }
    // pub fn insert_into_bst(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     val: i32,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     let mut node = root;
    //     Self::bst(&mut node, val);
    //     node
    // }
}

#[cfg(test)]
mod leetcode701 {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test0() {
        // []
        let node0 = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        assert_eq!(node0, Solution::insert_into_bst(None, 0));
    }

    #[test]
    fn test1() {
        let node10 = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        })));
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: node5,
            right: None,
        })));
        assert_eq!(expected, Solution::insert_into_bst(node10, 5));
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

        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node5b = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: node3,
            right: node10.clone(),
        })));
        assert_eq!(node5b, Solution::insert_into_bst(node5, 3));
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
        let node7 = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
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

        // [4,2,7,1,3,5]
        let node1b = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node3b = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node5b = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let node7b = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: node5b,
            right: None,
        })));
        let node2b = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node1b,
            right: node3b,
        })));
        let node4b = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: node2b,
            right: node7b,
        })));
        assert_eq!(node4b, Solution::insert_into_bst(node4, 5));
    }

    #[test]
    fn test7a() {
        let node40;
        {
            // [40,20,60,10,30,50,70]
            let node10 = Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: None,
            })));
            let node30 = Some(Rc::new(RefCell::new(TreeNode {
                val: 30,
                left: None,
                right: None,
            })));
            let node20 = Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: node10,
                right: node30,
            })));
            let node50 = Some(Rc::new(RefCell::new(TreeNode {
                val: 50,
                left: None,
                right: None,
            })));
            let node70 = Some(Rc::new(RefCell::new(TreeNode {
                val: 70,
                left: None,
                right: None,
            })));
            let node60 = Some(Rc::new(RefCell::new(TreeNode {
                val: 60,
                left: node50,
                right: node70,
            })));
            node40 = Some(Rc::new(RefCell::new(TreeNode {
                val: 40,
                left: node20,
                right: node60,
            })));
        }

        let node40b;
        {
            // [40,20,60,10,30,50,70,null,null,25]
            let node25 = Some(Rc::new(RefCell::new(TreeNode {
                val: 25,
                left: None,
                right: None,
            })));
            let node10 = Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: None,
            })));
            let node30 = Some(Rc::new(RefCell::new(TreeNode {
                val: 30,
                left: node25,
                right: None,
            })));
            let node20 = Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: node10,
                right: node30,
            })));
            let node50 = Some(Rc::new(RefCell::new(TreeNode {
                val: 50,
                left: None,
                right: None,
            })));
            let node70 = Some(Rc::new(RefCell::new(TreeNode {
                val: 70,
                left: None,
                right: None,
            })));
            let node60 = Some(Rc::new(RefCell::new(TreeNode {
                val: 60,
                left: node50,
                right: node70,
            })));
            node40b = Some(Rc::new(RefCell::new(TreeNode {
                val: 40,
                left: node20,
                right: node60,
            })));
        }
        assert_eq!(node40b, Solution::insert_into_bst(node40, 25));
    }
}
