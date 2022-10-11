use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy)]
enum Destination {
    #[default]
    None,
    Bot(usize),
    Output(usize),
}

impl std::str::FromStr for Destination {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, number) = s.split_once(' ').unwrap();
        match kind {
            "bot" => Ok(Destination::Bot(number.parse()?)),
            "output" => Ok(Destination::Output(number.parse()?)),
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Bot {
    holds: Vec<usize>,
    held: HashSet<usize>,
    low: Destination,
    high: Destination,
}

impl Bot {
    fn new() -> Self {
        Self::default()
    }

    fn take(&mut self, value: usize) {
        self.holds.push(value);
        self.holds.sort();
        self.held.insert(value);
    }

    fn gives(&mut self, low: Destination, high: Destination) {
        self.low = low;
        self.high = high;
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        self.held.contains(&x) && self.held.contains(&y)
    }
}

#[derive(Default, Debug, Clone)]
struct Machine {
    outputs: HashMap<usize, usize>,
    bots: HashMap<usize, Bot>,
}

impl Machine {
    fn tick(&mut self) {
        let distribute = self
            .bots
            .iter_mut()
            .filter(|(_, b)| b.holds.len() >= 2)
            .flat_map(|(_, b)| {
                let dist = vec![(b.low, b.holds[0]), (b.high, b.holds[1])];
                b.holds.clear();
                dist
            })
            .collect_vec();

        for (dest, value) in distribute {
            match dest {
                Destination::Bot(other) => {
                    self.bots.entry(other).or_insert_with(Bot::new).take(value)
                }
                Destination::Output(out) => {
                    self.outputs.insert(out, value);
                }
                _ => {}
            }
        }
    }

    fn who_compares(&self, x: usize, y: usize) -> usize {
        let mut m = self.clone();
        loop {
            if let Some((&k, _)) = m.bots.iter().find(|(_, b)| b.contains(x, y)) {
                return k;
            }
            m.tick();
        }
    }

    fn products(&self) -> usize {
        let mut m = self.clone();
        loop {
            m.tick();
            if (0..=2).all(|i| m.outputs.contains_key(&i)) {
                return (0..=2).map(|i| m.outputs.get(&i).unwrap()).product();
            }
        }
    }
}

#[aoc_generator(day10)]
fn generate(input: &str) -> Machine {
    let mut m = Machine::default();
    for l in input.lines() {
        let toks = l.split_ascii_whitespace().collect_vec();
        match toks[0] {
            "value" => {
                let v = toks[1].parse().unwrap();
                let bot = toks[5].parse().unwrap();
                m.bots
                    .entry(bot)
                    .and_modify(|b| b.take(v))
                    .or_insert_with(|| {
                        let mut b = Bot::new();
                        b.take(v);
                        b
                    });
            }
            "bot" => {
                let source = toks[1].parse().unwrap();
                let low = toks[5..=6].join(" ").parse().unwrap();
                let high = toks[10..=11].join(" ").parse().unwrap();
                m.bots
                    .entry(source)
                    .and_modify(|b| b.gives(low, high))
                    .or_insert_with(|| {
                        let mut b = Bot::new();
                        b.gives(low, high);
                        b
                    });
            }
            _ => {}
        }
    }
    m
}

#[aoc(day10, part1)]
fn solve(machine: &Machine) -> usize {
    machine.who_compares(61, 17)
}

#[test]
fn test_example() {
    assert_eq!(
        generate(include_str!("day10_example.txt")).who_compares(2, 5),
        2
    );
}

#[aoc(day10, part2)]
fn solve2(machine: &Machine) -> usize {
    machine.products()
}
