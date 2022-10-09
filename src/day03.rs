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
