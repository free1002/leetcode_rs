use std::collections::HashMap;

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // 각 row 들은 각각 다른 숫자들이 들어가야 함
    // 이를 위한 row 는 최소가 되어야 함
    // 1 3 4 1 2 3 1
    // ->
    // 1 1 1 3 3 2 4
    // row 의 최대 수 : 최대 중복 갯수만큼
    /*
    1 3 4
    1 3
    1 2
    */
    // count and sort by count
    // let hash_set = nums.iter().collect::<std::collections::HashSet<_>>();
    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in nums {
        map.entry(e).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x,y| y.1.cmp(&x.1));

    // create rows
    let first = v[0];
    let mut result_vec: Vec<Vec<i32>> = Vec::new();
    for i in 0..first.1 {
        result_vec.push(vec![first.0]);
    }

    // assign least
    for (number, repeat) in &v[1..] {
        for i in 0..*repeat {
            result_vec[i as usize].push(*number);
        }
    }

    result_vec
}


pub fn find_matrix2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result_vec: Vec<Vec<i32>> = vec![vec![]];

    nums.sort();
    result_vec[0].push(nums[0]);

    let mut row_idx = 0;
    let mut prev = nums[0];
    for e in &nums[1..] {
        if prev == *e {
            row_idx += 1;
            if row_idx >= result_vec.len() {
                result_vec.push(vec![]);
            }
            result_vec[row_idx].push(*e);
        } else {
            row_idx = 0;
            result_vec[row_idx].push(*e);
        }
        prev = *e;
    }

    result_vec
}

#[test]
fn test_one() {
    let nums = vec![1, 3, 4, 1, 2, 3, 1];
    let expected = vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]];
    let result = find_matrix2(nums);
    assert_eq!(result.len(), 3);
    for idx in 0..result.len() {
        let row1_set = expected[idx].iter().collect::<std::collections::HashSet<_>>();
        let row2_set = result[idx].iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(row1_set, row2_set);
    }
}

#[test]
fn test_two() {
    let nums = vec![1, 2, 3, 4];
    let expected = vec![vec![1, 4, 2, 3]];
    let result = find_matrix2(nums);
    assert_eq!(result.len(), 1);
    for idx in 0..result.len() {
        let row1_set = expected[idx].iter().collect::<std::collections::HashSet<_>>();
        let row2_set = result[idx].iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(row1_set, row2_set);
    }

}

#[test]
fn test_counter() {
    let nums = vec![1, 3, 4, 1, 2, 3, 1];
    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in nums {
        map.entry(e).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x,y| y.1.cmp(&x.1));

    // create row and assign
    println!("{:?}", &v);
    for e in &v[1..] {
        println!("key : {}, repeat : {}", e.0, e.1);
    }
}

#[test]
fn test_sort() {
    let mut nums = vec![1, 3, 4, 1, 2, 3, 1];
    nums.sort();
    println!("{:?}", nums);
    for (a,b) in nums.iter().zip(nums.iter().skip(1)) {
        println!("a : {}, b : {}", a, b);
        if a == b {
            println!("same");
        }
        else {
            println!("not same");
        }
    }
}
