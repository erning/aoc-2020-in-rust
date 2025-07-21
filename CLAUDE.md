# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a complete Advent of Code 2020 implementation in Rust, with solutions for all 25 days. The project follows a consistent modular structure with standardized input handling and testing patterns.

## Architecture

- **Entry Point**: `src/main.rs` - Main executable that runs all or selected days
- **Library**: `src/lib.rs` - Re-exports all day modules and provides I/O utilities
- **Day Modules**: `src/dayXX.rs` - Individual solutions for each day (01-25)
- **Input Files**: `inputs/` - Contains example and actual input files for each day

## Key Patterns

Each day module follows this consistent structure:
- Private `parse_input()` function for parsing raw input into structured data
- Public `part_one()` and `part_two()` functions that take string input and return results
- Tests that use `read_example()` to validate against provided examples

## Commands

### Build and Run
```bash
cargo build --release
cargo run --release --               # Run all days
cargo run --release -- 1 5 10        # Run specific days
cargo run --release -- --example     # Use example inputs
```

### Testing
```bash
cargo test                           # Run all tests
cargo test --lib                     # Run library tests only
cargo test --bin aoc                 # Run binary tests (none currently)
```

### Development
```bash
cargo check                          # Quick syntax/type checking
cargo clippy                         # Linting with Clippy
cargo fmt                            # Format code
```

### Input Handling
- `aoc::read_input(day)` - Read actual input for day (01-25)
- `aoc::read_example(day)` - Read example input for day
- `aoc::read_as_string(day, filename)` - Read custom filename

## File Structure

```
src/
├── main.rs          # Main runner with puzzle registry
├── lib.rs           # Library root, I/O utilities
├── dayXX.rs         # Individual day solutions (01-25)
inputs/
├── XX-input.txt     # Real puzzle input
├── XX-example.txt   # Example input
└── 14-example-2.txt # Special case for day 14 part 2
```

## Common Tasks

- **Add new day**: Create `src/day26.rs` following established pattern, add to lib.rs and main.rs
- **Test single day**: `cargo test day05` (runs tests in day05 module)
- **Run with timing**: `cargo run --release -- --time 5`