use std::collections::HashMap;

pub fn close_strings(word1: String, word2: String) -> bool {
    let count_map1: HashMap<char, usize> = create_counter(word1);
    let count_map2: HashMap<char, usize> = create_counter(word2);

    if count_map1.len() != count_map2.len() {
        return false;
    }

    // count map 기준, key 의 set 이 같으면서, 대응되는 value 값이 같아야 함


    let mut v1 = count_map1.values().cloned().collect::<Vec<usize>>();
    let mut v2 = count_map2.values().cloned().collect::<Vec<usize>>();
    v1.sort();
    v2.sort();

    let mut key_set1 = count_map1.keys().cloned().collect::<Vec<char>>();
    let mut key_set2 = count_map2.keys().cloned().collect::<Vec<char>>();
    key_set1.sort();
    key_set2.sort();

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    v1 == v2 && key_set1 == key_set2
}


#[test]
fn test_one() {
    let word1 = "abc".to_string();
    let word2 = "bca".to_string();
    assert!(close_strings(word1, word2));

    let word1 = "a".to_string();
    let word2 = "aa".to_string();
    assert!(!close_strings(word1, word2));

    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    assert!(close_strings(word1, word2));

    let word1 = "uau".to_string();
    let word2 = "ssx".to_string();
    assert!(!close_strings(word1, word2));
}

fn create_counter(word: String) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in word.chars() {
        map.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    map
}

#[test]
fn test_create_counter() {
    let word1 = "aabccc".to_string();
    let count_map = create_counter(word1);
    let mut values: Vec<usize> = count_map.values().cloned().collect();
    values.sort();
    assert_eq!(vec![1, 2, 3], values);
}