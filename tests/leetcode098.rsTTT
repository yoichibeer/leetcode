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
    fn recursive(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, bool) {
        (0, 0, true)
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        true
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

    // #[test]
    // fn test5() {
    //     // [4,2,7,1,3]
    //     let node1 = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 1,
    //         left: None,
    //         right: None,
    //     })));
    //     let node3 = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 3,
    //         left: None,
    //         right: None,
    //     })));
    //     let node7 = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 7,
    //         left: None,
    //         right: None,
    //     })));
    //     let node2 = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: node1,
    //         right: node3,
    //     })));
    //     let node4 = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 4,
    //         left: node2,
    //         right: node7,
    //     })));

    //     // [4,2,7,1,3,5]
    //     let node1b = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 1,
    //         left: None,
    //         right: None,
    //     })));
    //     let node3b = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 3,
    //         left: None,
    //         right: None,
    //     })));
    //     let node5b = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 5,
    //         left: None,
    //         right: None,
    //     })));
    //     let node7b = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 7,
    //         left: node5b,
    //         right: None,
    //     })));
    //     let node2b = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: node1b,
    //         right: node3b,
    //     })));
    //     let node4b = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 4,
    //         left: node2b,
    //         right: node7b,
    //     })));
    //     assert_eq!(node4b, Solution::insert_into_bst(node4, 5));
    // }

    // #[test]
    // fn test7a() {
    //     let node40;
    //     {
    //         // [40,20,60,10,30,50,70]
    //         let node10 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 10,
    //             left: None,
    //             right: None,
    //         })));
    //         let node30 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 30,
    //             left: None,
    //             right: None,
    //         })));
    //         let node20 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 20,
    //             left: node10,
    //             right: node30,
    //         })));
    //         let node50 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 50,
    //             left: None,
    //             right: None,
    //         })));
    //         let node70 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 70,
    //             left: None,
    //             right: None,
    //         })));
    //         let node60 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 60,
    //             left: node50,
    //             right: node70,
    //         })));
    //         node40 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 40,
    //             left: node20,
    //             right: node60,
    //         })));
    //     }

    //     let node40b;
    //     {
    //         // [40,20,60,10,30,50,70,null,null,25]
    //         let node25 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 25,
    //             left: None,
    //             right: None,
    //         })));
    //         let node10 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 10,
    //             left: None,
    //             right: None,
    //         })));
    //         let node30 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 30,
    //             left: node25,
    //             right: None,
    //         })));
    //         let node20 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 20,
    //             left: node10,
    //             right: node30,
    //         })));
    //         let node50 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 50,
    //             left: None,
    //             right: None,
    //         })));
    //         let node70 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 70,
    //             left: None,
    //             right: None,
    //         })));
    //         let node60 = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 60,
    //             left: node50,
    //             right: node70,
    //         })));
    //         node40b = Some(Rc::new(RefCell::new(TreeNode {
    //             val: 40,
    //             left: node20,
    //             right: node60,
    //         })));
    //     }
    //     assert_eq!(node40b, Solution::insert_into_bst(node40, 25));
    // }
}
