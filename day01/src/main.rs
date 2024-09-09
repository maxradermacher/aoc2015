use std::fs;

fn main() {
    let input = fs::read_to_string("01.txt").unwrap();
    {
        let mut floor = 0;
        for ch in input.chars() {
            match ch {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => (),
            }
        }
        println!("{}", floor);
    }
    {
        let mut floor = 0;
        for (pos, ch) in input.chars().enumerate() {
            match ch {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => (),
            }
            if floor == -1 {
                println!("{}", pos + 1);
                break;
            }
        }
    }
}
