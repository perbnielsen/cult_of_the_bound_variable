use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

pub struct UniversalMachine {
    instruction_pointer: usize,
    registers: [i32; 8],
    memory: HashMap<usize, Vec<i32>>,
    next_mem: usize,
}

impl UniversalMachine {
    pub fn run(
        Self {
            mut instruction_pointer,
            mut registers,
            mut memory,
            mut next_mem,
        }: Self,
    ) {
        loop {
            let operator = Operator::from(memory[&0][instruction_pointer]);
            instruction_pointer = instruction_pointer + 1;
            match operator {
                Operator::CondMove(a, b, c) => {
                    if c != 0 {
                        registers[a] = registers[b]
                    }
                }
                Operator::Read(a, b, c) => registers[a] = memory[&(b)][registers[c] as usize],
                Operator::Write(a, b, c) => {
                    memory.get_mut(&(a)).unwrap()[registers[b] as usize] = registers[c]
                }
                Operator::Add(a, b, c) => registers[a] = registers[b] + registers[c],
                Operator::Mul(a, b, c) => registers[a] = registers[b] * registers[c],
                Operator::Div(a, b, c) => registers[a] = registers[b] / registers[c],
                Operator::NotAnd(a, b, c) => registers[a] = !registers[b] & !registers[c],
                Operator::Halt => break,
                Operator::Alloc(b, c) => {
                    memory.insert(next_mem, vec![0, registers[c]]);
                    registers[b] = next_mem as i32;
                    next_mem += 1;
                }
                Operator::Dealloc(c) => {
                    memory.remove(&(c));
                }
                Operator::Out(c) => println!("{}", c),
                Operator::In(c) => todo!(),
                Operator::Load(b, c) => {
                    instruction_pointer = c;
                }
                Operator::Immediate(a, value) => registers[a] = value,
            }
        }
    }
}

enum Operator {
    CondMove(usize, usize, usize),
    Read(usize, usize, usize),
    Write(usize, usize, usize),
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Div(usize, usize, usize),
    NotAnd(usize, usize, usize),
    Halt,
    Alloc(usize, usize),
    Dealloc(usize),
    Out(i8),
    In(i8),
    Load(usize, usize),
    Immediate(usize, i32),
}

impl From<i32> for Operator {
    fn from(bit_pattern: i32) -> Self {
        let a = (bit_pattern >> 6 & 7) as usize;
        let b = (bit_pattern >> 3 & 7) as usize;
        let c = (bit_pattern & 7) as usize;

        match bit_pattern >> 28 {
            0 => Operator::CondMove(a, b, c),
            1 => Operator::Read(a, b, c),
            2 => Operator::Write(a, b, c),
            3 => Operator::Add(a, b, c),
            4 => Operator::Mul(a, b, c),
            5 => Operator::Div(a, b, c),
            6 => Operator::NotAnd(a, b, c),
            7 => Operator::Halt,
            8 => Operator::Alloc(b, c),
            9 => Operator::Dealloc(c),
            10 => Operator::Out(c),
            11 => Operator::In(c),
            12 => Operator::Load(b, c),
            13 => Operator::Immediate(
                (bit_pattern >> 25 & 0b111) as i8,
                bit_pattern & 0b1111111111111111111111111 - 1,
            ),
            _ => Operator::Halt,
        }
    }
}

fn main() -> io::Result<()> {
    println!("Welcome to the Universal Machine");

    let mut program_source = File::open("../sandmark.umz")?;
    let mut buf = [0; 4];
    let mut instructions = Vec::<i32>::new();

    while let Ok(_) = program_source.read_exact(&mut buf) {
        instructions.push(i32::from_be_bytes(buf));
    }

    print!("test {0}\n", instructions.len());

    let mut memory = HashMap::<i32, Vec<i32>>::new();
    memory.insert(0, instructions);

    let um = UniversalMachine {
        instruction_pointer: 0,
        registers: [0; 8],
        memory,
        next_mem: 0,
    };

    UniversalMachine::run(um);

    Ok(())
}
