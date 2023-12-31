use std::collections::HashMap;

fn convert(num: i32) -> String {
    let mut num_vec = vec![1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];
    let mut roman_vec = vec!["I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M"];

    num_vec.reverse();
    roman_vec.reverse();

    let mut n = num;
    let mut current_idx = 0;
    let mut result_str = String::new();

    loop {
        let current_num = num_vec[current_idx];
        if n >= current_num {
            let quotient = n / current_num;
            let remainder = n % current_num;
            result_str.push_str(&roman_vec[current_idx].repeat(quotient as usize));
            n = remainder;
        }
        current_idx += 1;
        if current_idx == num_vec.len() {
            break;
        }
    }

    result_str
}

#[test]
fn test_one() {
    assert_eq!(convert(3), "III");
    assert_eq!(convert(58), "LVIII");
    assert_eq!(convert(1994), "MCMXCIV");
}

#[test]
fn test_map() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("I", 1);
    map.insert("V", 5);
    map.insert("X", 10);

    assert_eq!(10, *map.get("X").unwrap());
    assert_eq!(5, *map.get("V").unwrap());
}

#[test]
fn test_map2() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "I");
    assert_eq!("I", *map.get(&1).unwrap());
}


#[test]
fn test_divide() {
    println!("{}", 3 / 5);
}


#[test]
fn test_reverse() {
    let mut num_vec = vec![1, 5, 10, 50, 100, 500, 1000];
    num_vec.reverse();
    // assert_eq!(1000, num_vec[0]);
    // assert_eq!(2, *num_vec.index(100));
    println!("{:?}", num_vec.iter().position(|&x| x == 100).unwrap());
    // println!("{:?}", num_vec[0]);

}