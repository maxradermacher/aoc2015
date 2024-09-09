use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("03.txt").unwrap();
    {
        let mut unique_positions = HashSet::new();
        let mut current_position = Position { x: 0, y: 0 };
        unique_positions.insert(current_position);
        for op in input.chars() {
            match op {
                '>' => current_position.x += 1,
                '<' => current_position.x -= 1,
                '^' => current_position.y += 1,
                'v' => current_position.y -= 1,
                _ => (),
            }
            unique_positions.insert(current_position);
        }
        println!("{}", unique_positions.len());
    }
    {
        let mut unique_positions = HashSet::new();
        let mut santa_position = Position { x: 0, y: 0 };
        let mut robot_position = Position { x: 0, y: 0 };
        unique_positions.insert(santa_position);
        for (idx, op) in input.chars().enumerate() {
            let current_position = if (idx % 2) == 0 {
                &mut santa_position
            } else {
                &mut robot_position
            };
            match op {
                '>' => current_position.x += 1,
                '<' => current_position.x -= 1,
                '^' => current_position.y += 1,
                'v' => current_position.y -= 1,
                _ => (),
            }
            unique_positions.insert(*current_position);
        }
        println!("{}", unique_positions.len());
    }
}
