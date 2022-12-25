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
    - Even though an IEEE 754 32-bit floating point number has a max value of 3.40282347E+38, it can't properly represent all numbers above 2^24 (https://stackoverflow.com/a/23031245/9080819)
    - `as f32` will silently overflow, instead use `into`/`try_into`
- Day 13
    - Be careful using `str.lines().tuples()` when there's an empty line being discarded. The end of the file may not be completely read unless it has the proper newlines.

## Other
My solutions from last year https://github.com/zachstence/advent-of-code-2021