use itertools::Itertools;

use crate::assembunny::*;

#[aoc_generator(day25)]
fn generate(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day25, part1)]
fn solve(program: &[Instruction]) -> i32 {
    (0..)
        .into_iter()
        .find(|&int| {
            let mut cpu = Cpu::new(program.to_vec());
            cpu.set('a', int);
            cpu.iter().take(1000).tuple_windows().all(|(a, b)| a != b)
        })
        .unwrap()
}
