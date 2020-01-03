use std::env::Args;
use crate::open_file_first_arg;
use std::io::BufRead;
use std::num::ParseIntError;
use std::convert::TryFrom;
use std::collections::VecDeque;

pub type MemData = i64;

pub fn main1(args: &mut Args) -> Result<MemData, String> {
    computer_from_args(args)
        .and_then(|mut computer| set_and_calc(&mut computer, 12, 2))
}

pub fn main2(args: &mut Args) -> Result<MemData, String> {
    computer_from_args(args).and_then(|mut computer| {
        let orig_mem = computer.memory.clone();
        for noun in 0..99 {
            for verb in 0..99 {
                computer.memory = orig_mem.clone();
                if let Ok(19690720) = set_and_calc(&mut computer, noun, verb) {
                    return Ok(100 * noun + verb);
                }
            }
        }
        Err(String::from("Can't find noun & verb!"))
    })
}

fn set_and_calc(computer: &mut Computer, noun: MemData, verb: MemData) -> Result<MemData, String> {
    *computer.get_mut(1)? = noun;
    *computer.get_mut(2)? = verb;
    // What value is left at position 0 after the program halts?
    let _ = computer.run_no_input()?;
    Ok(*computer.get(0)?)
}

pub fn computer_from_args(args: &mut Args) -> Result<Computer, String> {
    let reader = open_file_first_arg(args)?;
    reader.lines().nth(0).ok_or("No lines in file!".to_string())
        .and_then(|line_res|
            line_res
                .map_err(|err| err.to_string())
                .and_then(|line|
                  ComputerMemory::from_line(&line).map_err(|err| err.to_string())
                      .map(|mem| Computer::new(mem))
                )
        )
}

#[derive(Eq, PartialEq)]
enum OpCode {
    Add, Multiply, Halt, TakeInput, Output, JumpIfTrue, JumpIfFalse, LessThan, Equals
}
impl OpCode {
    fn parse(data: MemData) -> Result<OpCode, String> {
        match data {
            99 => Ok(OpCode::Halt),
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Multiply),
            3 => Ok(OpCode::TakeInput),
            4 => Ok(OpCode::Output),
            5 => Ok(OpCode::JumpIfTrue),
            6 => Ok(OpCode::JumpIfFalse),
            7 => Ok(OpCode::LessThan),
            8 => Ok(OpCode::Equals),
            _ => Err(format!("Unknown op code {}", data))
        }
    }
}

#[derive(Copy, Clone)]
pub enum ParameterMode { Position, Immediate }

struct Instruction {
    op_code: OpCode,
    parameter_modes: Vec<ParameterMode>
}
impl Instruction {
    fn parse(data: MemData) -> Result<Instruction, String> {
        let op_code = OpCode::parse(data % 100)?;
        let mut parameters_data = data / 100;
        let mut parameter_modes = Vec::new();
        while parameters_data > 0 {
            let mode_res = match parameters_data % 10 {
                0 => Ok(ParameterMode::Position),
                1 => Ok(ParameterMode::Immediate),
                other => Err(format!("Unknown parameter mode {} for parameter {}", other, parameter_modes.len())),
            };
            parameter_modes.push(mode_res?);
            parameters_data /= 10;
        }
        Ok(Instruction { op_code, parameter_modes })
    }

    fn parameter_mode(&self, idx: usize) -> ParameterMode {
        self.parameter_modes.get(idx).map_or_else(|| ParameterMode::Position, |r| *r)
    }
}

#[derive(Clone)]
pub struct ComputerMemory(Vec<MemData>);
impl ComputerMemory {
    pub fn from_line(s: &str) -> Result<ComputerMemory, ParseIntError> {
        let mem_res: Result<Vec<_>, ParseIntError> =
            s.split(",").map(|s| s.parse::<MemData>()).collect();
        mem_res.map(|memory| ComputerMemory(memory))
    }
}

