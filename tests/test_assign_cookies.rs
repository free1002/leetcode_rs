pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    let mut g_idx = 0;
    let mut s_idx = 0;
    let mut count = 0;
    g.sort();
    g.reverse();
    s.sort();
    s.reverse();
    // g.sort_by(|a, b| b.cmp(a));
    // s.sort_by(|a, b| b.cmp(a));

    // while g_idx < g.len() && s_idx < s.len() {
    loop {
        if g_idx == g.len() {
            break;
        }
        if s_idx == s.len() {
            break;
        }

        println!("{:?} : g_idx: {}, {:?} : s_idx: {}", g, g_idx, s, s_idx);
        if g[g_idx] <= s[s_idx] {
            count += 1;
            g_idx += 1;
            s_idx += 1;
        }
        else {
            // skip children greed
            g_idx += 1;
        }
    }

    count
}

#[test]
fn test_one() {
    let g = vec![1, 2, 3];
    let s = vec![1, 1];
    let result = find_content_children(g, s);
    assert_eq!(1, result);
}

#[test]
fn test_two() {
    let g = vec![1, 2];
    let s = vec![1, 2, 3];
    let result = find_content_children(g, s);
    assert_eq!(2, result);
}

#[test]
fn test_three() {
    let g = vec![1, 2, 3];
    let s = vec![];
    let result = find_content_children(g, s);
    assert_eq!(0, result);
}

#[test]
fn test_four() {
    let g = vec![10, 9, 8, 7];
    let s = vec![5, 6, 7, 8];
    let result = find_content_children(g, s);
    assert_eq!(2, result);
}
