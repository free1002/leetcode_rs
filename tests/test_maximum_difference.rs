use std::cell::RefCell;
use std::rc::Rc;
use std::thread::current;

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

pub fn create_tree_node(val: i32,
                        left: Option<Rc<RefCell<TreeNode>>>,
                        right: Option<Rc<RefCell<TreeNode>>>
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode {val, left, right})))
}

pub fn create_leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    create_tree_node(val, None, None)
}


pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root_v = root.as_ref().unwrap().borrow().val;
    let (min_v, max_v) = _max_ancester_diff(root.as_ref().unwrap(), root_v, root_v);
    min_v.abs_diff(max_v) as i32
}

pub fn _max_ancester_diff(x: &RefCell<TreeNode>, path_min_v: i32, path_max_v: i32) -> (i32, i32) {
    let current_val = x.borrow().val;
    let current_node = x.borrow();

    let min_v = current_val.min(path_min_v);
    let max_v = current_val.max(path_max_v);
    let current_diff = min_v.abs_diff(max_v);
    let mut out_vec = vec![(current_diff, min_v, max_v)];

    if let Some(left) = &current_node.left {
        let (left_min_v, left_max_v) = _max_ancester_diff(left.as_ref(), min_v, max_v);
        let diff = left_min_v.abs_diff(left_max_v);
        out_vec.push((diff, left_min_v, left_max_v));
    }
    if let Some(right) = &current_node.right {
        let (right_min_v, right_max_v) = _max_ancester_diff(right.as_ref(), min_v, max_v);
        let diff = right_min_v.abs_diff(right_max_v);
        out_vec.push((diff, right_min_v, right_max_v));
    }

    // find max diff
    out_vec.sort_by(|(diff1, _, _), (diff2, _, _)| diff1.cmp(diff2));
    let (_, min_v, max_v) = out_vec.pop().unwrap();
    // println!("[{}] prev: {}/{}, new : {}/{}", current_val, path_min_v, path_max_v, min_v, max_v);
    (min_v, max_v)
}


#[test]
fn test_one() {
    let root = create_tree_node(8,
                                create_tree_node(3,
                                                 create_leaf(1),
                                                 create_tree_node(6,
                                                                  create_leaf(4),
                                                                  create_leaf(7),
                                                 ),
                                ),
                                create_tree_node(10,
                                                 None,
                                                 create_tree_node(14,
                                                                  create_leaf(13),
                                                                  None,
                                                 ),
                                ),
    );
    assert_eq!(7, max_ancestor_diff(root));
}

#[test]
fn test_two() {
    let root = create_tree_node(1,
                                None,
                                create_tree_node(2,
                                                 None,
                                                 create_tree_node(0,
                                                                  create_leaf(3),
                                                                  None,
                                                 ),
                                ),
    );
    assert_eq!(3, max_ancestor_diff(root));
}
