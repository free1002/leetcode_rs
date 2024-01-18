pub fn climb_stairs(n: i32) -> i32 {
    let mut result = 1;
    let mut n_2 = 1;
    let mut n_1 = 2;

    if n <= 2 {
        return n;
    }

    for i in 3..n {
        result = n_1 + n_2;
        n_2 = n_1;
        n_1 = result;
    }

    n_1+n_2
}

#[test]
fn test_one() {
    assert_eq!(2, climb_stairs(2));
    assert_eq!(3, climb_stairs(3));
    assert_eq!(5, climb_stairs(4));
    assert_eq!(8, climb_stairs(5));
}