use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("19.txt").unwrap();
    let mut lines = input.lines();
    let molecule = lines.next_back().unwrap();
    assert!(lines.next_back().unwrap().is_empty());
    let mut replacements: HashMap<&[u8], Vec<&str>> = HashMap::new();
    for line in lines {
        let mut components = line.split_ascii_whitespace();
        replacements.entry(components.next().unwrap().as_bytes()).or_default().push(components.next_back().unwrap());
    }
    let mut index = 0;
    while index < molecule.bytes().len() {
        for length in 1..=2.min(molecule.bytes().len() - index) {
            println!("{:?}", replacements.get(&molecule[index..(index + length)]));
        }
    }
    // let mut candidate: Vec<u8> = Vec::new();
    // for (index, character) in molecule.bytes().enumerate() {
    //     candidate.push(character);
    //     if let Some(replacements) = replacements.get(candidate.as_slice()) {
    //         candidate.clear();
    //         continue;
    //     }
    //     if candidate.len() == 2 {
    //         candidate.remove(0);
    //     }
    //     match replacements.get(candidate.as_slice()) {
    //     }
    //     candidate.push(character);
    //     println!("{:?}", replacements.get(candidate.as_slice()));
    // }
}
