pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return vec![];
    }

    let mut result: Vec<String> = vec![];
    let mut start = nums[0];
    let mut end = start;
    for e in nums[1..].iter() {

        if *e == end + 1 {
            // update end
            end = *e;
            continue;
        }
        else {
            // add chunk
            if start == end {
                let s = format!("{}", start);
                result.push(s);
            }
            else {
                let s = format!("{}->{}", start, end);
                result.push(s);
            }

            // and new chunk
            start = *e;
            end = *e;
        }
    }
    // last chunk
    if start == end {
        let s = format!("{}", start);
        result.push(s);
    }
    else {
        let s = format!("{}->{}", start, end);
        result.push(s);
    }

    println!("{:?}", result);

    result
}

#[test]
fn test_one() {
    let nums = vec![0,1,2,4,5,7];
    let expected = vec!["0->2","4->5","7"];
    let result = summary_ranges(nums);
    assert_eq!(result.len(), 3);
    for idx in 0..result.len() {
        assert_eq!(expected[idx], result[idx]);
    }
}

#[test]
fn test_two() {
    let nums = vec![0,2,3,4,6,8,9];
    let expected = vec!["0", "2->4", "6", "8->9"];
    let result = summary_ranges(nums);
    assert_eq!(result.len(), 4);
    for idx in 0..result.len() {
        assert_eq!(expected[idx], result[idx]);
    }
}

#[test]
fn test_three() {
    let nums = vec![];
    let expected: Vec<String> = vec![];
    let result = summary_ranges(nums);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_four() {
    let nums = vec![-1, 2];
    let expected= vec!["-1", "2"];
    let result = summary_ranges(nums);
    assert_eq!(result.len(), 2);
    for idx in 0..result.len() {
        assert_eq!(expected[idx], result[idx]);
    }
}
