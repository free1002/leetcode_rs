pub fn min_falling_path_sum_1(matrix: Vec<Vec<i32>>) -> i32 {
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

pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
    for i in (0..matrix.len()).rev() {
        update_sum_tree_inplace(&mut matrix, i);
    }
    matrix[0].iter().fold(i32::MAX, |acc, e| acc.min(*e))
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


fn update_sum_tree_inplace(m: &mut Vec<Vec<i32>>, row_idx: usize) {
    for col_idx in 0..m[row_idx].len() {
        m[row_idx][col_idx] += find_min_child_inplace(m, row_idx, col_idx);
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
        if i >= m[row_idx].len() {
            continue;
        }

        let current_child = m[child_row_idx][i];
        let child_value = current_child.0 + current_child.1 as i32;
        min_value = min_value.min(child_value);
    }

    min_value
}

fn find_min_child_inplace(m: &Vec<Vec<i32>>, row_idx: usize, col_idx: usize) -> i32 {
    let child_row_idx = row_idx + 1;
    if child_row_idx >= m.len() {
        return 0;
    }
    let last_idx = m[child_row_idx].len() - 1;

    if col_idx == 0 {
        return m[child_row_idx][0].min(m[child_row_idx][1]);
    } else if col_idx == last_idx {
        return m[child_row_idx][last_idx-1].min(m[child_row_idx][last_idx]);
    }

    m[child_row_idx][col_idx-1].min(
        m[child_row_idx][col_idx]).min(
        m[child_row_idx][col_idx+1])
}

#[test]
fn test_one() {
    let matrix = vec![
        vec![2, 1, 3],
        vec![6, 5, 4],
        vec![7, 8, 9],
    ];
    assert_eq!(min_falling_path_sum(matrix), 13);

    let matrix = vec![
        vec![-19, 57],
        vec![-40, -5],
    ];
    assert_eq!(min_falling_path_sum(matrix), -59);
}

#[test]
fn test_two() {
    let matrix = vec![
        vec![-80, -13, 22],  // -66
        vec![83, 94, -5],  // -53
        vec![73, -48, 61],
    ];

    /*
        [[-45, 22, -31],
          [35, 46, -53],
          [73, -48, 61]]

     */
    assert_eq!(min_falling_path_sum(matrix), -66);
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

