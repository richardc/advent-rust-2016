use crate::assembunny::*;

#[aoc_generator(day12)]
fn generate(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
fn solve(program: &[Instruction]) -> i32 {
    let mut cpu = Cpu::new(program.to_vec());
    cpu.run();
    cpu.get('a')
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day12_example.txt"))), 42);
}

#[aoc(day12, part2)]
fn solve2(program: &[Instruction]) -> i32 {
    let mut cpu = Cpu::new(program.to_vec());
    cpu.set('c', 1);
    cpu.run();
    cpu.get('a')
}
