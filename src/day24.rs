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

#[aoc(day24, part1)]
fn solve(hvac: &Hvac) -> usize {
    let routes: HashMap<(Point, Point), usize> = HashMap::from_iter(
        hvac.nodes()
            .into_iter()
            .permutations(2)
            .filter_map(|v| {
                let start = v[0];
                let end = v[1];
                if start == end {
                    None
                } else {
                    dijkstra(
                        &start,
                        |p| hvac.successors(p).into_iter().map(|s| (s, 1)).collect_vec(),
                        |p| *p == end,
                    )
                }
            })
            .flat_map(|(path, cost)| {
                vec![
                    ((*path.first().unwrap(), *path.last().unwrap()), cost),
                    ((*path.last().unwrap(), *path.first().unwrap()), cost),
                ]
            }),
    );

    let nodes = hvac.nodes();
    let start = nodes[0];
    let mut cheapest = usize::MAX;
    for walk in nodes[1..].into_iter().permutations(nodes.len() - 1) {
        let cost = std::iter::once(&start)
            .chain(walk)
            .tuple_windows()
            .map(|(start, end)| *routes.get(&(*start, *end)).unwrap())
            .sum();
        cheapest = std::cmp::min(cheapest, cost);
    }

    cheapest
}

#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day24_example.txt"))), 14);
}
