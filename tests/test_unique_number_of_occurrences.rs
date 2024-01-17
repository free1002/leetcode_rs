use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in arr {
        map.entry(e).and_modify(|x| *x += 1).or_insert(1);
    }
    let set: HashSet<i32> = map.values().cloned().collect();
    set.len() == map.len()
}

#[test]
fn test_one() {
    let arr = vec![1,2,2,1,1,3];
    assert_eq!(true, unique_occurrences(arr));

    let arr = vec![1,2];
    assert_eq!(false, unique_occurrences(arr));

    let arr = vec![-3,0,1,-3,1,1,1,-3,10,0];
    assert_eq!(true, unique_occurrences(arr));
}