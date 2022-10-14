use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum Instr {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateByLetter(char),
    RotateRight(usize),
    RotateLeft(usize),
    Reverse(usize, usize),
    Move(usize, usize),
}

#[derive(Error, Debug)]
enum InstrParseError {
    #[error("Unrecognised instruction `{0}`")]
    Unrecognised(String),

    #[error("Number Parse")]
    NumberParse(#[from] ParseIntError),
}

impl FromStr for Instr {
    type Err = InstrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks = s.split_ascii_whitespace().collect_vec();
        match (toks[0], toks[1]) {
            ("swap", "position") => Ok(Instr::SwapPosition(toks[2].parse()?, toks[5].parse()?)),
            ("swap", "letter") => Ok(Instr::SwapLetter(
                toks[2].chars().next().unwrap(),
                toks[5].chars().next().unwrap(),
            )),
            ("rotate", "based") => Ok(Instr::RotateByLetter(toks[6].chars().next().unwrap())),
            ("rotate", "left") => Ok(Instr::RotateLeft(toks[2].parse()?)),
            ("rotate", "right") => Ok(Instr::RotateRight(toks[2].parse()?)),
            ("reverse", _) => Ok(Instr::Reverse(toks[2].parse()?, toks[4].parse()?)),
            ("move", _) => Ok(Instr::Move(toks[2].parse()?, toks[5].parse()?)),
            _ => Err(InstrParseError::Unrecognised(s.to_string())),
        }
    }
}

struct Scrambler {
    program: Vec<Instr>,
}

impl Scrambler {
    fn new(from: &str) -> Self {
        let program = from.lines().map(|l| l.parse().unwrap()).collect();
        Self { program }
    }

    fn scramble(&self, s: &str) -> String {
        let mut bytes = s.as_bytes().to_vec();
        self.program.iter().for_each(|&instr| match instr {
            Instr::SwapPosition(x, y) => (bytes[x], bytes[y]) = (bytes[y], bytes[x]),
            Instr::SwapLetter(x, y) => bytes.iter_mut().for_each(|c| {
                if *c == x as u8 {
                    *c = y as u8
                } else if *c == y as u8 {
                    *c = x as u8
                }
            }),
            Instr::RotateLeft(distance) => bytes.rotate_left(distance as usize),
            Instr::RotateRight(distance) => bytes.rotate_right(distance as usize),
            Instr::RotateByLetter(c) => {
                if let Some(index) = bytes.iter().position(|b| *b == c as u8) {
                    let distance = if index >= 4 { index + 2 } else { index + 1 };
                    let distance = distance % bytes.len();
                    bytes.rotate_right(distance)
                }
            }
            Instr::Reverse(start, end) => bytes[start..=end].reverse(),
            Instr::Move(x, y) => {
                let c = bytes.remove(x);
                bytes.insert(y, c);
            }
        });
        String::from_utf8(bytes).unwrap()
    }
}

#[test]
fn test_scrambler() {
    assert_eq!(
        generate(include_str!("day21_example.txt")).scramble("abcde"),
        "decab"
    );
}

#[aoc_generator(day21)]
fn generate(input: &str) -> Scrambler {
    Scrambler::new(input)
}

#[aoc(day21, part1)]
fn solve(scrambler: &Scrambler) -> String {
    scrambler.scramble("abcdefgh")
}
