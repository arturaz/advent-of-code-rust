use crate::program2::*;
use std::env::Args;

pub fn main1(args: &mut Args) -> Result<Vec<MemData>, String> {
    main(args, vec![1])
}

pub fn main2(args: &mut Args) -> Result<Vec<MemData>, String> {
    main(args, vec![5])
}

fn main(args: &mut Args, data: Vec<MemData>) -> Result<Vec<MemData>, String> {
    let (outputs, output_io) = ComputerOutput::vec();
    {
        let io = ComputerIO {
            input: ComputerInput::from_vec(data),
            output: output_io
        };
        computer_from_args(args, io).and_then(|mut computer| computer.run())?;
    }

    outputs.try_unwrap_outputs()
}