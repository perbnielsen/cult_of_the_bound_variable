use std::convert::TryInto;
use std::io::Error;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

pub struct UniversalMachine {
    instruction_pointer: usize,
    registers: [u32; 8],
    memory: HashMap<usize, Vec<u32>>,
    next_mem: usize,
    free_mem: Vec<usize>,
}

impl UniversalMachine {
    pub fn run(
        Self {
            mut instruction_pointer,
            mut registers,
            mut memory,
            mut next_mem,
            mut free_mem,
        }: Self,
    ) {
        loop {
            let instruction = memory[&0][instruction_pointer];
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
                    registers[a] = memory[&source][offset]
                }
                Operator::Write(a, b, c) => {
                    let memory_allocation = registers[a] as usize;
                    if let Some(allocation) = memory.get_mut(&memory_allocation) {
                        let offset = registers[b];
                        let value = registers[c];
                        // println!(
                        //     "Writing {} to memory {} at offset {} from register {}",
                        //     value, memory_allocation, offset, a
                        // );
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
                    let mem_size = registers[c] as usize;

                    let mem_index = match free_mem.pop() {
                        Some(free_index) => free_index,
                        None => {
                            let free_index = next_mem;
                            next_mem += 1;
                            free_index
                        }
                    };

                    memory.insert(mem_index, vec![0; mem_size]);
                    registers[b] = mem_index as u32;
                }
                Operator::Dealloc(c) => {
                    let target = registers[c] as usize;
                    memory.remove(&target);
                    free_mem.push(target);
                }
                Operator::Out(c) => print!("{}", registers[c as usize] as u8 as char),
                Operator::In(c) => {
                    let mut buffer = String::new();
                    if let Ok(length) = io::stdin().read_to_string(&mut buffer) {
                        if length == 1 {
                            registers[c] = buffer.into_bytes()[0] as u32;
                        } else {
                            registers[c] = 0;
                        }
                    } else {
                        registers[c] = 0;
                    }
                }
                Operator::Load(b, c) => {
                    let target = registers[b] as usize;
                    if target != 0 {
                        if let Some(program) = memory.get(&target) {
                            let program_clone = program.clone();
                            memory.insert(0, program_clone);
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
    Out(i8),
    In(usize),
    Load(usize, usize),
    Immediate(usize, u32),
    Unsupported(u32),
}

impl From<u32> for Operator {
    fn from(bit_pattern: u32) -> Self {
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
            10 => Operator::Out(c.try_into().unwrap()),
            11 => Operator::In(c),
            12 => Operator::Load(b, c),
            13 => Operator::Immediate(
                (bit_pattern >> 25 & 7) as usize,
                bit_pattern & 0b1111111111111111111111111, // - 1,
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

    let program_file = "../sandmark.umz";
    // let program_file = "../um.um";
    // let program_file = "../codex.umz";
    let program = read_program(program_file)?;
    let mut memory = HashMap::<usize, Vec<u32>>::new();
    memory.insert(0, program);

    let um = UniversalMachine {
        instruction_pointer: 0,
        registers: [0; 8],
        memory,
        next_mem: 1,
        free_mem: vec![],
    };

    UniversalMachine::run(um);

    println!();

    Ok(())
}
