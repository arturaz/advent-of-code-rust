use std::env::Args;
use permutohedron::Heap;
use crate::program2::{computer_from_args, ComputerIO, Computer, ComputerInput, ComputerOutput, MemData};

pub fn main1(args: &mut Args) -> Result<MemData, String> {
    let initial_computer = computer_from_args(args, ComputerIO::empty())?;

    let mut data = [0u8, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);
    let mut max: Option<MemData> = None;
    for inputs in heap {
        let a = run_amplifier(&initial_computer, inputs[0], 0)?;
        let b = run_amplifier(&initial_computer, inputs[1], a)?;
        let c = run_amplifier(&initial_computer, inputs[2], b)?;
        let d = run_amplifier(&initial_computer, inputs[3], c)?;
        let f = run_amplifier(&initial_computer, inputs[4], d)?;
        max = match max { None => Some(f), Some(v) => Some(v.max(f)) };
//        println!("{:?} = {:?}", inputs, f);
    }
    max.ok_or(String::from("No calculations were performed."))
}

fn run_amplifier(initial: &Computer, phase_setting: u8, input: MemData) -> Result<MemData, String> {
    let (outputs, output_io) = ComputerOutput::vec();
    {
        let mut amplifier = Computer::from_cloned_memory(
            &initial,
            ComputerIO {
                input: ComputerInput::from_vec(vec![phase_setting as MemData, input]),
                output: output_io
            }
        );
        amplifier.run()?;
    }
    outputs.try_unwrap_outputs()
        .and_then(|vec|
            vec.get(0).map(|v| *v).ok_or(String::from("Computer produced no output!"))
        )
}