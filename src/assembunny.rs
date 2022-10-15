use itertools::Itertools;
use thiserror::Error;

type Register = char;

#[derive(Clone, Copy)]
pub enum Value {
    Literal(i32),
    Register(char),
}

#[derive(Debug, Error)]
pub enum ValueParseError {}
impl std::str::FromStr for Value {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<i32>() {
            Ok(Value::Literal(value))
        } else {
            Ok(Value::Register(s.chars().next().unwrap()))
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
    Out(Value),
}

#[derive(Error, Debug)]
pub enum InstructionParseError {
    #[error("unknown instruction")]
    Unknown(String),

    #[error("Number Parse")]
    Number(#[from] std::num::ParseIntError),

    #[error("Value Parse")]
    Value(#[from] ValueParseError),
}

impl std::str::FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks = s.split_ascii_whitespace().collect_vec();
        match toks[0] {
            "cpy" => Ok(Instruction::Cpy(toks[1].parse()?, toks[2].parse()?)),
            "inc" => Ok(Instruction::Inc(toks[1].parse()?)),
            "dec" => Ok(Instruction::Dec(toks[1].parse()?)),
            "jnz" => Ok(Instruction::Jnz(toks[1].parse()?, toks[2].parse()?)),
            "tgl" => Ok(Instruction::Tgl(toks[1].parse()?)),
            "out" => Ok(Instruction::Out(toks[1].parse()?)),
            _ => Err(InstructionParseError::Unknown(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Cpu {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: i32,
    program: Vec<Instruction>,
    output: Option<i32>,
}

impl Cpu {
    pub fn new(program: Vec<Instruction>) -> Self {
        Cpu {
            program,
            ..Default::default()
        }
    }
    pub fn get(&self, src: Register) -> i32 {
        match src {
            'a' => self.a,
            'b' => self.b,
            'c' => self.c,
            'd' => self.d,
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, dest: Register, value: i32) {
        match dest {
            'a' => self.a = value,
            'b' => self.b = value,
            'c' => self.c = value,
            'd' => self.d = value,
            _ => unreachable!(),
        }
    }

    fn eval(&self, value: Value) -> i32 {
        match value {
            Value::Register(c) => self.get(c),
            Value::Literal(v) => v,
        }
    }

    fn tick(&mut self) {
        let mut next = 1;
        match self.program[self.pc as usize] {
            Instruction::Cpy(src, Value::Register(dest)) => self.set(dest, self.eval(src)),
            Instruction::Cpy(_, Value::Literal(_)) => (),
            Instruction::Inc(Value::Register(dest)) => self.set(dest, self.get(dest) + 1),
            Instruction::Inc(Value::Literal(_)) => (),
            Instruction::Dec(Value::Register(dest)) => self.set(dest, self.get(dest) - 1),
            Instruction::Dec(Value::Literal(_)) => (),
            Instruction::Jnz(test, jump) => {
                if self.eval(test) != 0 {
                    next = self.eval(jump);
                }
            }
            Instruction::Tgl(Value::Register(dest)) => {
                let addr = self.pc + self.get(dest);
                if addr >= 0 && addr < self.program.len() as i32 {
                    self.program[addr as usize] = match self.program[addr as usize] {
                        Instruction::Inc(a) => Instruction::Dec(a),
                        Instruction::Dec(a) => Instruction::Inc(a),
                        Instruction::Tgl(a) => Instruction::Inc(a),
                        Instruction::Out(a) => Instruction::Inc(a),
                        Instruction::Jnz(a, b) => Instruction::Cpy(a, b),
                        Instruction::Cpy(a, b) => Instruction::Jnz(a, b),
                    };
                }
            }
            Instruction::Tgl(Value::Literal(_)) => (),
            Instruction::Out(src) => self.output = Some(self.eval(src)),
        }
        self.pc += next;
    }

    pub fn halted(&self) -> bool {
        self.pc >= self.program.len() as i32
    }

    pub fn run(&mut self) {
        while !self.halted() {
            self.tick();
        }
    }

    pub fn run_till_output(&mut self) -> Option<i32> {
        self.output = None;
        while !self.halted() && self.output == None {
            self.tick();
        }
        self.output
    }

    pub fn iter(self) -> CpuIter {
        CpuIter { cpu: self }
    }
}

pub struct CpuIter {
    cpu: Cpu,
}

impl Iterator for CpuIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpu.run_till_output()
    }
}
