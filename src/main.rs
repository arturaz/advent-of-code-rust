use std::env;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use std::env::Args;

mod program1;
mod program2;
mod program3;
mod program4;
mod program5;
mod program6;
mod program7;

fn main() {
    let mut args = env::args();
    let res = match args.nth(1) {
        None => Err("No args!".to_string()),
        Some(ref r) if r == "1-1" =>
            program1::main(&mut args, false).map(|v| v.to_string()),
        Some(ref r) if r == "1-2" =>
            program1::main(&mut args, true).map(|v| v.to_string()),
        Some(ref r) if r == "2-1" =>
            program2::main1(&mut args).map(|v| v.to_string()),
        Some(ref r) if r == "2-2" =>
            program2::main2(&mut args).map(|v| v.to_string()),
        Some(ref r) if r == "3-1" =>
            program3::main(&mut args, false).map(|v| v.to_string()),
        Some(ref r) if r == "3-2" =>
            program3::main(&mut args, true).map(|v| v.to_string()),
        Some(ref r) if r == "4-1" =>
            Ok(program4::main(264793..=803935, true).to_string()),
        Some(ref r) if r == "4-2" =>
            Ok(program4::main(264793..=803935, false).to_string()),
        Some(ref r) if r == "5-1" =>
            program5::main1(&mut args).map(|v|
                v.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join("\n")
            ),
        Some(ref r) if r == "5-2" =>
            program5::main2(&mut args).map(|v|
                v.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join("\n")
            ),
        Some(ref r) if r == "6-1" => program6::main1(&mut args).map(|v| v.to_string()),
        Some(ref r) if r == "6-2" => program6::main2(&mut args).map(|v| v.to_string()),
        Some(ref r) if r == "7-1" => program7::main1(&mut args).map(|v| v.to_string()),
        Some(ref r) if r == "7-2" => program7::main2(&mut args).map(|v| v.to_string()),
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

fn open_file_first_arg(args: &mut Args) -> Result<BufReader<File>, String> {
    let path = args.nth(0).ok_or("No input file!".to_string())?;
    open_file(&path)
}

fn open_file(path: &str) -> Result<BufReader<File>, String>{
    let file = File::open(&path)
        .map_err(|err| format!("Failed to open {}: {}", &path, err.to_string()))?;
    Ok(BufReader::new(file))
}
