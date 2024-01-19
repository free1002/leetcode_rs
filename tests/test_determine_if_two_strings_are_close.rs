use std::collections::BTreeMap;

pub fn close_strings_1(word1: String, word2: String) -> bool {
    let count_map1: BTreeMap<char, usize> = create_counter(word1);
    let count_map2: BTreeMap<char, usize> = create_counter(word2);

    if count_map1.len() != count_map2.len() {
        return false;
    }

    // key check
    for key in count_map1.keys() {
        if !count_map2.contains_key(key) {
            return false;
        }
    }

    // count map 기준, key 의 set 이 같으면서, 대응되는 value 값이 같아야 함
    // 갯수 list
    let mut v1 = count_map1.values().cloned().collect::<Vec<usize>>();
    let mut v2 = count_map2.values().cloned().collect::<Vec<usize>>();
    v1.sort();
    v2.sort();

    v1 == v2
}

pub fn close_strings(word1: String, word2: String) -> bool {
    let mut array1: [usize; 26] = [0; 26];
    for c in word1.chars() {
        array1[c as usize - 'a' as usize] += 1;
    }
    let mut array2: [usize; 26] = [0; 26];
    for c in word2.chars() {
        array2[c as usize - 'a' as usize] += 1;
    }

    let mut value_vec1 = vec![];
    let mut value_vec2 = vec![];
    for i in 0..26 {
        // check key
        if array1[i] > 0 && array2[i] == 0 || array1[i] == 0 && array2[i] > 0 {
            return false;
        }
        if array1[i] > 0 {
            value_vec1.push(array1[i]);
        }
        if array2[i] > 0 {
            value_vec2.push(array2[i]);
        }
    }

    value_vec1.sort();
    value_vec2.sort();
    value_vec1 == value_vec2
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

    let word1 = "aaabbbbccddeeeeefffff".to_string();
    let word2 = "aaaaabbcccdddeeeeffff".to_string();
    // assert!(!close_strings_1(word1, word2));
    assert!(!close_strings(word1, word2));
}

fn create_counter(word: String) -> BTreeMap<char, usize> {
    let mut map = BTreeMap::new();
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