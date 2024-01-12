use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
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

pub fn leaf_list(x: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let binding = x.unwrap();
    let xx= binding.as_ref();
    let mut list = Vec::new();
    _leaf_list(xx, &mut list);
    list
}

pub fn _leaf_list(x: &RefCell<TreeNode>, out_list: &mut Vec<i32>) {
    let xx = x.borrow();
    println!("val: {}", xx.val);
    if xx.left.is_none() && xx.right.is_none() {
        out_list.push(xx.val);
        return;
    }

    if let Some(left) = &xx.left {
        _leaf_list(left.as_ref(), out_list);
    }
    if let Some(right) = &xx.right {
        _leaf_list(right.as_ref(), out_list);
    }
}

pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let leaf_list1 = leaf_list(root1);
    let leaf_list2 = leaf_list(root2);
    leaf_list1 == leaf_list2
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

//
// #[test]
// fn test_traverse_generator() {
//     let root1 = create_tree_node(3,
//                                  create_tree_node(5,
//                                                   create_leaf(6),
//                                                   create_tree_node(2,
//                                                                    create_leaf(7),
//                                                                    create_leaf(4)
//                                                   )
//                                  ),
//                                  create_tree_node(1,
//                                                   create_leaf(9),
//                                                   create_leaf(8)
//                                  )
//     );
//     for i in leaf_list_generator(root1) {
//         println!("{}", i);
//     }
// }


// pub fn _traverse_leaf_list_generator(x: &RefCell<TreeNode>, out_list: &mut Vec<i32>) {
//     let xx = x.borrow();
//     println!("val: {}", xx.val);
//     if xx.left.is_none() && xx.right.is_none() {
//         __yield()
//         return;
//     }
//
//     if let Some(left) = &xx.left {
//         crate::_leaf_list(left.as_ref(), out_list);
//     }
//     if let Some(right) = &xx.right {
//         crate::_leaf_list(right.as_ref(), out_list);
//     }
// }


// fn leaf_list_generator(p0: Option<Rc<RefCell<TreeNode>>>) -> _ {
//
// }


#[test]
fn test_leaf_list() {
    let root1 = create_tree_node(3,
        create_tree_node(5,
            create_leaf(6),
            create_tree_node(2,
                create_leaf(7),
                create_leaf(4)
            )
        ),
        create_tree_node(1,
            create_leaf(9),
            create_leaf(8)
        )
    );
    let leaf_list: Vec<i32> = leaf_list(root1);
    println!("{:?}", leaf_list);
    assert_eq!(5, leaf_list.len());
}

#[test]
fn test_one() {
    let root1 = create_tree_node(3,
                                 create_tree_node(5,
                                                  create_leaf(6),
                                                  create_tree_node(2,
                                                                   create_leaf(7),
                                                                   create_leaf(4)
                                                  )
                                 ),
                                 create_tree_node(1,
                                                  create_leaf(9),
                                                  create_leaf(8)
                                 )
    );

    let root2 = create_tree_node(3,
                                 create_tree_node(5,
                                                  create_leaf(6),
                                                  create_leaf(7),
                                 ),
                                 create_tree_node(1,
                                                  create_leaf(4),
                                                  create_tree_node(2,
                                                                   create_leaf(9),
                                                                   create_leaf(8))
                                 )
    );
    assert!(leaf_similar(root1, root2))
}

#[test]
fn test_two() {
    let root1 = create_tree_node(1,
                                 create_leaf(2),
                                 create_leaf(3)
    );

    let root2 = create_tree_node(1,
                                 create_leaf(3),
                                 create_leaf(2)
    );

    assert!(!leaf_similar(root1, root2))
}
