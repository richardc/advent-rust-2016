use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Disc {
    start: usize,
    period: usize,
}

#[derive(Debug)]
struct DiscParseErr {}
impl FromStr for Disc {
    type Err = DiscParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"has (\d+) positions; at time=0, it is at position (\d+)\.").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        let period = captures.get(1).unwrap().as_str().parse().unwrap();
        let start = captures.get(2).unwrap().as_str().parse().unwrap();
        Ok(Disc { start, period })
    }
}

#[cfg(test)]
#[test]
fn test_disc_from_str() {
    let disc: Disc = "Disc #1 has 5 positions; at time=0, it is at position 4."
        .parse()
        .unwrap();

    assert_eq!(
        disc,
        Disc {
            start: 4,
            period: 5
        }
    )
}

#[aoc_generator(day15)]
fn generate(s: &str) -> Vec<Disc> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
fn solve(discs: &[Disc]) -> usize {
    'time: for time in 0.. {
        for (i, disc) in discs.iter().enumerate() {
            if (i + 1 + disc.start + time) % disc.period != 0 {
                continue 'time;
            }
        }
        return time;
    }
    unreachable!()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day15_example.txt"))), 5)
}

#[aoc(day15, part2)]
fn solve2(discs: &[Disc]) -> usize {
    solve(
        &[
            discs,
            &[Disc {
                period: 11,
                start: 0,
            }],
        ]
        .concat(),
    )
}
