pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut losers_array:[i16; 100001] = [-1; 100001];
    for e in matches {
        let winner = e[0] as usize;
        let loser = e[1] as usize;
        if losers_array[winner] == -1 {
            losers_array[winner] = 0;
        }
        if losers_array[loser] == -1 {
            losers_array[loser] = 0;
        }
        losers_array[loser] += 1;
    }

    let mut result1: Vec<i32> = Vec::new();
    let mut result2: Vec<i32> = Vec::new();
    for (idx, e) in losers_array.iter().enumerate() {
        match *e {
            0 => result1.push(idx as i32),
            1 => result2.push(idx as i32),
            _ => ()
        }
    }
    vec![result1, result2]
}

#[test]
fn test_one() {
    let matches = vec![
        vec![1, 3],
        vec![2, 3],
        vec![3, 6],
        vec![5, 6],
        vec![5, 7],
        vec![4, 5],
        vec![4, 8],
        vec![4, 9],
        vec![10, 4],
        vec![10, 9]
    ];
    let result = find_winners(matches);
    assert_eq!(vec![1,2,10], result[0]);
    assert_eq!(vec![4,5,7,8], result[1]);

    let matches = vec![
        vec![2,3],
        vec![1,3],
        vec![5,4],
        vec![6,4],
    ];
    let result = find_winners(matches);
    let empty: Vec<i32> = vec![];
    assert_eq!(vec![1,2,5,6], result[0]);
    assert_eq!(empty, result[1]);
}

#[test]
fn test_two() {
    let matches = vec![
        vec![1, 100000]
    ];
    let result = find_winners(matches);
    assert_eq!(vec![1], result[0]);
    assert_eq!(vec![100000], result[1]);
}
