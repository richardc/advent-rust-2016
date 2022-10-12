use std::collections::HashSet;

use itertools::Itertools;
use lazy_static::lazy_static;
use pathfinding::prelude::bfs;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
enum Item {
    Generator(String),
    Chip(String),
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Floor(HashSet<Item>);

impl std::hash::Hash for Floor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.iter().sorted().for_each(|i| i.hash(state));
    }
}

impl Floor {
    fn safe(&self) -> bool {
        // Unsafe if: any generator is unpaired and there's an unpaired chip to hurt
        let unmatched_generators = self.0.iter().any(|i| match i {
            Item::Generator(kind) => !self.0.contains(&Item::Chip(kind.clone())),
            _ => false,
        });
        let unmatched_chips = self.0.iter().any(|i| match i {
            Item::Chip(kind) => !self.0.contains(&Item::Generator(kind.clone())),
            _ => false,
        });
        !(unmatched_generators && unmatched_chips)
    }

    fn add(&mut self, item: Item) {
        self.0.insert(item);
    }

    fn remove(&mut self, item: &Item) {
        self.0.remove(item);
    }
}

#[cfg(test)]
#[test]
fn test_floor_safe() {
    let mut floor = Floor::default();
    assert!(floor.safe(), "Empty floors are safe");
    floor.add(Item::Chip("Red".to_string()));
    assert!(floor.safe(), "Chips on their own are safe");
    floor.add(Item::Generator("Green".to_string()));
    assert!(!floor.safe(), "Unmatched generator is unsafe");
    floor.add(Item::Chip("Green".to_string()));
    assert!(floor.safe(), "Matched chip to make it safe");
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
struct Factory {
    lift: usize,
    floors: [Floor; 4],
}

#[derive(Debug)]
enum FactoryParseError {}

impl std::str::FromStr for Factory {
    type Err = FactoryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut factory = Factory::default();
        for (floor, contents) in s.lines().enumerate() {
            lazy_static! {
                static ref CHIP: Regex = Regex::new(r"a (\S+)-compatible microchip").unwrap();
            }
            lazy_static! {
                static ref GEN: Regex = Regex::new(r"a (\S+) generator").unwrap();
            }
            CHIP.captures_iter(contents).for_each(|c| {
                factory.floors[floor].add(Item::Chip(c.get(1).unwrap().as_str().to_string()))
            });
            GEN.captures_iter(contents).for_each(|c| {
                factory.floors[floor].add(Item::Generator(c.get(1).unwrap().as_str().to_string()))
            });
        }
        Ok(factory)
    }
}

#[cfg(test)]
#[test]
fn test_factory_from_str() {
    let parsed: Factory = include_str!("day11_example.txt").parse().unwrap();
    let mut factory = Factory::default();
    factory.floors[0].add(Item::Chip("hydrogen".to_string()));
    factory.floors[0].add(Item::Chip("lithium".to_string()));
    factory.floors[1].add(Item::Generator("hydrogen".to_string()));
    factory.floors[2].add(Item::Generator("lithium".to_string()));
    assert_eq!(parsed, factory);
}

impl Factory {
    fn legal(&self) -> bool {
        self.floors.iter().all(|f| f.safe())
    }

    fn moves_to(&self, floor: usize) -> Vec<Self> {
        if self.lift == floor || (self.lift as i32 - floor as i32).abs() > 1 {
            return vec![];
        }
        self.floors[self.lift]
            .0
            .iter()
            .powerset()
            .skip(1)
            .take_while(|v| v.len() <= 2)
            .filter(|v| {
                // Contents of the lift must be safe
                let mut lift = Floor::default();
                v.iter().for_each(|&item| lift.add(item.clone()));
                lift.safe()
            })
            .map(|set| {
                let mut factory = self.clone();
                set.iter()
                    .for_each(|&item| factory.floors[self.lift].remove(item));
                set.iter()
                    .for_each(|&item| factory.floors[floor].add(item.clone()));
                factory.lift = floor;
                factory
            })
            .filter(|factory| factory.legal())
            .collect()
    }

    fn moves(&self) -> Vec<Self> {
        // The lift can take up to 2 items, one floor up or down
        self.floors
            .iter()
            .enumerate()
            .flat_map(|(floor, _)| self.moves_to(floor))
            .collect()
    }

    fn solved(&self) -> bool {
        (0..self.floors.len() - 1).all(|floor| self.floors[floor].0.is_empty())
    }
}

#[cfg(test)]
#[test]
fn test_factory_moves() {
    let factory: Factory = include_str!("day11_example.txt").parse().unwrap();
    let moves = factory.moves();
    assert_eq!(moves.len(), 2, "There are two legal moves");
    assert!(
        moves.iter().all(|factory| factory.lift == 1),
        "All moves go to floor 1"
    );
}

#[cfg(test)]
#[test]
fn test_factory_solved() {
    assert!(Factory::default().solved(), "Empty factories are solved");

    let factory: Factory = include_str!("day11_example.txt").parse().unwrap();
    assert!(!factory.solved(), "Example factory does not start solved");

    let mut factory = Factory::default();
    factory.floors[3].add(Item::Chip("Green".to_string()));
    assert!(factory.solved(), "Solved when everything on the top floor");
}

#[aoc(day11, part1)]
fn solve(input: &str) -> usize {
    let factory: Factory = input.parse().unwrap();
    if let Some(path) = bfs(&factory, |f| f.moves(), |f| f.solved()) {
        path.len() + 1
    } else {
        0
    }
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(include_str!("day11_example.txt")), 11);
}
