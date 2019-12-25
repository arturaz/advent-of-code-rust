use std::env;
use std::process::exit;

mod program1;

fn main() {
    let mut args = env::args();
    let res = match args.nth(1) {
        None => Err("No args!".to_string()),
        Some(ref r) if r == "1-1" =>
            program1::main(args, false).map(|v| v.to_string()),
        Some(ref r) if r == "1-2" =>
            program1::main(args, true).map(|v| v.to_string()),
        Some(other) => Err(format!("Unknown arg: {}", other))
    };
    match res {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        },
    }
}
