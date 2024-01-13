fn is_increasing(s: &Vec<i32>) -> bool {
    if s.len() == 1 {
        return true;
    }

    for (a, b) in s.iter().zip(s.iter().skip(1)) {
        if a >= b {
            return false;
        }
    }
    true
}


pub fn length_of_lis1(nums: Vec<i32>) -> i32 {
    let subsequences = generate_subsequences(nums);
    let mut max_length = 0;
    for e in &subsequences {
        if is_increasing(e) {
            println!("{:?}", e);
            max_length = max_length.max(e.len());
        }
    }
    max_length as i32
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    // generate_longest_subsequences(nums).len() as i32
    generate_longest_subsequences2(nums).len() as i32
}


#[test]
fn test_one() {
    //  subsequence and not sub-array.
    // Subsequence can have some missing numbers, but order should be same

    // 2 3 7 101
    let nums = vec![10,9,2,5,3,7,101,18];
    assert_eq!(4, length_of_lis(nums));
    //
    let nums = vec![7,7,7,7,7,7,7];
    assert_eq!(1, length_of_lis(nums));

    // 0 1 2 3
    let nums = vec![0,1,0,3,2,3];
    assert_eq!(4, length_of_lis(nums));
}

#[test]
fn test_two() {
    let nums = vec![1,3,6,7,9,4,10,5,6];
    assert_eq!(6, length_of_lis(nums));
}

#[test]
fn test_three() {
    let nums = vec![7,7,7,7,7,7,7];
    assert_eq!(1, length_of_lis(nums));
}

#[test]
fn test_four() {
    let nums = vec![0,1,0,3,2,3];
    // assert_eq!(4, length_of_lis(nums));
    generate_longest_subsequences2(nums);
}

fn generate_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out_vec: Vec<Vec<i32>> = Vec::new();
    for i in 0..nums.len() {
        let v = vec![nums[i]];
        out_vec.push(v.clone());
        _generate_subsequences(&nums, i+1, &v, &mut out_vec);
    }
    out_vec
}


fn generate_longest_subsequences(nums: Vec<i32>) -> Vec<i32> {
    let mut max_subsequences = vec![nums[0]];

    for i in 0..nums.len() {
        let v = vec![nums[i]];
        _generate_longest_subsequences(&nums, i+1, &v, &mut max_subsequences);
    }
    max_subsequences.clone()
}

struct Seq {
    seq: Vec<i32>,
    start: usize,
    end: usize
}

impl Seq {
    fn new(seq: Vec<i32>, idx: usize) -> Self {
        Seq {
            seq,
            start: idx,
            end: idx
        }
    }
}


fn generate_longest_subsequences2(nums: Vec<i32>) -> Vec<i32> {
    // one scan algorithm
    let mut subsequences = vec![vec![nums[0]]];
    for i in 1..nums.len() {
        let current_v = nums[i];
        subsequences.push(vec![current_v]);

        println!("----------------------");
        println!("current_subseq : {:?}", subsequences);
        for j in 0..subsequences.len() {
            let mut sub = &mut subsequences[j];
            // sub last 값 보다 current_v 가 크면 : append
            if sub[sub.len()-1] < current_v {
                sub.push(current_v);
                println!("appended : {:?}", sub);
                continue;
            }
            // sub 에서 current_v 보다 작은 값이 있는 경우 : create new sub sequence (divide)
            for j in (0..sub.len()).rev() {
                if sub[j] < current_v {
                    // create new sub sequence - slide
                    let mut new_sub: Vec<i32> = sub[0..j+1].to_vec();
                    new_sub.push(current_v);
                    println!("divided : {:?}[:{}] [{}] - {:?}", sub.clone(), j, current_v, new_sub);
                    subsequences.push(new_sub.clone());
                    break;
                }
            }
        }
    }
    // println!("subsequences : {:?}", subsequences);
    // determine longest subsequence
    let mut max_length = 0;
    let mut max_subsequence = vec![];
    for sub in subsequences {
        if sub.len() > max_length {
            max_length = sub.len();
            max_subsequence = sub;
        }
    }
    max_subsequence
}


fn _generate_longest_subsequences(nums: &Vec<i32>, idx: usize, prev_sequence: &Vec<i32>, candidate: &mut Vec<i32>) {
    // i : nums 에서 i idx 만큼 이미 소비한 상태
    // prev_sequence : 앞 노드에서 이미 생성한 sequence
    if idx == nums.len() {
        return;
    }

    // new sequence
    for i in idx..nums.len() {
        let mut v = prev_sequence.clone();
        v.push(nums[i]);
        if !is_increasing(&v) {
            continue;
        }
        if v.len() > candidate.len() {
            candidate.clear();
            candidate.append(&mut v.clone());
            println!("updated : {:?}", v);
        }
        _generate_longest_subsequences(nums, i+1, &v, candidate);
    }
}


