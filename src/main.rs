#![feature(linked_list_cursors)]
use std::fs;

use clap::Parser;

mod day1;
mod day2;
mod day3;

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
        2 => day2::run(&input),
        3 => day3::run(&input),
        _ => panic!("Day {} not implemented", args.day),
    }
}
