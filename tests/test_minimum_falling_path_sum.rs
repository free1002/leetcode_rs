pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut m = create_sum_matrix(matrix);
    for i in (0..m.len()).rev() {
        update_sum_tree(&mut m, i);
    }

    let mut min_value = i32::MAX;
    for i in 0..m[0].len() {
        let v = m[0][i].0 + m[0][i].1;
        min_value = min_value.min(v);
    }

    min_value
}


#[test]
fn test_one() {

}

fn create_sum_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<(i32, i32)>> {
    let mut result: Vec<Vec<(i32, i32)>> = Vec::new();
    for row in matrix {
        let row_vec = row.iter().map(|e| (*e, 0)).collect();
        result.push(row_vec);
    }
    result
}

fn update_sum_tree(m: &mut Vec<Vec<(i32, i32)>>, row_idx: usize) {
    for col_idx in 0..m[row_idx].len() {
        let v = find_min_child(m, row_idx, col_idx);
        m[row_idx][col_idx].1 = v;
    }
}

fn find_min_child(m: &Vec<Vec<(i32, i32)>>, row_idx: usize, col_idx: usize) -> i32 {
    let child_row_idx = row_idx + 1;
    if child_row_idx >= m.len() {
        return 0;
    }
    let mut min_value = i32::MAX;
    let col_start = if col_idx > 0 { col_idx-1 } else { 0 };

    for i in col_start..col_idx+2 {
        if i < 0 || i >= m[row_idx].len() {
            continue;
        }

        let current_child = m[child_row_idx][i];
        let child_value = current_child.0 + current_child.1 as i32;
        min_value = min_value.min(child_value);
    }

    min_value
}

#[test]
fn test_path_sum_tree() {
    let matrix = vec![
        vec![6,5,4],
        vec![7,8,9]
    ];

    // create sum tree
    let mut m = create_sum_matrix(matrix);
    println!("{:?}", m);
    assert_eq!(m[0][0], (6, 0));
    assert_eq!(m[1][0], (7, 0));

    assert_eq!(find_min_child(&m, 0, 0), 7);
    assert_eq!(find_min_child(&m, 0, 1), 7);
    assert_eq!(find_min_child(&m, 0, 2), 8);

    update_sum_tree(&mut m, 0);
    println!("{:?}", m);
    assert_eq!(m[0][0], (6, 7));
    assert_eq!(m[0][1], (5, 7));
    assert_eq!(m[0][2], (4, 8));
    assert_eq!(m[1][0], (7, 0));
}

#[test]
fn test_path_sum2() {
    let matrix = vec![
        vec![2, 1, 3],
        vec![6, 5, 4],
        vec![7, 8, 9],
    ];

    let mut m = create_sum_matrix(matrix);
    update_sum_tree(&mut m, 1);
    update_sum_tree(&mut m, 0);

    assert_eq!(m[0][0], (2, 12));
    assert_eq!(m[0][1], (1, 12));
    assert_eq!(m[0][2], (3, 12));
}
