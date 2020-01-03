use std::env::Args;
use permutohedron::Heap;
use crate::program2::{computer_from_args, Computer, MemData, ComputerMemory};
use std::collections::VecDeque;
use arraymap::ArrayMap;

pub fn main1(args: &mut Args) -> Result<MemData, String> {
    let initial_computer = computer_from_args(args)?;

    let mut data = [0u8, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);
    let mut max: Option<MemData> = None;
    for inputs in heap {
        let result = AmplifierChain::new(initial_computer.memory(), inputs).run(0)?;
        max = match max { None => Some(result), Some(v) => Some(v.max(result)) };
//        println!("{:?} = {:?}", inputs, f);
    }
    max.ok_or(String::from("No calculations were performed."))
}

pub fn main2(args: &mut Args) -> Result<MemData, String> {
    let initial_computer = computer_from_args(args)?;

    let mut data = [5u8, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);
    let mut max: Option<MemData> = None;
    for inputs in heap {
        let result = AmplifierChain::new(initial_computer.memory(), inputs).run(0)?;
        max = match max { None => Some(result), Some(v) => Some(v.max(result)) };
//        println!("{:?} = {:?}", inputs, f);
    }
    max.ok_or(String::from("No calculations were performed."))
}

struct Amplifier { computer: Computer, inputs: VecDeque<MemData>, outputs: Vec<MemData> }
impl Amplifier {
    fn new(program: &ComputerMemory, phase_setting: u8) -> Amplifier {
        let mut inputs = VecDeque::<MemData>::new();
        inputs.push_front(phase_setting as MemData);
        Amplifier {
            computer: Computer::new(program.clone()),
            inputs,
            outputs: Vec::new()
        }
    }

    fn run(&mut self, input: MemData) -> Result<MemData, String> {
        self.inputs.push_back(input);
        self.computer.run(&mut self.inputs, &mut self.outputs)?;
        self.get_last_output()
    }

    fn get_last_output(&self) -> Result<MemData, String> {
        self.outputs.last().map(|v| *v).ok_or_else(|| String::from("No output!"))
    }
}

struct AmplifierChain([Amplifier; 5]);
impl AmplifierChain {
    fn new(program: &ComputerMemory, phase_settings: [u8; 5]) -> AmplifierChain {
        let amplifiers = phase_settings.map(|phase_setting| Amplifier::new(program, *phase_setting));
        AmplifierChain(amplifiers)
    }

    fn get_last_output(&self, idx: usize) -> Result<MemData, String> {
        self.0.get(idx)
            .ok_or_else(|| format!("Wrong index: {}", idx))
            .and_then(|a| a.get_last_output())
    }

    fn run(&mut self, input: MemData) -> Result<MemData, String> {
        let last_idx = self.0.len() - 1;
        for idx in 0..=last_idx {
            let amplifier_input = if idx == 0 { input } else { self.get_last_output(idx - 1)? };

            let amplifier = &mut self.0[idx];
            amplifier.outputs.clear();
            amplifier.run(amplifier_input)?;
        }

        self.get_last_output(last_idx)
    }
}