#![feature(linked_list_cursors)]
use clap::Parser;
use std::fs;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod solution;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(format!("./input/day{}/input.txt", args.day))
        .expect("Could not read input");

    let solver = get_day_solver(args.day);

    let start = Instant::now();
    let (p1, p2) = solver(&input);
    let elapsed_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;

    println!("\n=== Day {:02} ===", args.day);
    println!("  · Part 1: {}", p1);
    println!("  · Part 2: {}", p2);
    println!("  · Elapsed: {:.4} ms", elapsed_ms);
}

fn get_day_solver(day: u8) -> fn(&str) -> solution::SolutionPair {
    match day {
        1 => day1::solve,
        2 => day2::solve,
        3 => day3::solve,
        4 => day4::solve,
        _ => panic!("Day {} not implemented", day),
    }
}
