use crate::program2::*;
use std::env::Args;
use std::cell::RefCell;
use std::rc::Rc;

pub fn main1(args: &mut Args) -> Result<Vec<MemData>, String> {
    main(args, vec![1])
}

pub fn main2(args: &mut Args) -> Result<Vec<MemData>, String> {
    main(args, vec![5])
}

fn main(args: &mut Args, data: Vec<MemData>) -> Result<Vec<MemData>, String> {
    let outputs = Rc::new(RefCell::new(Vec::<MemData>::new()));
    {
        let outputs = Rc::clone(&outputs);
        let io = ComputerIO {
            input: ComputerInput::from_vec(data),
            output: ComputerOutput(Box::new(move |data| outputs.borrow_mut().push(data)))
        };
        computer_from_args(args, io).and_then(|mut computer| computer.run())?;
    }

    Ok(Rc::try_unwrap(outputs).map_err(|_| String::from("Can't unwrap!"))?.into_inner())
}