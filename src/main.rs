use std::io::Error;
use std::{
    fs::File,
    io::{self, Read},
};

pub struct UniversalMachine {
    instruction_pointer: usize,
    registers: [u32; 8],
    memory: Vec<Vec<u32>>,
    free_platters: Vec<usize>,
}

impl UniversalMachine {
    pub fn run(
        Self {
            mut instruction_pointer,
            mut registers,
            mut memory,
            free_platters: mut free_allocation_slots,
        }: Self,
    ) {
        loop {
            let instruction = memory[0][instruction_pointer];
            let operator = Operator::from(instruction);
            instruction_pointer = instruction_pointer + 1;
            // println!("{:?}", operator);
            match operator {
                Operator::CondMove(a, b, c) => {
                    if registers[c] != 0 {
                        registers[a] = registers[b]
                    }
                }
                Operator::Read(a, b, c) => {
                    let source = registers[b] as usize;
                    let offset = registers[c] as usize;
                    registers[a] = memory[source][offset];
                }
                Operator::Write(a, b, c) => {
                    let memory_allocation = registers[a] as usize;
                    if let Some(allocation) = memory.get_mut(memory_allocation) {
                        let offset = registers[b];
                        let value = registers[c];
                        allocation[offset as usize] = value
                    } else {
                        println!("Failed to write value {} to memory {}", registers[c], a);
                        break;
                    }
                }
                Operator::Add(a, b, c) => registers[a] = registers[b].wrapping_add(registers[c]),
                Operator::Mul(a, b, c) => registers[a] = registers[b].wrapping_mul(registers[c]),
                Operator::Div(a, b, c) => registers[a] = registers[b].wrapping_div(registers[c]),
                Operator::NotAnd(a, b, c) => registers[a] = !(registers[b] & registers[c]),
                Operator::Halt => break,
                Operator::Alloc(b, c) => {
                    let allocation_size = registers[c] as usize;
                    let allocation = vec![0; allocation_size];
                    let allocation_slot = match free_allocation_slots.pop() {
                        Some(free_allocation_slot) => {
                            memory[free_allocation_slot] = allocation;
                            free_allocation_slot
                        }
                        None => {
                            memory.push(allocation);
                            memory.len() - 1
                        }
                    };

                    registers[b] = allocation_slot as u32;
                }
                Operator::Dealloc(c) => {
                    let allocation_slot = registers[c] as usize;
                    memory[allocation_slot] = vec![];
                    free_allocation_slots.push(allocation_slot);
                }
                Operator::Out(c) => print!("{}", registers[c] as u8 as char),
                Operator::In(c) => {
                    let mut buffer = [0; 1];
                    match io::stdin().read_exact(&mut buffer) {
                        Ok(()) => registers[c] = buffer[0] as u32,
                        _ => registers[c] = 0,
                    }
                }
                Operator::Load(b, c) => {
                    let target = registers[b] as usize;
                    if target != 0 {
                        if let Some(program) = memory.get(target) {
                            let program_clone = program.clone();
                            memory[0] = program_clone;
                        } else {
                            println!("Failed to load program from array {}", b);
                            break;
                        }
                    }
                    instruction_pointer = registers[c] as usize;
                }
                Operator::Immediate(a, value) => registers[a] = value,
                Operator::Unsupported(opcode) => {
                    println!("Unsupported instruction: {}", opcode);
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
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
    Out(usize),
    In(usize),
    Load(usize, usize),
    Immediate(usize, u32),
    Unsupported(u32),
}

impl From<u32> for Operator {
    fn from(instruction: u32) -> Self {
        let a = (instruction >> 6 & 7) as usize;
        let b = (instruction >> 3 & 7) as usize;
        let c = (instruction & 7) as usize;
        let opcode = instruction >> 28;

        match opcode {
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
                (instruction >> 25 & 7) as usize,
                instruction & 0b1111111111111111111111111,
            ),
            opcode => Operator::Unsupported(opcode),
        }
    }
}

fn read_program(program_file_name: &str) -> Result<Vec<u32>, Error> {
    println!("Input file: {}", program_file_name);

    let mut program_source = File::open(program_file_name)?;
    let mut buf = [0; 4];
    let mut instructions = Vec::<u32>::new();

    while let Ok(_) = program_source.read_exact(&mut buf) {
        let instruction = u32::from_be_bytes(buf);
        instructions.push(instruction);
        // println!("{:?}", Operator::from(instruction));
    }

    print!("Number of instructions: {}\n", instructions.len());

    Result::Ok(instructions)
}

fn main() -> io::Result<()> {
    println!("Welcome to the Universal Machine.");

    // let program_file = "../sandmark.umz";
    // let program_file = "../um.um";
    let program_file = "../codex.umz";
    let program = read_program(program_file)?;
    // let mut memory = HashMap::<usize, Vec<u32>>::new();
    let mut memory = vec![];
    memory.insert(0, program);

    let um = UniversalMachine {
        instruction_pointer: 0,
        registers: [0; 8],
        memory,
        free_platters: vec![],
    };

    UniversalMachine::run(um);

    println!();

    Ok(())
}
