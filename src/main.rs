use std::{env, process};

use adventofcode2022::{calendar, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Cannot parse arguments: {err}");
        process::exit(1);
    });
    
    match config.day_number {
        1 => calendar::day_1::run(config),
        2 => calendar::day_2::run(config),
        3 => calendar::day_3::run(config),
        d => {
            eprintln!("Not implemented yet for day: {d}");
            process::exit(1);
        }
    }
}
