use std::collections::HashMap;


pub fn ops_for_representation_2_3(v: i32) -> i32 {
    // v = 2*x + 3*y 일 때 x,y 가 최소
    // ex) 10 :
    // 3 * 3 + 2 * 0
    // 3 * 2 + 2 * 2
    // 3 * 1 + 2 * 4
    // or return -1;

    let (q_3, r_3) = (v/3, v%3);
    if q_3 > 0 {
        // 3*x only
        if r_3 == 0 {
            return q_3;
        }
        if r_3 == 1 {
            return q_3-1 + 2; // 3 ops q_3-1 , 2 ops 는 항상 2
        }
        if r_3 == 2 {
            return q_3 + 1;
        }

    }

    // only divided by 2
    let (q_2, r_2) = (v / 2, v % 2);
    return if q_2 > 0 && r_2 == 0 {
        q_2
    }
    else {
        -1
    }
}

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in nums {
        map.entry(e).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut num_of_op = 0;
    for (k, v) in map.iter() {
        let ops = ops_for_representation_2_3(*v);
        if ops == -1 {
            return -1;
        }
        num_of_op += ops;
    }
    num_of_op
}


pub fn min_operations2(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in nums {
        map.entry(e).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut num_of_op = 0;
    for v in map.values() {
        if *v == 1 {
            return -1;
        }

        let q_3 = v/3;
        if q_3 > 0 {
            // 3*x only
            match v % 3 {
                0 => num_of_op += q_3,
                1 => num_of_op += q_3-1 + 2, // 3 ops q_3-1 , 2 ops 는 항상 2
                2 => num_of_op += q_3 + 1,
                _ => (),
            }
        }
        else {
            // only divided by 2
            num_of_op += v / 2;
        }
    }
    num_of_op
}



#[test]
fn test_one() {
    let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
    let result = min_operations2(nums);
    assert_eq!(4, result);

    let nums = vec![2, 1, 2, 2, 3, 3];
    let result = min_operations2(nums);
    assert_eq!(-1, result);
}

#[test]
fn test_two() {
    // 총 19개
    // 14 : 10 , 12 : 9
    // 12 제거 : 3 - 3
    // 14 제거 : 3 * 2 , 2 * 2
    let nums = vec![14, 12, 14, 14, 12, 14, 14, 12, 12, 12, 12, 14, 14, 12, 14, 14, 14, 12, 12];
    let result = min_operations2(nums);
    assert_eq!(7, result);
}

#[test]
fn test_three() {
    // 13 : 5
    // 7 : 4

    // 5 : 3 1 , 2 1
    // 4 : 2 2
    let nums = vec![13,7,13,7,13,7,13,13,7];
    let result = min_operations2(nums);
    assert_eq!(4, result);

}

#[test]
fn test_divmod() {
    let s = 4;
    let (quotient, remainder) = (s/2, s%2);
    // println!("{} {}", s/2, s%2);
    assert_eq!(quotient, 2);
    assert_eq!(remainder, 0);

    let s = 3;
    let (quotient, remainder) = (s/2, s%2);
    assert_eq!(quotient, 1);
    assert_eq!(remainder, 1);
}
