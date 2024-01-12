#![feature(pattern)]

use std::collections::HashSet;
use std::str::pattern::Pattern;

pub fn halves_are_alike_1(s: String) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let mut first_count = 0;
    let mut second_count = 0;
    let mid = s.len()/2;

    for i in 0..mid {
        if chars[i].is_contained_in("aeiouAEIOU") {
            first_count += 1;
        }
        if chars[mid+i].is_contained_in("aeiouAEIOU") {
            second_count += 1;
        }
    }
    first_count == second_count
}


pub fn halves_are_alike(s: String) -> bool {
    let chars = s.as_bytes();
    let mut first_count: u16 = 0;
    let mut second_count: u16 = 0;
    let mid = s.len()/2;
    let is_vowel = |x: u8| -> bool {
        match x {
            97 | 101 | 105 | 111 | 117 | 65 | 69 | 73 | 79 | 85 => true,
            _ => false
        }
    };

    for i in 0..mid {
        if is_vowel(chars[i]) {
            first_count += 1;
        }
        if is_vowel(chars[mid+i]) {
            second_count += 1;
        }
    }
    first_count == second_count
}


#[test]
fn test_one() {
    assert!(halves_are_alike("book".to_string()));
    assert!(!halves_are_alike("textbook".to_string()));
}

#[test]
fn test_two() {
    let s = "aeiouAEIOU";
    for e in s.as_bytes() {
        println!("{}", e);
    }
}
