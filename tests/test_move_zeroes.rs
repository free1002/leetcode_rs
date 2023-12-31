fn move_zeroes(nums: &mut Vec<i32>) {
    // let mut remove_list: Vec<usize> = Vec::new();
    //
    // for i in 0..nums.len() {
    //     if nums[i] == 0 {
    //         remove_list.push(i);
    //     }
    // }
    //
    // for idx in remove_list.iter().rev() {
    //     nums.remove(*idx);
    // }
    //
    // for _ in 0..remove_list.len() {
    //     nums.push(0);
    // }
    nums.sort_by_key(|&x| x == 0)
}


#[test]
fn test_one() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    assert_eq!(vec![1, 3, 12, 0, 0], nums);
}

#[test]
fn test_two() {
    let mut nums = vec![0];
    move_zeroes(&mut nums);
    assert_eq!(vec![0], nums);
}

#[test]
fn test_three() {
    let mut nums = vec![0, 0];
    move_zeroes(&mut nums);
    assert_eq!(vec![0, 0], nums);
}

#[test]
fn test_four() {
    let mut nums = vec![0, 0, 0, 0, 0, 0, 1, 1, 1];
    move_zeroes(&mut nums);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0, 0, 0], nums);
}


#[test]
fn test_performance() {
    for i in 0..10000 {
        let mut nums = vec![0, 0, 0, 0, 0, 0, 1, 1, 1];
        move_zeroes(&mut nums);
    }
}
