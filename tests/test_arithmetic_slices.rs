// pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
//     let subsequences = find_subsequence(nums);
//     let mut count = 0;
//     for e in &subsequences {
//         // println!("{:?}", e);
//         if is_arithmetic_slices(e) {
//             count += 1;
//         }
//     }
//     count
// }

fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    find_arithmetic_slices(&nums) as i32
}

fn find_arithmetic_slices(num: &Vec<i32>) -> usize {
    let mut count = 0;
    for idx in 0..num.len() {
        let subseq = vec![num[idx]];
        _find_arithmetric_slices(num, idx, &subseq, &mut count);
    }
    count
}

fn _find_arithmetric_slices(num: &Vec<i32>, c_idx: usize, current_seq: &Vec<i32>, out_count: &mut usize) {
    if c_idx >= num.len() {
        return;
    }

    // current_seq + current_idx
    let mut new_subseq = current_seq.clone();
    new_subseq.push(num[c_idx]);
    // next : new_subseq + for each idx

    _find_arithmetric_slices(&num, c_idx+1, out_count);
}


fn is_arithmetic_slices(subseq: &Vec<i32>) -> bool {
    if subseq.len() < 3 {
        return false;
    }

    let diff = subseq[1].checked_sub(subseq[0]);
    if diff.is_none() {
        return false;
    }
    let diff = diff.unwrap();

    for i in 1..subseq.len()-1 {
        // overflow
        let z = subseq[i+1].checked_sub(subseq[i]);
        match z {
            Some(v) => {
                if v != diff {
                    return false;
                }
            },
            None => {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_one() {
    let nums = vec![2,4,6,8,10];
    assert_eq!(7, number_of_arithmetic_slices(nums));

    let nums = vec![7,7,7,7,7];
    assert_eq!(16, number_of_arithmetic_slices(nums));
}

#[test]
fn test_two() {
    let nums = vec![0,2000000000,-294967296];
    println!("{:?}", nums);
    assert_eq!(0, number_of_arithmetic_slices(nums));
}

#[test]
fn test_three() {
    let nums = vec![-2147483648,0,-2147483648];
    println!("{:?}", nums);
    assert_eq!(0, number_of_arithmetic_slices(nums));
}

#[test]
fn test_four() {
    let nums = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    println!("{:?}", nums);
    assert_eq!(0, number_of_arithmetic_slices(nums));
}


#[test]
fn test_subsequence() {
    let nums = vec![1,2,3,4];
    let result = find_subsequence(nums);

    /*
    expected :
        1
        2
        3
        4
        1 2
        1 3
        1 4
        2 3
        2 4
        3 4
        1 2 3
        1 2 4
        1 3 4
        2 3 4
        1 2 3 4
    */
    let expected = vec![
        vec![1], vec![2], vec![3], vec![4],
        vec![1,2], vec![1,3], vec![1,4], vec![2,3], vec![2,4], vec![3,4],
        vec![1,2,3], vec![1,2,4], vec![1,3,4], vec![2,3,4],
        vec![1,2,3,4]
    ];
    assert_eq!(result.len(), expected.len());
    for e in &result {
        println!("{:?}", e);
    }
    let expected_set = expected.iter().collect::<std::collections::HashSet<_>>();
    for e in &result {
        assert!(expected_set.contains(e));
    }
}

fn find_subsequence(num: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result_vec: Vec<Vec<i32>> = Vec::new();
    _aggregate_subsequence(&num, 0, &mut result_vec);

    result_vec
}

fn _aggregate_subsequence(num: &Vec<i32>, idx: i32, out_vec: &mut Vec<Vec<i32>>) {
    println!("current idx : {}, out : {:?}", idx, out_vec);
    if idx == num.len() as i32 {
        return;
    }

    let mut new_vec: Vec<Vec<i32>> = Vec::new();
    for e in out_vec.iter_mut() {
        let mut new_e = e.clone();
        new_e.push(num[idx as usize]);
        new_vec.push(new_e);
    }
    out_vec.append(&mut new_vec);
    out_vec.push(vec![num[idx as usize]]);

    _aggregate_subsequence(num, idx+1, out_vec);
}
