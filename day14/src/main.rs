use std::cmp::{max, min};
use std::fs;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    move_duration: u32,
    rest_duration: u32,
}

fn main() {
    let input = fs::read_to_string("14.txt").unwrap();
    let mut reindeer: Vec<Reindeer> = Vec::new();
    for line in input.lines() {
        let mut components = line.split_whitespace();
        reindeer.push(Reindeer {
            speed: components.nth(3).unwrap().parse().unwrap(),
            move_duration: components.nth(2).unwrap().parse().unwrap(),
            rest_duration: components.nth(6).unwrap().parse().unwrap(),
        })
    }
    println!("{}", furthest_after(&reindeer, 2503));
    println!("{}", points_after(&reindeer, 2503));
}

fn furthest_after(reindeer: &[Reindeer], seconds: u32) -> u32 {
    assert!(!reindeer.is_empty());
    let mut result = 0;
    for reindeer in reindeer {
        result = max(result, position_after(reindeer, seconds));
    }
    result
}
fn points_after(reindeer: &[Reindeer], seconds: u32) -> u32 {
    assert!(!reindeer.is_empty());
    let mut points = vec![0u32; reindeer.len()];
    for second in 1..=seconds {
        let mut furthest_index = 0;
        let mut furthest_position = 0;
        for (idx, reindeer) in reindeer.iter().enumerate() {
            let position = position_after(reindeer, second);
            if position > furthest_position {
                furthest_index = idx;
                furthest_position = position;
            }
        }
        points[furthest_index] += 1;
    }
    *points.iter().max().unwrap()
}

fn position_after(reindeer: &Reindeer, seconds: u32) -> u32 {
    let cycle_length = reindeer.move_duration + reindeer.rest_duration;
    let cycle_count = seconds / cycle_length;
    let overflow_seconds = seconds % cycle_length;
    let partial_duration = min(overflow_seconds, reindeer.move_duration);
    reindeer.speed * (cycle_count * reindeer.move_duration + partial_duration)
}