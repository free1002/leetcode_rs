use std::collections::HashMap;

fn single_number(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in nums {
        map.entry(e).and_modify(|v| *v += 1).or_insert(1);
    }
    map.iter().find_map(|(k, v)| if *v == 1 { Some(*k) } else { None }).unwrap()
    // *(map.iter().find(|(_, v)| **v == 1).unwrap().0)
}


#[test]
fn test_one() {
    assert_eq!(1, single_number(vec![2, 2, 1]));
    assert_eq!(4, single_number(vec![4, 1, 2, 1,2]));
    assert_eq!(1, single_number(vec![1]));
}