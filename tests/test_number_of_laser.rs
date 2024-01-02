fn count_in_string(s: &String, c:char) -> usize {
    // s.chars().into_iter().filter(|e| *e == c).count()
    let mut count = 0;
    for e in s.chars() {
        if e == c {
            count += 1;
        }
    }
    count
}

fn next_dev_row_count(p0: &Vec<usize>, dev_row: usize) -> usize {
    if dev_row >= p0.len() {
        return 0;
    }

    for i in (dev_row+1)..(p0.len()) {
        if p0[i] > 0 {
            return p0[i];
        }
    }
    0
}

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut dev_row_vec: Vec<usize> = Vec::new();
    for row in bank {
        let dev_count = count_in_string(&row, '1');
        dev_row_vec.push(dev_count);
    }
    let mut result = 0;
    for i in 0..dev_row_vec.len() {
        if dev_row_vec[i] == 0 {
            continue;
        }
        result += next_dev_row_count(&dev_row_vec, i) * dev_row_vec[i];
    }

    result as i32
}

pub fn number_of_beams2(bank: Vec<String>) -> i32 {
    if bank.len() <= 1 {
        return 0;
    }

    let mut prev_count = count_in_string(&bank[0], '1');
    let mut result = 0;
    for row in bank[1..].iter() {
        let count = count_in_string(&row, '1');
        if count == 0 {
            continue;
        }
        result += prev_count * count;
        prev_count = count;
    }

    result as i32
}


#[test]
fn test_num_of_beams() {
    let bank: Vec<String> = vec![
        "011001".to_string(),
        "000000".to_string(),
        "010100".to_string(),
        "001000".to_string()
    ];
    let result = number_of_beams(bank);
    assert_eq!(8, result);
}

#[test]
fn test_num_of_beams2() {
    let bank: Vec<String> = vec![
        "011001".to_string(),
        "000000".to_string(),
        "010100".to_string(),
        "001000".to_string()
    ];
    let result = number_of_beams2(bank);
    assert_eq!(8, result);
}

#[test]
fn test_num_of_beams3() {
    let bank: Vec<String> = vec![
        "000000".to_string(),
        "000000".to_string(),
        "010100".to_string(),
        "001000".to_string()
    ];
    let result = number_of_beams2(bank);
    assert_eq!(2, result);
}

#[test]
fn test_one() {
    let row_vec = vec![3, 0, 2, 1];
    assert_eq!(2, next_dev_row_count(&row_vec, 0));
    assert_eq!(1, next_dev_row_count(&row_vec, 2));
}


#[test]
fn test_count_in_string() {
    let s = "011001".to_string();
    assert_eq!(3, count_in_string(&s, '1'));
}

