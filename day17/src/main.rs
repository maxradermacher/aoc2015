use std::fs;

fn main() {
    let input = fs::read_to_string("17.txt").unwrap();
    let mut containers: Vec<i32> = Vec::new();
    for line in input.lines() {
        containers.push(line.parse().unwrap());
    }
    containers.sort();
    containers.reverse();
    println!("{}", count_possibilities(&containers, 150));
    println!("{}", count_most_efficient(&containers, 150));
}

fn count_possibilities(containers: &[i32], target: i32) -> u32 {
    let mut result = 0;
    let mut bitmask: u32 = 0;
    while bitmask < (1 << containers.len()) {
        let mut total = 0;
        for (idx, capacity) in containers.iter().enumerate() {
            let bit = 1 << (containers.len() - idx - 1);
            if bitmask & bit != 0 {
                total += capacity;
                if total >= target {
                    if total == target {
                        result += 1;
                    }
                    bitmask |= bit - 1;
                    break;
                }
            }
        }
        bitmask += 1;
    }
    result
}

fn count_most_efficient(containers: &[i32], target: i32) -> u32 {
    let mut result = 0;
    let mut minimum: u32 = containers.len().try_into().unwrap();
    let mut bitmask: u32 = 0;
    while bitmask < (1 << containers.len()) {
        let mut total = 0;
        for (idx, capacity) in containers.iter().enumerate() {
            let bit = 1 << (containers.len() - idx - 1);
            if bitmask & bit != 0 {
                total += capacity;
                if total >= target {
                    if total == target {
                        if bitmask.count_ones() < minimum {
                            minimum = bitmask.count_ones();
                            result = 0;
                        }
                        if bitmask.count_ones() == minimum {
                            result += 1;
                        }
                    }
                    bitmask |= bit - 1;
                    break;
                }
            }
        }
        bitmask += 1;
    }
    result
}
