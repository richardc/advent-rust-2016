use crate::assembunny::*;

#[aoc_generator(day23)]
fn generate(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day23, part1)]
fn solve(program: &[Instruction]) -> i32 {
    let mut cpu = Cpu::new(program.to_vec());
    cpu.set('a', 7);
    cpu.run();
    cpu.get('a')
}

#[aoc(day23, part2)]
fn solve2(program: &[Instruction]) -> i32 {
    let mut cpu = Cpu::new(program.to_vec());
    cpu.set('a', 12);
    cpu.run();
    cpu.get('a')
}
