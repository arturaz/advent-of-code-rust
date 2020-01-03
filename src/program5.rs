use crate::program2::*;
use std::env::Args;
use std::collections::VecDeque;

pub fn main1(args: &mut Args) -> Result<Vec<MemData>, String> {
    main(args, vec![1])
}

pub fn main2(args: &mut Args) -> Result<Vec<MemData>, String> {
    main(args, vec![5])
}

fn main(args: &mut Args, data: Vec<MemData>) -> Result<Vec<MemData>, String> {
    let mut inputs = VecDeque::from(data);
    computer_from_args(args).and_then(|mut computer| computer.run(&mut inputs))
}