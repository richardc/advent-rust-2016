use itertools::Itertools;
use thiserror::Error;

#[derive(PartialEq)]
struct Node {
    x: usize,
    y: usize,
    used: usize,
    avail: usize,
}

#[derive(Debug, Error)]
enum NodeParseError {
    #[error("Parse failed")]
    ParseInt(#[from] std::num::ParseIntError),
}

impl std::str::FromStr for Node {
    type Err = NodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Filesystem              Size  Used  Avail  Use%
        // /dev/grid/node-x0-y0     93T   67T    26T   72%
        let toks = s.split_ascii_whitespace().collect_vec();
        let path = toks[0].split('-').collect_vec();
        let x = path[1][1..].parse()?;
        let y = path[2][1..].parse()?;
        let used = toks[2][..toks[2].len() - 1].parse()?;
        let avail = toks[3][..toks[3].len() - 1].parse()?;
        Ok(Node { x, y, used, avail })
    }
}

#[aoc_generator(day22)]
fn generate(s: &str) -> Vec<Node> {
    s.lines().skip(2).map(|s| s.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
fn solve(nodes: &[Node]) -> usize {
    nodes
        .iter()
        .permutations(2)
        .filter(|v| v[0] != v[1] && v[0].used != 0 && v[1].avail >= v[0].used)
        .count()
}
