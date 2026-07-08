# Advent of Code 2025

My solutions to all 12 days of [advent of code](https://adventofcode.com/2025), refactored after the fact with brief comments and some new and faster algorithms.

## Building from source

The best way to run my code is to clone the repository and build it yourself. In a terminal:

```sh
git clone https://github.com/ramelen/advent-of-code-2025.git
cd advent-of-code-2025
```

Now you can run the repo if you have [rust](https://rust-lang.org/tools/install/) installed:

```sh
cargo run --release
```

This will print out the answers for each day and how long they took, which for my machine looks like:

```
Day 01, part one (took      0.478 ms): 1102
Day 01, part two (took      0.267 ms): 6175
Day 02, part one (took    111.781 ms): 44854383294
Day 02, part one (took      0.044 ms): 44854383294 (variant 'fancy')
Day 02, part two (took    429.856 ms): 55647141923
Day 02, part two (took     74.167 ms): 55647141923 (variant 'memoized')
Day 02, part two (took      0.459 ms): 55647141923 (variant 'fancy')
Day 03, part one (took      0.322 ms): 17193
Day 03, part two (took      0.243 ms): 171297349921310
Day 04, part one (took      0.390 ms): 1419
Day 04, part two (took      5.777 ms): 8739
Day 05, part one (took      0.195 ms): 848
Day 05, part two (took      0.226 ms): 334714395325710
Day 06, part one (took      0.350 ms): 6503327062445
Day 06, part two (took      0.687 ms): 9640641878593
Day 07, part one (took      0.280 ms): 1555
Day 07, part two (took      0.174 ms): 12895232295789
Day 08, part one (took     40.986 ms): 29406
Day 08, part two (took     44.750 ms): 7499461416
Day 09, part one (took      0.291 ms): 4771508457
Day 09, part two (took     40.502 ms): 1539809693
Day 09, part two (took      6.034 ms): 1539809693 (variant 'fancy')
Day 10, part one (took     26.872 ms): 409
Day 10, part two                     : skipped due to long runtime (variant 'long')
Day 10, part two (took  17756.253 ms): 15489 (variant 'fancy')
Day 11, part one (took      1.051 ms): 613
Day 11, part two (took      0.900 ms): 372918445876116
Day 12, part one (took      0.287 ms): 526
```

To change the days that are run or their inputs, edit `src/main.rs` in your favorite text editor.
