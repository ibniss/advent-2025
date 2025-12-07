# advent-2025

Advent of Code 2025 solutions in Rust.

## Requirements

- Rust nightly (uses `linked_list_cursors` and `macro_metavar_expr` features)

## Project Structure

```
src/
  main.rs       # CLI and day registration
  solution.rs   # Day trait and Solution type
  dayN.rs       # Solution for day N
  ...           # Other util modules
input/
  dayN/
    input.txt   # Puzzle input (gitignored)
answers.txt     # Saved answers for regression testing (gitignored)
```

## Adding a New Day

1. Create `src/dayN.rs`:

```rust
use crate::solution::Day;

pub struct Solution;

impl Day for Solution {
    fn part1(input: &str) -> crate::solution::Solution {
        todo!()
    }

    fn part2(input: &str) -> crate::solution::Solution {
        todo!()
    }
}
```

2. Register it in `src/main.rs`:

```rust
register_days!(day1, day2, ..., dayN);
```

3. Add input to `input/dayN/input.txt`

## CLI Usage

```bash
cargo run              # Run all days
cargo run -- 3         # Run day 3
cargo run -- 3 -p 1    # Run only part 1 of day 3
cargo run -- 3 -p 2    # Run only part 2 of day 3
```

### Answer Management

Answers are stored in `answers.txt`, which is gitignored. Used locally for regression testing.

```bash
cargo run -- --save       # Save all answers to answers.txt
cargo run -- 3 --save     # Save day 3 answers
cargo run -- --verify     # Verify all answers against answers.txt
cargo run -- 3 -v         # Verify day 3 answers
```

The `--verify` flag exits with code 1 if any answers don't match.

## Testing

```bash
cargo test             # Run all unit tests
cargo test day3        # Run day 3 tests only
```


## AI Disclosure

I have not used AI to solve challenges directly. In some cases, I have used it to debug a particular language-level issue due to my limited Rust knowledge.
I have also used it after solving a challenge to refactor and improve the code to be more idiomatic, and to help build out utils/the project harness.
