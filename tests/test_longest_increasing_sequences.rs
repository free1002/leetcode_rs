pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    // vec : (head, tail, count)
    // let mut seq_vec: Vec<(i32, i32, i32)> = Vec::new();
    //
    // for e in nums {
    //     let mut counted = false;
    //     for i in 0..seq_vec.len() {
    //         let (head, tail, count) = seq_vec[i];
    //         if head < e && tail < e {
    //             // count
    //             seq_vec[i] = (head, e, count+1);
    //             counted = true;
    //             continue;
    //         }
    //         if head < e && tail > e {
    //             // update tail only
    //             seq_vec[i] = (head, e, count);
    //             counted = true;
    //             continue;
    //         }
    //     }
    //     if !counted {
    //         seq_vec.push((e, e, 1));
    //     }
    //     for row in &seq_vec {
    //         println!("[{}] {:?}", e, row);
    //     }
    //     println!("-------------------");
    // }
    //
    // // find max count
    // let mut max_count = -1;
    // for (_, _, c) in &seq_vec {
    //     if *c > max_count {
    //         max_count = *c;
    //     }
    // }
    //
    // max_count
    0
}

//
// #[test]
// fn test_one() {
//     //  subsequence and not sub-array.
//     // Subsequence can have some missing numbers, but order should be same
//
//     // 2 3 7 101
//     let nums = vec![10,9,2,5,3,7,101,18];
//     assert_eq!(4, length_of_lis(nums));
//     //
//     let nums = vec![7,7,7,7,7,7,7];
//     assert_eq!(1, length_of_lis(nums));
//
//     // 0 1 2 3
//     let nums = vec![0,1,0,3,2,3];
//     assert_eq!(4, length_of_lis(nums));
// }
//
// #[test]
// fn test_two() {
//     let nums = vec![1,3,6,7,9,4,10,5,6];
//     assert_eq!(6, length_of_lis(nums));
// }

struct SequenceTree {
    value: i32,
    children: Vec<SequenceTree>
}

impl SequenceTree {
    fn new(value: i32) -> SequenceTree {
        SequenceTree {
            value,
            children: Vec::new()
        }
    }

    pub fn append(&mut self, value: i32) {
        // find the right place to append
        let mut current_node = self;
        loop {
            if current_node.value < value {
                // append to the last child
                if current_node.children.len() == 0 {
                    current_node.children.push(SequenceTree::new(value));
                    break;
                }
                current_node = current_node.children.last_mut().unwrap();
            } else {
                // append to the current node
                current_node.children.push(SequenceTree::new(value));
                break;
            }
        }
    }

    pub fn print(&self, depth: i32=0) {
        println!("{}", self.value);
        for child in &self.children {
            child.print();
        }
    }
}

#[test]
fn test_tree() {
    let mut tree = SequenceTree::new(1);
    tree.append(2);
    // tree.print();
    tree.append(3);
    tree.append(6);
    tree.append(4);
    tree.print();
}