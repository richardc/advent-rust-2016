use std::collections::HashMap;

use itertools::Itertools;
use ndarray::prelude::*;
use pathfinding::prelude::dijkstra;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

#[derive(Debug)]
struct Hvac {
    map: Array2<u8>,
}

impl Hvac {
    fn new(input: &str) -> Self {
        let rows = input.lines().collect_vec().len();
        let cols = input.lines().next().unwrap().len();
        let data = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c as u8)
            .collect();

        let map = Array::from_shape_vec((rows, cols), data)
            .unwrap()
            .reversed_axes();

        Hvac { map }
    }

    fn successors(&self, from: &Point) -> Vec<Point> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(xp, yp)| Point(from.0 + xp, from.1 + yp))
            .filter(|p| self.map[[p.0 as usize, p.1 as usize]] != b'#')
            .collect_vec()
    }

    fn nodes(&self) -> Vec<Point> {
        self.map
            .indexed_iter()
            .filter_map(|((x, y), c)| {
                if c.is_ascii_digit() {
                    Some((c, Point(x as i32, y as i32)))
                } else {
                    None
                }
            })
            .sorted_by(|a, b| Ord::cmp(a.0, b.0))
            .map(|t| t.1)
            .collect()
    }
}

#[aoc_generator(day24)]
fn generate(s: &str) -> Hvac {
    Hvac::new(s)
}

fn solver(hvac: &Hvac, back_home: bool) -> usize {
    let nodes = hvac.nodes();
    let routes: HashMap<(Point, Point), usize> = HashMap::from_iter(
        nodes
            .iter()
            .permutations(2)
            .map(|v| (v[0], v[1]))
            .filter(|(start, end)| start != end)
            .filter_map(|(start, end)| {
                dijkstra(
                    start,
                    |p| hvac.successors(p).into_iter().map(|s| (s, 1)).collect_vec(),
                    |p| *p == *end,
                )
            })
            .flat_map(|(path, cost)| {
                [
                    ((*path.first().unwrap(), *path.last().unwrap()), cost),
                    ((*path.last().unwrap(), *path.first().unwrap()), cost),
                ]
            }),
    );

    let start = nodes[0];
    let rest = &nodes[1..];
    rest.iter()
        .permutations(rest.len())
        .map(|walk| {
            // all paths start at start.  Part 2 they go back home
            let path = match back_home {
                false => [&[&start], &walk[..]].concat(),
                true => [&[&start], &walk[..], &[&start]].concat(),
            };

            path.into_iter()
                .tuple_windows()
                .map(|(start, end)| *routes.get(&(*start, *end)).unwrap())
                .sum()
        })
        .min()
        .unwrap()
}

#[aoc(day24, part1)]
fn solve(hvac: &Hvac) -> usize {
    solver(hvac, false)
}

#[aoc(day24, part2)]
fn solve2(hvac: &Hvac) -> usize {
    solver(hvac, true)
}

#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day24_example.txt"))), 14);
}
