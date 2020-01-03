use std::env::Args;
use permutohedron::Heap;
use crate::program2::{computer_from_args, Computer, MemData, ComputerMemory};
use std::collections::VecDeque;

pub fn main1(args: &mut Args) -> Result<MemData, String> {
    let initial_computer = computer_from_args(args)?;

    let mut data = [0u8, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);
    let mut max: Option<MemData> = None;
    for inputs in heap {
        let a = run_amplifier(initial_computer.memory(), inputs[0], 0)?;
        let b = run_amplifier(initial_computer.memory(), inputs[1], a)?;
        let c = run_amplifier(initial_computer.memory(), inputs[2], b)?;
        let d = run_amplifier(initial_computer.memory(), inputs[3], c)?;
        let f = run_amplifier(initial_computer.memory(), inputs[4], d)?;
        max = match max { None => Some(f), Some(v) => Some(v.max(f)) };
//        println!("{:?} = {:?}", inputs, f);
    }
    max.ok_or(String::from("No calculations were performed."))
}

//struct Amplifier { computer: Computer, input: Option<MemData>, last_output: Option<MemData> }
//impl Amplifier {
//    fn new(program: &Computer, phase_setting: u8) -> Amplifier {
//
//    }
//}
//
//struct Amplifiers([Amplifier; 5])
//impl Amplifiers {
//    fn new(program: &Computer, phase_settings: [u8; 5], input: MemData) -> Amplifiers {
//        let mut amplifiers = Amplifiers()
//        for idx in 0..5 {
//            amplifiers[idx] = Amplifier {
//                input: if idx == 0 { input } else 0,
//                computer: Computer::from_cloned_memory(
//                    &program,
//                    ComputerIO {
//                        input: ComputerInput(Box::new(|| {
//                            Ok(amplifiers.0[idx].input)
//                        }))
//                    }
//                )
//            }
//        }
//    }
//}

fn run_amplifier(initial: &ComputerMemory, phase_setting: u8, input: MemData) -> Result<MemData, String> {
    let mut amplifier = Computer::new(initial.clone());
    let outputs = amplifier.run(&mut VecDeque::from(vec![phase_setting as MemData, input]))?;
    outputs.get(0).map(|v| *v).ok_or(String::from("Computer produced no output!"))
}