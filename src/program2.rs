use std::env::Args;
use crate::open_file_first_arg;
use std::io::BufRead;
use std::num::ParseIntError;

pub fn main1(args: &mut Args) -> Result<usize, String> {
    computer_from_args(args).and_then(|mut computer| set_and_calc(&mut computer, 12, 2))
}

pub fn main2(args: &mut Args) -> Result<usize, String> {
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

fn set_and_calc(computer: &mut Computer, noun: usize, verb: usize) -> Result<usize, String> {
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

#[derive(Clone)]
struct Computer {
    memory: Vec<usize>
}
impl Computer {
    pub fn from_line(s: &str) -> Result<Computer, ParseIntError> {
        let mem_res: Result<Vec<usize>, ParseIntError> =
            s.split(",").map(|s| s.parse::<usize>()).collect();
        mem_res.map(|mem| Computer { memory: mem } )
    }

    fn oob_err(&self, idx: usize) -> String {
        format!("Out of bounds index {} (max: {})", idx, self.memory.len() - 1)
    }

    pub fn get(&self, idx: usize) -> Result<&usize, String> {
        self.memory.get(idx).ok_or_else(|| self.oob_err(idx))
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut usize, String> {
        let err = self.oob_err(idx);
        if let Some(reference) = self.memory.get_mut(idx) { Ok(reference) }
        else { Err(
            err /* If I move computation of err here, compiler complains of borrowing self mutably
            and immutably at the same time. I think I am not. Why? Is this related to Polonius?
            http://smallcultfollowing.com/babysteps/blog/2018/06/15/mir-based-borrow-check-nll-status-update/
            */
        ) }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut index = 0usize;
        loop {
            let op_code = *self.get(index)?;
            match op_code {
                99 => return Ok(()),
                1 | 2 => {
                    let a_idx = self.get(index + 1)?;
                    let b_idx = self.get(index + 2)?;
                    let result_idx = *self.get(index + 3)?;
                    let a = *self.get(*a_idx)?;
                    let b = *self.get(*b_idx)?;
                    let result = self.get_mut(result_idx)?;
                    *result = if op_code == 1 { a + b } else { a * b };
                    index += 4;
                },
                other => return Err(format!("Unknown op code: {}", other))
            }
        }
    }
}
