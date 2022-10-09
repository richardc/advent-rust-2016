use std::collections::HashSet;

use ndarray::prelude::*;

#[aoc(day1, part1)]
fn solve(input: &str) -> i32 {
    let mut position = array![0, 0];
    let vectors = [
        array![0 as i32, 1],
        array![1, 0],
        array![0, -1],
        array![-1, 0],
    ];
    let mut facing = 0;

    for cmd in input.split(", ") {
        facing = match cmd.chars().next().unwrap() {
            'L' => (vectors.len() + facing - 1) % vectors.len(),
            'R' => (facing + 1) % vectors.len(),
            _ => 0,
        };
        let distance = cmd[1..].parse::<i32>().unwrap();
        let delta = &vectors[facing] * distance;
        position += &delta;
    }
    return position[0].abs() + position[1].abs();
}

#[test]
fn test_solve() {
    assert_eq!(solve("R2, L3"), 5);
    assert_eq!(solve("R2, R2, R2"), 2);
    assert_eq!(solve("R5, L5, R5, R3"), 12);
}

#[aoc(day1, part2)]
fn solve2(input: &str) -> i32 {
    let mut position = array![0, 0];
    let vectors = [
        array![0 as i32, 1],
        array![1, 0],
        array![0, -1],
        array![-1, 0],
    ];
    let mut facing = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for cmd in input.split(", ") {
        facing = match cmd.chars().next().unwrap() {
            'L' => (vectors.len() + facing - 1) % vectors.len(),
            'R' => (facing + 1) % vectors.len(),
            _ => 0,
        };
        let distance = cmd[1..].parse::<i32>().unwrap();
        for _ in 0..distance {
            // Each step is is a grid corner we visited
            position += &vectors[facing];
            let here = (position[0], position[1]);
            if let Some(_) = visited.get(&here) {
                return position[0].abs() + position[1].abs();
            }
            visited.insert(here);
        }
    }
    return position[0].abs() + position[1].abs();
}

#[test]
fn test_solve2() {
    assert_eq!(solve2("R8, R4, R4, R8"), 4)
}
