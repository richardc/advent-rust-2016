use itertools::iproduct;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

fn is_wall(seed: i32, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        true
    } else {
        (x * x + 3 * x + 2 * x * y + y + y * y + seed).count_ones() % 2 == 1
    }
}

#[cfg(test)]
#[test]
fn test_is_wall() {
    assert!(is_wall(10, -1, 0), "Outside is walls");
    assert!(!is_wall(10, 0, 0));
    assert!(is_wall(10, 1, 0));
}

fn successors(seed: i32, x: i32, y: i32) -> Vec<((i32, i32), usize)> {
    [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .iter()
        .map(|(xp, yp)| (x + xp, y + yp))
        .filter(|&(x, y)| !is_wall(seed, x, y))
        .map(|p| (p, 1))
        .collect_vec()
}

fn find_route(seed: i32, dx: i32, dy: i32) -> usize {
    let goal = (dx, dy);
    let (_path, cost) =
        dijkstra(&(1, 1), |&(x, y)| successors(seed, x, y), |&p| p == goal).unwrap();
    cost
}

#[cfg(test)]
#[test]
fn test_find_route() {
    assert_eq!(find_route(10, 7, 4), 11);
}

#[aoc(day13, part1)]
fn solve(input: &str) -> usize {
    find_route(input.parse().unwrap(), 31, 39)
}

#[aoc(day13, part2)]
fn solve2(input: &str) -> usize {
    let seed = input.parse().unwrap();
    iproduct!(0..=50, 0..=50)
        .filter(|&(x, y)| !is_wall(seed, x, y))
        .flat_map(|goal| {
            match dijkstra(&(1, 1), |&(x, y)| successors(seed, x, y), |&p| p == goal) {
                Some((nodes, count)) if count <= 50 => nodes,
                _ => vec![],
            }
        })
        .unique()
        .count()
}
