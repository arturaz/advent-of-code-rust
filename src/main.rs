use std::env;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use std::env::Args;

mod program1;
mod program2;

fn main() {
    let mut args = env::args();
    let res = match args.nth(1) {
        None => Err("No args!".to_string()),
        Some(ref r) if r == "1-1" =>
            program1::main(&args, false).map(|v| v.to_string()),
        Some(ref r) if r == "1-2" =>
            program1::main(&args, true).map(|v| v.to_string()),
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

fn open_file_first_arg(mut args: &Args) -> Result<BufReader<File>, String> {
    let path = args.nth(0).ok_or("No input file!".to_string())?;
    open_file(&path)
}

fn open_file(path: &str) -> Result<BufReader<File>, String>{
    let file = File::open(&path)
        .map_err(|err| format!("Failed to open {}: {}", &path, err.to_string()))?;
    Ok(BufReader::new(file))
}
