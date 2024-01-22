pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    let mut nums_table: [i32; 10000] = [-1; 10000];
    for i in 0..nums.len() {
        nums_table[nums[i] as usize - 1] += 1;
    }
    let mut missed = -1;
    let mut dup = -1;
    for i in 0..nums.len() {
        if nums_table[i] == -1 {
            missed = i as i32 + 1;
        } else if nums_table[i] == 1 {
            dup = i as i32 + 1;
        }
    }
    vec![dup, missed]
}


#[test]
fn test_one() {
    let nums = vec![1,2,2,4];
    assert_eq!(vec![2,3], find_error_nums(nums));

    let nums = vec![1,1];
    assert_eq!(vec![1,2], find_error_nums(nums));

    let nums = vec![3,2,2];
    assert_eq!(vec![2,1], find_error_nums(nums));

    let nums = vec![3,2,3,4,6,5];
    assert_eq!(vec![3,1], find_error_nums(nums));
}
