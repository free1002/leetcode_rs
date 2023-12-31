use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for e in 0..nums.len() {
        map.entry(nums[e]).and_modify(|v| v.push(e as i32)).or_insert(vec![e as i32]);
    }

    for i in 0..nums.len() {
        let complement = target - nums[i];
        if let Some(complement_idx_vec) = map.get(&complement) {
            for complement_idx in complement_idx_vec {
                if *complement_idx != i as i32 {
                    return vec![i as i32, *complement_idx];
                }
            }
        }
    }

    vec![0, 0]
}

#[test]
fn test_one() {
    assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    assert_eq!(vec![0, 2], two_sum(vec![-3, 4, 3, 90], 0));
    assert_eq!(vec![0, 2], two_sum(vec![3, 4, -3, 90], 0));
    assert_eq!(vec![1, 2], two_sum(vec![2, 5, 5, 11], 10));
}
