use std::collections::HashMap;

use itertools::Itertools;

fn solver(input: &str, sort: fn(&usize, &usize) -> std::cmp::Ordering) -> String {
    let lines = input.lines();
    let mut counts: Vec<HashMap<u8, usize>> = vec![];
    for l in lines {
        for (i, c) in l.char_indices() {
            if counts.len() < i + 1 {
                counts.push(HashMap::new());
            }
            counts[i]
                .entry(c as u8)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    String::from_iter(counts.into_iter().map(|c| {
        c.iter()
            .sorted_by(|(_, av), (_, bv)| sort(av, bv))
            .map(|(&k, _)| k as char)
            .take(1)
            .next()
            .unwrap()
    }))
}

#[aoc(day6, part1)]
fn solve(input: &str) -> String {
    solver(input, |av, bv| Ord::cmp(bv, av))
}

#[test]
fn test_solve() {
    assert_eq!(solve(include_str!("day06_example.txt")), "easter");
}

#[aoc(day6, part2)]
fn solve2(input: &str) -> String {
    solver(input, |av, bv| Ord::cmp(av, bv))
}

#[test]
fn test_solve2() {
    assert_eq!(solve2(include_str!("day06_example.txt")), "advent");
}
