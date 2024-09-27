use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Path<'a> {
    total_distance: isize,
    already_visited: Vec<&'a str>,
}

fn main() {
    let input = fs::read_to_string("09.txt").unwrap();

    let mut origins: HashMap<&str, HashMap<&str, isize>> = HashMap::new();
    for line in input.lines() {
        let mut components = line.split_whitespace();
        let place1 = components.next().unwrap();
        components.next().unwrap();
        let place2 = components.next().unwrap();
        components.next().unwrap();
        let distance: isize = components.next().unwrap().parse().unwrap();
        origins.entry(place1).or_insert(HashMap::new()).insert(place2, distance);
        origins.entry(place2).or_insert(HashMap::new()).insert(place1, distance);
    }

    println!("{}", find_best_path(&origins, -1));
    println!("{}", find_best_path(&origins, 1));
}

fn find_best_path(origins: &HashMap<&str, HashMap<&str, isize>>, multiplier: isize) -> usize {
    let mut best_paths: BinaryHeap<Path> = BinaryHeap::new();
    for &origin in origins.keys() {
        best_paths.push(Path {
            total_distance: 0,
            already_visited: vec![origin],
        });
    }

    loop {
        let mut next_best_paths: BinaryHeap<Path> = BinaryHeap::new();
        while let Some(path) = best_paths.pop() {
            if path.already_visited.len() == origins.len() {
                return (path.total_distance * multiplier).try_into().unwrap();
            }
            for (destination, distance) in &origins[path.already_visited.last().unwrap()] {
                if path.already_visited.contains(destination) {
                    continue;
                }
                let mut new_already_visited = path.already_visited.to_vec();
                new_already_visited.push(destination);
                next_best_paths.push(Path {
                    total_distance: path.total_distance + distance * multiplier,
                    already_visited: new_already_visited,
                })
            }
        }
        best_paths = next_best_paths;
    }
}
