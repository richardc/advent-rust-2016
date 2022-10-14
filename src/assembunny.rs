use itertools::Itertools;
use thiserror::Error;

type Register = char;

#[derive(Clone, Copy)]
pub enum Value {
    Value(i32),
    Register(char),
}

#[derive(Debug, Error)]
pub enum ValueParseError {}
impl std::str::FromStr for Value {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<i32>() {
            Ok(Value::Value(value))
        } else {
            Ok(Value::Register(s.chars().next().unwrap()))
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Cpy(Value, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Value, i32),
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
            "cpy" => Ok(Instruction::Cpy(
                toks[1].parse()?,
                toks[2].chars().next().unwrap(),
            )),
            "inc" => Ok(Instruction::Inc(toks[1].chars().next().unwrap())),
            "dec" => Ok(Instruction::Dec(toks[1].chars().next().unwrap())),
            "jnz" => Ok(Instruction::Jnz(toks[1].parse()?, toks[2].parse()?)),
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
}

impl Cpu {
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
            Value::Value(v) => v,
        }
    }

    fn apply(&mut self, inst: Instruction) {
        let mut next = 1;
        match inst {
            Instruction::Cpy(src, dest) => self.set(dest, self.eval(src)),
            Instruction::Inc(dest) => self.set(dest, self.get(dest) + 1),
            Instruction::Dec(dest) => self.set(dest, self.get(dest) - 1),
            Instruction::Jnz(test, jump) => {
                if self.eval(test) != 0 {
                    next = jump;
                }
            }
        }
        self.pc += next;
    }

    pub fn run(&mut self, program: &[Instruction]) {
        while self.pc < program.len() as i32 {
            self.apply(program[self.pc as usize]);
        }
    }
}
