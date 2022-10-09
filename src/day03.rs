use itertools::Itertools;

#[aoc(day3, part1)]
fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .sorted()
                .collect_vec()
        })
        .filter(|t| t[0] + t[1] > t[2])
        .count()
}

#[test]
fn test_solve() {
    assert_eq!(solve("5 10 25"), 0);
}

#[aoc(day3, part2)]
fn solve2(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|c| {
            let square = c
                .map(|l| {
                    l.split_ascii_whitespace()
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect_vec()
                })
                .collect_vec();
            vec![
                vec![square[0][0], square[1][0], square[2][0]],
                vec![square[0][1], square[1][1], square[2][1]],
                vec![square[0][2], square[1][2], square[2][2]],
            ]
        })
        .map(|mut t| {
            t.sort();
            t
        })
        .filter(|t| t[0] + t[1] > t[2])
        .count()
}

#[test]
fn test_solve2() {
    assert_eq!(solve2(include_str!("day03_example.txt")), 6);
}
