use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    // TODO: tree traversal
    // if root is None, return 0
    // if root.val is in range, add to sum
    // if root.val is less than low, go right
    // if root.val is greater than high, go left
    // if root.val is in range, go left and right
    // return sum
    let mut sum = 0;
    if let Some(node) = root {
        let node = node.borrow();
        if node.val >= low && node.val <= high {
            sum += node.val;
        }
        if node.val < low {
            sum += range_sum_bst(node.right.clone(), low, high);
        }
        if node.val > high {
            sum += range_sum_bst(node.left.clone(), low, high);
        }
        if node.val >= low && node.val <= high {
            sum += range_sum_bst(node.left.clone(), low, high);
            sum += range_sum_bst(node.right.clone(), low, high);
        }
    }
    sum
}


#[test]
fn test_one() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 18,
                left: None,
                right: None
            })))
        })))
    })));

    assert_eq!(32, range_sum_bst(root, 7, 15));
}

#[test]
fn test_two() {
    // [10,5,15,3,7,13,18,1,null,6]
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 18,
                left: None,
                right: None
            })))
        })))
    })));
    assert_eq!(23, range_sum_bst(root, 6, 10));
}
