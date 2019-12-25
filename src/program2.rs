use std::env::Args;
use crate::open_file_first_arg;
use std::io::BufRead;
use std::num::ParseIntError;

pub fn main(args: &mut Args) -> Result<usize, String> {
    let reader = open_file_first_arg(args)?;
    reader.lines().nth(0).ok_or("No lines in file!".to_string())
        .and_then(|line_res|
            line_res
                .map_err(|err| err.to_string())
                .and_then(|line|
                    Computer::from_line(&line).map_err(|err| err.to_string() )
                )
        )
        .and_then(|mut computer| {
            // replace position 1 with the value 12 and replace position 2 with the value 2.
            *computer.get_mut(1)? = 12;
            *computer.get_mut(2)? = 2;
            // What value is left at position 0 after the program halts?
            let _ = computer.run()?;
            Ok(*computer.get(0)?)
        })
}

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
        // TODO: make this lazy
        let err = self.oob_err(idx);
        match self.memory.get_mut(idx) {
            None => Err(err),
            Some(reference) => Ok(reference),
        }
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
