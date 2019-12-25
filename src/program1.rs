use std::env::Args;
use std::io::{BufRead};
use crate::open_file_first_arg;

pub fn main(args: &mut Args, recurse_fuel: bool) -> Result<u64, String> {
    fn fuel_for(mass: u64, current: u64, recurse: bool) -> u64 {
        let fuel = (mass / 3).checked_sub(2).unwrap_or(0);
        if recurse && fuel != 0 { fuel_for(fuel, current + fuel, recurse) }
        else { current + fuel }
    }

    let reader = open_file_first_arg(args)?;
    let fuel_results  = reader.lines().map(|line_res|
        line_res
            .map_err(|err| format!("Reading line failed: {}", err))
            .and_then(|line|
                line.parse::<u64>()
                    .map_err(|err| format!("Can't parse '{}' as number: {}", line, err))
                    .map(|mass| fuel_for(mass, 0, recurse_fuel))
            )
    );
    let fuels_res = fuel_results.collect::<Result<Vec<u64>, String>>();
    fuels_res.map(|vec| vec.iter().fold(0, |a, b| a + *b))
}