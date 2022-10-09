use ndarray::prelude::*;

#[aoc(day2, part1)]
fn solve(input: &str) -> i32 {
    let buttons = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]].reversed_axes();
    let mut x: usize = 1;
    let mut y: usize = 1;
    let mut value = 0;

    for cmd in input.lines() {
        for act in cmd.chars() {
            match act {
                'L' => x = x.saturating_sub(1),
                'R' => x = std::cmp::min(2, x + 1),
                'U' => y = y.saturating_sub(1),
                'D' => y = std::cmp::min(2, y + 1),
                _ => {}
            };
        }

        value = value * 10 + buttons[[x, y]];
    }
    return value;
}

#[test]
fn test_solve() {
    assert_eq!(solve(include_str!("day02_example.txt")), 1985);
}