pub struct Computer {
    memory: ComputerMemory
}
impl Computer {
    pub fn new(memory: ComputerMemory) -> Computer {
        Computer { memory }
    }

    fn oob_err(&self, idx: usize) -> String {
        format!("Out of bounds index {} (max: {})", idx, self.memory.0.len() - 1)
    }

    pub fn memory(&self) -> &ComputerMemory { &self.memory }

    pub fn get(&self, idx: usize) -> Result<&MemData, String> {
        self.memory.0.get(idx).ok_or_else(|| self.oob_err(idx))
    }

    pub fn get_idx(&self, idx: usize) -> Result<usize, String> {
        self.get(idx).and_then(|v|
            usize::try_from(*v)
                .map_err(|err| format!("Value {} at index {} is not usize: {}!", v, idx, err))
        )
    }

    pub fn get_param(&self, idx: usize, mode: ParameterMode) -> Result<&MemData, String> {
        match mode {
            ParameterMode::Position => {
                self.get_idx(idx).and_then(|i| self.get(i))
            },
            ParameterMode::Immediate => self.get(idx),
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut MemData, String> {
        if idx <= self.memory.0.len() - 1 { Ok(unsafe { self.memory.0.get_unchecked_mut(idx) }) }
        else { Err(self.oob_err(idx)) }
    }

    pub fn get_output_cell(&mut self, idx: usize) -> Result<&mut MemData, String> {
        let cell_idx = self.get_idx(idx)?;
        self.get_mut(cell_idx)
    }

    pub fn run_no_input(&mut self) -> Result<Vec<MemData>, String> { self.run(&mut VecDeque::new()) }

    pub fn run(
        &mut self, inputs: &mut VecDeque<MemData>
    ) -> Result<Vec<MemData>, String> {
        let mut index = 0usize;
        let mut outputs = Vec::<MemData>::new();
        loop {
            let instruction = Instruction::parse(*self.get(index)?)?;
            let get_param = |idx: u8| {
                self.get_param(index + idx as usize + 1, instruction.parameter_mode(idx.into()))
            };
            let jump_if = |jump_if_true: bool| {
                let is_true = *get_param(0)? != 0;
                if is_true == jump_if_true {
                    let p2 = *get_param(1)?;
                    usize::try_from(p2).map_err(|err|
                        format!("Value {} at for jump if {} is not usize: {}!", p2, jump_if_true, err)
                    )
                } else {
                    Ok(index + 3)
                }
            };

            match instruction.op_code {
                OpCode::Halt => return Ok(outputs),
                OpCode::Add | OpCode::Multiply => {
                    let a = *get_param(0)?;
                    let b = *get_param(1)?;
                    let result = self.get_output_cell(index + 3)?;
                    *result = if instruction.op_code == OpCode::Add { a + b } else { a * b };
                    index += 4;
                },
                OpCode::TakeInput => {
                    let input = inputs.pop_front().ok_or_else(|| String::from("Out of inputs!"))?;
                    let result = self.get_output_cell(index + 1)?;
                    *result = input;
                    index += 2;
                },
                OpCode::Output => {
                    let value = *get_param(0)?;
                    outputs.push(value);
                    index += 2;
                },
                OpCode::JumpIfTrue => {
                    index = jump_if(true)?;
                },
                OpCode::JumpIfFalse => {
                    index = jump_if(false)?;
                },
                OpCode::LessThan => {
                    let p0 = *get_param(0)?;
                    let p1 = *get_param(1)?;
                    let result = self.get_output_cell(index + 3)?;
                    *result = if p0 < p1 { 1 } else { 0 };
                    index += 4;
                },
                OpCode::Equals => {
                    let p0 = *get_param(0)?;
                    let p1 = *get_param(1)?;
                    let result = self.get_output_cell(index + 3)?;
                    *result = if p0 == p1 { 1 } else { 0 };
                    index += 4;
                }
            }
        }
    }
}