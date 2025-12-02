use std::fs;

use clap::Parser;
mod day1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(format!("./input/day{}/input.txt", args.day))
        .expect("Could not read input");

    match args.day {
        1 => day1::run(&input),
        _ => panic!("Day {} not implemented", args.day),
    }
}
