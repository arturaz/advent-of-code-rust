use std::env::Args;
use crate::open_file_first_arg;
use std::io::BufRead;
use std::num::ParseIntError;
use std::convert::TryFrom;

type MemData = i64;

pub fn main1(args: &mut Args) -> Result<MemData, String> {
    computer_from_args(args).and_then(|mut computer| set_and_calc(&mut computer, 12, 2))
}

pub fn main2(args: &mut Args) -> Result<MemData, String> {
    computer_from_args(args).and_then(|computer_orig| {
        for noun in 0..99 {
            for verb in 0..99 {
                let mut computer = computer_orig.clone();
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
    let _ = computer.run()?;
    Ok(*computer.get(0)?)
}

fn computer_from_args(args: &mut Args) -> Result<Computer, String> {
    let reader = open_file_first_arg(args)?;
    reader.lines().nth(0).ok_or("No lines in file!".to_string())
        .and_then(|line_res|
            line_res
                .map_err(|err| err.to_string())
                .and_then(|line|
                    Computer::from_line(&line).map_err(|err| err.to_string() )
                )
        )
}

//mod computer {
//    enum ParameterMode { Position, Immediate }
//}

#[derive(Clone)]
struct Computer {
    memory: Vec<i64>
}
impl Computer {
    pub fn from_line(s: &str) -> Result<Computer, ParseIntError> {
        let mem_res: Result<Vec<_>, ParseIntError> =
            s.split(",").map(|s| s.parse::<MemData>()).collect();
        mem_res.map(|mem| Computer { memory: mem } )
    }

    fn oob_err(&self, idx: usize) -> String {
        format!("Out of bounds index {} (max: {})", idx, self.memory.len() - 1)
    }

    pub fn get(&self, idx: usize) -> Result<&MemData, String> {
        self.memory.get(idx).ok_or_else(|| self.oob_err(idx))
    }

    pub fn get_idx(&self, idx: usize) -> Result<usize, String> {
        self.get(idx).and_then(|v|
            usize::try_from(*v)
                .map_err(|err| format!("Value {} at index {} is not usize: {}!", v, idx, err))
        )
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut MemData, String> {
        if idx <= self.memory.len() - 1 { Ok(unsafe { self.memory.get_unchecked_mut(idx) }) }
        else { Err(self.oob_err(idx)) }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut index = 0usize;
        loop {
            let op_code = *self.get(index)?;
            match op_code {
                99 => return Ok(()),
                1 | 2 => {
                    let a_idx = self.get_idx(index + 1)?;
                    let b_idx = self.get_idx(index + 2)?;
                    let result_idx = self.get_idx(index + 3)?;
                    let a = *self.get(a_idx)?;
                    let b = *self.get(b_idx)?;
                    let result = self.get_mut(result_idx)?;
                    *result = if op_code == 1 { a + b } else { a * b };
                    index += 4;
                },
                3 => {
                    unimplemented!()
                },
                4 => {
                    unimplemented!()
                },
                other => return Err(format!("Unknown op code: {}", other))
            }
        }
    }
}
