use std::env::Args;
use permutohedron::Heap;
use crate::program2::{computer_from_args, Computer, MemData, ComputerMemory, ComputerError, ComputerMemoryIndex};
use std::collections::VecDeque;
use arraymap::ArrayMap;

pub fn main1(args: &mut Args) -> Result<MemData, String> {
    let initial_computer = computer_from_args(args)?;

    let mut data = [0u8, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);
    let mut max: Option<MemData> = None;
    for inputs in heap {
        let result = AmplifierChain::new(initial_computer.memory(), inputs).run(0, false)
            .map_err(|err| format!("{:?}", err))?;
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
        let result = AmplifierChain::new(initial_computer.memory(), inputs).run(0, true)
            .map_err(|err| format!("{:?}", err))?;
        max = match max { None => Some(result), Some(v) => Some(v.max(result)) };
//        println!("{:?} = {:?}", inputs, f);
    }
    max.ok_or(String::from("No calculations were performed."))
}

struct Amplifier {
    computer: Computer, inputs: VecDeque<MemData>, outputs: Vec<MemData>,
    last_index: Option<ComputerMemoryIndex>
}
impl Amplifier {
    fn new(program: &ComputerMemory, phase_setting: u8) -> Amplifier {
        let mut inputs = VecDeque::<MemData>::new();
        inputs.push_front(phase_setting as MemData);
        Amplifier {
            computer: Computer::new(program.clone()),
            inputs,
            outputs: Vec::new(),
            last_index: None
        }
    }

    fn run(&mut self, input: MemData, feedback_loop_mode: bool) -> Result<(), ComputerError> {
        self.inputs.push_back(input);
        let result = match self.last_index {
            None => self.computer.run(&mut self.inputs, &mut self.outputs),
            Some(index) => self.computer.run_from(index, &mut self.inputs, &mut self.outputs),
        };
        match result {
            Err(ComputerError::OutOfInputs(last_index)) if feedback_loop_mode => {
                self.last_index = Some(last_index);
                Ok(())
            },
            other => {
                self.last_index = None;
                other
            },
        }
    }

    fn get_last_output(&self) -> Result<MemData, ComputerError> {
        self.outputs.last().map(|v| *v)
            .ok_or_else(|| ComputerError::Other(String::from("No output!")))
    }
}

struct AmplifierChain([Amplifier; 5]);
impl AmplifierChain {
    fn new(program: &ComputerMemory, phase_settings: [u8; 5]) -> AmplifierChain {
        let amplifiers = phase_settings.map(|phase_setting| Amplifier::new(program, *phase_setting));
        AmplifierChain(amplifiers)
    }

    fn get_last_output(&self, idx: usize) -> Result<MemData, ComputerError> {
        self.0.get(idx)
            .ok_or_else(|| ComputerError::Other(format!("Wrong index: {}", idx)))
            .and_then(|a| a.get_last_output().map_err(|err| ComputerError::Other(format!("For {}: {:?}", idx, err))))
    }

    fn run(&mut self, input: MemData, feedback_loop: bool) -> Result<MemData, ComputerError> {
        let last_idx = self.0.len() - 1;

        let mut current_input = input;
        loop {
            println!("Iter");
            for idx in 0..=last_idx {
                let amplifier_input =
                    if idx == 0 { current_input }
                    else { self.get_last_output(idx - 1)? };

                let amplifier = &mut self.0[idx];
                println!("{} from {:?} with input {}", idx, amplifier.last_index, amplifier_input);
                amplifier.outputs.clear();
                amplifier.run(amplifier_input, feedback_loop)
                    .map_err(|err| ComputerError::Other(format!("Amplifier {}: {:?}", idx, err)))?;
                println!("outputs={:?}", amplifier.outputs);
            }

            if feedback_loop { current_input = self.get_last_output(last_idx)?; }
            else { break; }
        }

        self.get_last_output(last_idx)
    }
}