fn _generate_subsequences(nums: &Vec<i32>, idx: usize, prev_sequence: &Vec<i32>, out_vec: &mut Vec<Vec<i32>>) {
    // i : nums 에서 i idx 만큼 이미 소비한 상태
    // prev_sequence : 앞 노드에서 이미 생성한 sequence
    if idx == nums.len() {
        return;
    }

    // new sequence
    for i in idx..nums.len() {
        let mut v = prev_sequence.clone();
        v.push(nums[i]);
        if !is_increasing(&v) {
            continue;
        }
        println!("added : {:?}", v);
        out_vec.push(v.clone());
        _generate_subsequences(nums, i+1, &v, out_vec);
    }
}


#[test]
fn test_generate_subsequence() {
    let nums = vec![1,2,3,4];
    let result = generate_subsequences(nums);
    println!("--------------------------");
    for e in &result {
        println!("{:?}", e);
    }
    assert_eq!(15, result.len());
}


#[test]
fn test_generate_longest_subsequence() {
    let nums = vec![1,2,3,4];
    let result = generate_longest_subsequences(nums);
    assert_eq!(4, result.len());
}


#[test]
fn test_longest_increasing_subsequence() {
    let nums = vec![1,2,3,4];
    let result = length_of_lis(nums);
    assert_eq!(4, result);
}

#[test]
fn test_memory() {
    let nums = (1..2501).collect::<Vec<i32>>();
    let result = length_of_lis(nums);
    assert_eq!(2500, result);
}

#[test]
fn test_memory2() {
    let nums = vec![-813, 82, -728, -82, -432, 887, -551, 324, -315, 306, -164, -499, -873, -613, 932, 177,
                    61, 52, 1000, -710, 372, -306, -584, -332, -500, 407, 399, -648, 290, -866, 222, 562, 993, -338,
                    -590, 303, -16, -134, 226, -648, 909, 582, 177, 899, -343, 55, 629, 248, 333, 1, -921, 143,
                    629, 981, -435, 681, 844, 349, 613, 457, 797, 695, 485, 15, 710, -450, -775, 961, -445, -905,
                    466, 942, 995, -289, -397, 434, -14, 34, -903, 314, 862, -441, 507, -966, 525, 624, -706,
                    39, 152, 536, 874, -364, 747, -35, 446, -608, -554, -411, 987, -354, -700, -34, 395, -977,
                    544, -330, 596, 335, -612, 28, 586, 228, -664, -841, -999, -100, -620, 718, 489, 346, 450,
                    772, 941, 952, -560, 58, 999, -879, 396, -101, 897, -1000, -566, -296, -555, 938, 941, 475,
                    -260, -52, 193, 379, 866, 226, -611, -177, 507, 910, -594, -856, 156, 71, -946, -660, -716,
                    -295, -927, 148, 620, 201, 706, 570, -659, 174, 637, -293, 736, -735, 377, -687, -962, 768,
                    430, 576, 160, 577, -329, 175, 51, 699, -113, 950, -364, 383, 5, 748, -250, -644, -576, -227,
                    603, 832, -483, -237, 235, 893, -336, 452, -526, 372, -418, 356, 325, -180, 134, -698];

    let result = length_of_lis(nums);
    println!("{}", result);
}


#[test]
fn test_generate_longest_subsequence2() {
    let nums = vec![1,2,3,4];
    let result = generate_longest_subsequences2(nums);
    println!("{:?}", result);
}

#[test]
fn test_subvector() {
    let nums = vec![1,2,3,4];
    let result = &nums[0..2];
    let new_vec = result.to_vec();
    println!("{:?}", new_vec);
}

#[test]
fn test_reverse_range() {
    println!("{:?}", (0..10).rev().collect::<Vec<i32>>());
}


// #[test]
// fn test_generate_longest_subsequence3() {
//     let nums = vec![1,2,3,4];
//     let result = longest_increasing_sequences_number(nums);
//     println!("{:?}", result);
// }
//
//
// struct TreeNode {
//     val: i32,
//     children: Vec<TreeNode>,
// }
//
// impl TreeNode {
//     fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             children: vec![],
//         }
//     }
// }
//
// fn longest_increasing_sequences_number(nums: Vec<i32>) -> i32 {
//     // TODO: create tree structure, update tree for each num, find max depth
//     let mut tree_list: Vec<&TreeNode> = Vec::new();
//
//     for i in 0..nums.len() {
//         let current_v = nums[i];
//         // find
//
//
//         // create default node
//         let mut tree = TreeNode::new(current_v);
//         tree_list.push(&tree);
//
//     }
//
//     0
// }
