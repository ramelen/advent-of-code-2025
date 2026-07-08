# Advent of Code 2025

My solutions to all 12 days of [advent of code](https://adventofcode.com/2025). I made an effort to keep the code as close as possible to the day that I wrote it in the spirit of authenticity. To see my solutions with some more care put into them, see the `refactors` branch. To see code that has had more attention paid to it, you can also take a look at [this project](https://github.com/ramelen/tick-tock-tau).

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
Day  1 Part 1 (took    0.733 ms): 1102
Day  1 Part 2 (took    0.174 ms): 6175
Day  2 Part 1 (took   97.406 ms): 44854383294
Day  2 Part 2 (took  333.041 ms): 55647141923
Day  3 Part 1 (took    0.385 ms): 17193
Day  3 Part 2 (took    0.437 ms): 171297349921310
Day  4 Part 1 (took    0.628 ms): 1419
Day  4 Part 2 (took    5.594 ms): 8739
Day  5 Part 1 (took    0.401 ms): 848
Day  5 Part 2 (took    0.026 ms): 334714395325710
Day  6 Part 1 (took    0.406 ms): 6503327062445
Day  6 Part 2 (took    0.567 ms): 9640641878593
Day  7 Part 1 (took    0.211 ms): 1555
Day  7 Part 2 (took    0.137 ms): 12895232295789
Day  8 Part 1 (took   40.104 ms): 29406
Day  8 Part 2 (took   40.010 ms): 7499461416
Day  9 Part 1 (took    0.384 ms): 4771508457
Day  9 Part 2 (took   55.958 ms): 1539809693
Day 11 Part 1 (took    0.443 ms): 613
Day 11 Part 2 (took    0.907 ms): 372918445876116
Day 12 Part 1 (took    0.456 ms): 526
Day 12 Part 2 (took 327080.067 ms): 526
```

To change the days that are run or their inputs, edit `src/main.rs` in your favorite text editor.
