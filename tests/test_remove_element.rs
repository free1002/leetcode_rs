fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut total = 0;
    let length = nums.len();
    for i in (0..nums.len()).rev() {
        if nums[i] == val {
            nums.swap_remove(i);
            total += 1;
        }
    }

    length as i32 - total
}

#[test]
fn test_one() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let result = remove_element(&mut nums, val);
    assert_eq!(2, result);
    println!("{:?}", nums);

    let expected = vec![2, 2];
    let expected_set = expected.iter().collect::<std::collections::HashSet<_>>();
    let sub_vec_set = nums[0..2].iter().collect::<std::collections::HashSet<_>>();
    assert_eq!(expected_set, sub_vec_set);
}

#[test]
fn test_two() {
let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let result = remove_element(&mut nums, val);
    assert_eq!(5, result);
    println!("{:?}", nums);

    let sub_vec_set = nums[0..5].iter().collect::<std::collections::HashSet<_>>();

    let expected = vec![0, 1, 3, 0, 4];
    let expected_set = expected.iter().collect::<std::collections::HashSet<_>>();
    assert_eq!(expected_set, sub_vec_set);
}


#[test]
fn test_range() {
    for e in (0..10).rev() {
        println!("{}", e);
    }
}