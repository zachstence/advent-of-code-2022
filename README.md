# advent-of-code-2022

My solutions to [Advent of Code 2022 problems](https://adventofcode.com/2022).

## Usage
```sh
# Run solution for current day
cargo aoc

# Run solution for specific day/part
cargo aoc -d 2 -p 1
```

More docs at https://github.com/gobanos/cargo-aoc

## Lessons Learned
- Day 11
    - `f32` is not as big as you think it is
    - `as f32` will silently overflow. Instead use `into`/`try_into`, or use one of the `.div*` helper functions

## Other
My solutions from last year https://github.com/zachstence/advent-of-code-2021