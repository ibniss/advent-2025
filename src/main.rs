#![feature(linked_list_cursors)]
#![feature(macro_metavar_expr)]
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

mod grid;
mod solution;
mod utils;

use solution::{Day, Solution};

const ANSWERS_FILE: &str = "answers.txt";

/// Macro to register all day solutions.
/// Generates module declarations and the dispatch function.
///
/// Usage: `register_days!(day1, day2, day3, ...);`
macro_rules! register_days {
    ($($day:ident),* $(,)?) => {
        // Generate module declarations
        $(mod $day;)*

        // Solver function pointers for each part
        type PartSolver = fn(&str) -> Solution;
        type BothSolver = fn(&str) -> solution::SolutionPair;

        struct DaySolvers {
            part1: PartSolver,
            part2: PartSolver,
            both: BothSolver,
        }

        // Generate the dispatch function
        fn get_day_solvers(day: u8) -> DaySolvers {
            const SOLVERS: &[DaySolvers] = &[
                $(DaySolvers {
                    part1: $day::Solution::part1,
                    part2: $day::Solution::part2,
                    both: $day::Solution::solve,
                },)*
            ];

            let idx = (day - 1) as usize;
            if idx < SOLVERS.len() {
                DaySolvers {
                    part1: SOLVERS[idx].part1,
                    part2: SOLVERS[idx].part2,
                    both: SOLVERS[idx].both,
                }
            } else {
                panic!("Day {} not implemented", day)
            }
        }

        // Count of implemented days
        fn num_days() -> u8 {
            [$($day::Solution::solve,)*].len() as u8
        }
    };
}

// Register all implemented days - just list them in order!
register_days!(day1, day2, day3, day4, day5, day6);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run (omit to run all implemented days)
    day: Option<u8>,

    /// Run only part 1 or 2
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Save answers to answers.txt
    #[arg(short, long)]
    save: bool,

    /// Verify answers against answers.txt
    #[arg(short, long)]
    verify: bool,
}

/// Stored answers for a day
#[derive(Debug, Clone)]
struct Answers {
    part1: String,
    part2: String,
}

/// Load answers from answers.txt
fn load_answers() -> HashMap<u8, Answers> {
    let mut answers = HashMap::new();

    let content = match fs::read_to_string(ANSWERS_FILE) {
        Ok(c) => c,
        Err(_) => return answers,
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Format: "day: part1, part2"
        if let Some((day_str, rest)) = line.split_once(':') {
            if let Ok(day) = day_str.trim().parse::<u8>() {
                let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    answers.insert(
                        day,
                        Answers {
                            part1: parts[0].to_string(),
                            part2: parts[1].to_string(),
                        },
                    );
                }
            }
        }
    }

    answers
}

/// Save answers to answers.txt
fn save_answers(answers: &HashMap<u8, Answers>) {
    let mut days: Vec<_> = answers.keys().collect();
    days.sort();

    let content: String = days
        .iter()
        .map(|day| {
            let a = &answers[day];
            format!("{}: {}, {}", day, a.part1, a.part2)
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(ANSWERS_FILE, content + "\n").expect("Failed to write answers file");
}

fn main() {
    let args = Args::parse();

    let days: Vec<u8> = match args.day {
        Some(day) => vec![day],
        None => (1..=num_days()).collect(),
    };

    let mut stored_answers = load_answers();
    let mut all_passed = true;

    let total_start = Instant::now();

    for day in &days {
        let result = run_day(*day, args.part, args.verify, stored_answers.get(day));

        if args.save {
            // Get existing answers or create new
            let mut ans = stored_answers.get(day).cloned().unwrap_or(Answers {
                part1: String::new(),
                part2: String::new(),
            });

            // Update based on what we ran
            if let Some(p1) = &result.part1 {
                ans.part1 = p1.to_string();
            }
            if let Some(p2) = &result.part2 {
                ans.part2 = p2.to_string();
            }

            if result.part1.is_some() || result.part2.is_some() {
                stored_answers.insert(*day, ans);
            }
        }

        if !result.passed {
            all_passed = false;
        }
    }

    if days.len() > 1 {
        let total_elapsed = total_start.elapsed().as_nanos() as f64 / 1_000_000.0;
        println!("\n=== Total: {:.4} ms ===", total_elapsed);
    }

    if args.save {
        save_answers(&stored_answers);
        println!("\nAnswers saved to {}", ANSWERS_FILE);
    }

    if args.verify && !all_passed {
        std::process::exit(1);
    }
}

struct DayResult {
    part1: Option<Solution>,
    part2: Option<Solution>,
    passed: bool,
}

fn run_day(day: u8, part: Option<u8>, verify: bool, expected: Option<&Answers>) -> DayResult {
    let input = match fs::read_to_string(format!("./input/day{}/input.txt", day)) {
        Ok(i) => i,
        Err(_) => {
            println!("\n=== Day {:02} ===", day);
            println!("  · Skipped (no input file)");
            return DayResult {
                part1: None,
                part2: None,
                passed: true,
            };
        }
    };

    let solvers = get_day_solvers(day);

    let start = Instant::now();

    let (p1, p2) = match part {
        Some(1) => (Some((solvers.part1)(&input)), None),
        Some(2) => (None, Some((solvers.part2)(&input))),
        _ => {
            let (a, b) = (solvers.both)(&input);
            (Some(a), Some(b))
        }
    };

    let elapsed_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;

    println!("\n=== Day {:02} ===", day);

    let mut passed = true;

    if let Some(ref sol) = p1 {
        if verify {
            if let Some(exp) = expected {
                let ok = sol.to_string() == exp.part1;
                let status = if ok { "ok" } else { "FAIL" };
                println!("  · Part 1: {} [{}]", sol, status);
                if !ok {
                    println!("           expected: {}", exp.part1);
                    passed = false;
                }
            } else {
                println!("  · Part 1: {} [no expected answer]", sol);
            }
        } else {
            println!("  · Part 1: {}", sol);
        }
    }

    if let Some(ref sol) = p2 {
        if verify {
            if let Some(exp) = expected {
                let ok = sol.to_string() == exp.part2;
                let status = if ok { "ok" } else { "FAIL" };
                println!("  · Part 2: {} [{}]", sol, status);
                if !ok {
                    println!("           expected: {}", exp.part2);
                    passed = false;
                }
            } else {
                println!("  · Part 2: {} [no expected answer]", sol);
            }
        } else {
            println!("  · Part 2: {}", sol);
        }
    }

    println!("  · Elapsed: {:.4} ms", elapsed_ms);

    DayResult { part1: p1, part2: p2, passed }
}
