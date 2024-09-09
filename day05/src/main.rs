use std::fs;

fn main() {
    let input = fs::read_to_string("05.txt").unwrap();
    {
        let mut result = 0;
        for line in input.lines() {
            if has_three_vowels(line) && has_double_letter(line) && !has_specific_pair(line) {
                result += 1;
            }
        }
        println!("{}", result);
    }
    {
        let mut result = 0;
        for line in input.lines() {
            if has_sandwiched_letter(line) && has_repeated_pair(line) {
                result += 1;
            }
        }
        println!("{}", result);
    }
}

fn has_three_vowels(value: &str) -> bool {
    let mut vowel_count = 0;
    for ch in value.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => (),
        }
    }
    vowel_count >= 3
}

fn has_double_letter(value: &str) -> bool {
    let mut ch1: Option<char> = None;
    for ch2 in value.chars() {
        if ch1 == Some(ch2) {
            return true;
        }
        ch1 = Some(ch2);
    }
    false
}

fn has_specific_pair(value: &str) -> bool {
    value.contains("ab") || value.contains("cd") || value.contains("pq") || value.contains("xy")
}

fn has_sandwiched_letter(value: &str) -> bool {
    let mut ch1: Option<char> = None;
    let mut ch2: Option<char> = None;
    for ch3 in value.chars() {
        if ch1 == Some(ch3) {
            return true;
        }
        ch1 = ch2;
        ch2 = Some(ch3);
    }
    false
}

fn has_repeated_pair(value: &str) -> bool {
    let mut remainder = value;
    while remainder.len() >= 4 {
        let pair = &remainder[..2];
        if remainder[2..].contains(pair) {
            return true;
        }
        remainder = &remainder[1..];
    }
    false
}