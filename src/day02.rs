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

#[aoc(day2, part2)]
fn solve2(input: &str) -> String {
    let buttons = array![
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ]
    .reversed_axes();
    let mut x: usize = 0;
    let mut y: usize = 2;
    let mut value = vec![];

    for cmd in input.lines() {
        for act in cmd.chars() {
            let mut nx = x;
            let mut ny = y;
            match act {
                'L' => nx = x.saturating_sub(1),
                'R' => nx = std::cmp::min(4, x + 1),
                'U' => ny = y.saturating_sub(1),
                'D' => ny = std::cmp::min(4, y + 1),
                _ => {}
            };
            if buttons[[nx, ny]] != ' ' {
                x = nx;
                y = ny;
            }
        }
        value.push(buttons[[x, y]]);
    }
    return String::from_iter(value);
}

#[test]
fn test_solve2() {
    assert_eq!(&solve2(include_str!("day02_example.txt")), "5DB3");
}
