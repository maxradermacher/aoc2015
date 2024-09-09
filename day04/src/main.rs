use std::fs;
use md5::{Md5, Digest};

fn main() {
    let input = fs::read_to_string("04.txt").unwrap();
    let input = input.trim_end();
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    {
        let mut number = 0;
        loop {
            let mut hasher = hasher.clone();
            hasher.update(number.to_string().as_bytes());
            let result = hasher.finalize();
            if (result[0] == 0) && (result[1] == 0) && (result[2] & 0xF0 == 0) {
                println!("{}", number);
                break;
            }
            number += 1;
        }
    }
    {
        let mut number = 0;
        loop {
            let mut hasher = hasher.clone();
            hasher.update(number.to_string().as_bytes());
            let result = hasher.finalize();
            if (result[0] == 0) && (result[1] == 0) && (result[2] == 0) {
                println!("{}", number);
                break;
            }
            number += 1;
        }
    }
}
