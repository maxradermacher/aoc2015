use std::fs;

fn main() {
    let input = fs::read_to_string("16.txt").unwrap();
    println!("{}", find_sue(&input, |name, count| match name {
        "children" => count == 3,
        "cats" => count == 7,
        "samoyeds" => count == 2,
        "pomeranians" => count == 3,
        "akitas" => count == 0,
        "vizslas" => count == 0,
        "goldfish" => count == 5,
        "trees" => count == 3,
        "cars" => count == 2,
        "perfumes" => count == 1,
        _ => panic!("don't know how to handle {}", name),
    }).unwrap());
    println!("{}", find_sue(&input, |name, count| match name {
        "children" => count == 3,
        "cats" => count > 7,
        "samoyeds" => count == 2,
        "pomeranians" => count < 3,
        "akitas" => count == 0,
        "vizslas" => count == 0,
        "goldfish" => count < 5,
        "trees" => count > 3,
        "cars" => count == 2,
        "perfumes" => count == 1,
        _ => panic!("don't know how to handle {}", name),
    }).unwrap());
}

fn find_sue<F>(input: &str, expr: F) -> Option<u32> where F: Fn(&str, u32) -> bool {
    'outer: for line in input.lines() {
        let mut it = line.split_whitespace();
        let sue_number = it.nth(1).unwrap().trim_end_matches(|c| c == ':');
        while let Some(item_name) = it.next() {
            let item_name = item_name.trim_end_matches(|c| c == ':');
            let item_count: u32 = it.next().unwrap().trim_end_matches(|c| c == ',').parse().unwrap();
            if !expr(item_name, item_count) {
                continue 'outer;
            }
        }
        return Some(sue_number.parse().unwrap());
    }
    None
}
