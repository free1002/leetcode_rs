pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
    let mut idx = 0;
    let mut prev = -101;
    let last = nums.last().unwrap().clone();
    for _ in 0..nums.len() {
        let current = nums[idx];
        if current == prev {
            nums.remove(idx);
        }
        else {
            if current == last {
                break;
            }
            prev = current;
            idx += 1;
        }
    }
    idx as i32 + 1
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut array: [bool; 300] = [false; 300];
    let mut start = nums[0];
    let mut end = nums[nums.len()-1];
    for e in &mut *nums {
        // array idx : -100 ~ 100
        let array_idx = (*e + 100) as usize;
        array[array_idx] = true;
    }
    let mut idx = 0;
    for i in start..end+1 {
        let array_idx = (i + 100) as usize;
        if array[array_idx] {
            nums[idx] = i;
            idx += 1;
        }
    }
    idx as i32
}

#[test]
fn test_one() {
    let mut nums = vec![1,1,2];
    let expected = vec![1,2];
    assert_eq!(2, remove_duplicates(&mut nums));
    for e in 0..2 {
        assert_eq!(nums[e], expected[e]);
    }

    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let expected = vec![0,1,2,3,4];
    assert_eq!(5, remove_duplicates(&mut nums));
    for e in 0..5 {
        assert_eq!(nums[e], expected[e]);
    }

    let mut nums = vec![0,0,0,0,3];
    let expected = vec![0,3];
    assert_eq!(2, remove_duplicates(&mut nums));
    for e in 0..2 {
        assert_eq!(nums[e], expected[e]);
    }
}

#[test]
fn test_two() {
    let mut nums = vec![1, 2, 2];
    let expected = vec![1, 2];
    assert_eq!(2, remove_duplicates(&mut nums));
    for e in 0..2 {
        assert_eq!(nums[e], expected[e]);
    }
}

#[test]
fn test_three() {
    let mut nums = vec![-1,0,0,0,0,3,3];
    let expected = vec![-1, 0, 3];
    assert_eq!(3, remove_duplicates(&mut nums));
    for e in 0..3 {
        assert_eq!(nums[e], expected[e]);
    }
}

#[test]
fn test_four() {
    let mut nums = vec![98,99,99,100];
    let expected = vec![98, 99, 100];
    assert_eq!(3, remove_duplicates(&mut nums));
    for e in 0..3 {
        assert_eq!(nums[e], expected[e]);
    }

}