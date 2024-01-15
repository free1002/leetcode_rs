pub fn minimum_sum(num: i32) -> i32 {
    let mut v: Vec<u8> = num.to_string().as_bytes().iter().map(|e| *e - ('0' as u8)).collect();
    v.sort();
    (v[0]*10 + v[2] + v[1]*10 + v[3]) as i32
}

#[test]
fn test_one() {
    assert_eq!(52, minimum_sum(2932));
    assert_eq!(13, minimum_sum(4009));
}

#[test]
fn test_least() {
    let i = 1;
    let j = 3;
    assert_eq!((0, 2), least(i, j));
}

fn least(i: usize, j: usize) -> (usize, usize) {
    let mut result = Vec::new();
    for e in 0..4 {
        if e != i && e != j {
            result.push(e);
        }
    }
    (result[0], result[1])
}