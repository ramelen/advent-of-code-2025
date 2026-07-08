#![expect(clippy::type_complexity, reason = "types are inlined for consistency")]

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod util {
    pub use super::{Part, Solution, Solver};
    use std::{num::ParseIntError, str::FromStr};

    // parse a string into an integer with a simple error message
    #[inline]
    pub fn parse_int<T: FromStr<Err = ParseIntError>>(val: &str) -> Result<T, String> {
        val.parse::<T>()
            .map_err(|_| format!("couldn't parse number '{val}'"))
    }

    // get the length of the sublists from the slice, or return an error if there is none
    pub fn width<T>(rect: &[Vec<T>]) -> Result<usize, String> {
        let [first, rest @ ..] = rect else {
            return Err("input must contain at least one row".into());
        };

        if rest.iter().all(|row| row.len() == first.len()) {
            Ok(first.len())
        } else {
            Err("input must have same number of items in each row".into())
        }
    }

    pub trait Mutate: Sized {
        // Mutate a value using the given function and return the mutated value. Useful when you only want to mutate something during its construction or if you want to mutate an intermediate value in a much longer method chain.
        #[inline]
        fn mutate<T>(mut self, f: impl FnOnce(&mut Self) -> T) -> Self {
            f(&mut self);
            self
        }
    }

    impl<T> Mutate for T {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Part {
    One,
    Two,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => f.write_str("one"),
            Self::Two => f.write_str("two"),
        }
    }
}

pub struct Solution<D: 'static> {
    day: u64,
    part: Part,
    variant: &'static str,
    parse: &'static dyn Fn(&str) -> Result<D, String>,
    solve: &'static dyn Fn(D) -> Result<u64, String>,
}

impl<D: 'static> Solution<D> {
    const fn new(
        day: u64,
        part: Part,
        parse: &'static dyn Fn(&str) -> Result<D, String>,
        solve: &'static dyn Fn(D) -> Result<u64, String>,
    ) -> Self {
        {
            Self {
                day,
                part,
                variant: "",
                parse,
                solve,
            }
        }
    }

    const fn new_variant(
        day: u64,
        part: Part,
        variant: &'static str,
        parser: &'static dyn Fn(&str) -> Result<D, String>,
        solver: &'static dyn Fn(D) -> Result<u64, String>,
    ) -> Self {
        {
            Self {
                day,
                part,
                variant,
                parse: parser,
                solve: solver,
            }
        }
    }
}

pub trait Solver {
    fn day(&self) -> u64;
    fn part(&self) -> Part;
    fn variant(&self) -> &str;
    fn name(&self) -> String;
    fn solve(&self, input: &str) -> Result<u64, String>;
}

impl<D: 'static> Solver for Solution<D> {
    fn day(&self) -> u64 {
        self.day
    }

    fn part(&self) -> Part {
        self.part
    }

    fn variant(&self) -> &str {
        self.variant
    }

    fn name(&self) -> String {
        format!(
            "Day {day:02}, part {part}",
            day = self.day(),
            part = self.part(),
        )
    }

    fn solve(&self, input: &str) -> Result<u64, String> {
        (self.parse)(input).and_then(self.solve)
    }
}

fn main() {
    let solutions: Vec<&dyn Solver> = [
        day1::SOLUTIONS,
        day2::SOLUTIONS,
        day3::SOLUTIONS,
        day4::SOLUTIONS,
        day5::SOLUTIONS,
        day6::SOLUTIONS,
        day7::SOLUTIONS,
        day8::SOLUTIONS,
        day9::SOLUTIONS,
        day10::SOLUTIONS,
        day11::SOLUTIONS,
        day12::SOLUTIONS,
    ]
    .concat();

    let inputs = [
        include_str!("day1.txt"),
        include_str!("day2.txt"),
        include_str!("day3.txt"),
        include_str!("day4.txt"),
        include_str!("day5.txt"),
        include_str!("day6.txt"),
        include_str!("day7.txt"),
        include_str!("day8.txt"),
        include_str!("day9.txt"),
        include_str!("day10.txt"),
        include_str!("day11.txt"),
        include_str!("day12.txt"),
    ];

    for solution in solutions {
        let name = solution.name();
        if std::env::var("RUN_LONG_SOLVES") == Err(std::env::VarError::NotPresent)
            && solution.variant() == "long"
        {
            println!("{name}                     : skipped due to long runtime (variant 'long')");
            continue;
        }

        let day_index = (solution.day() - 1) as usize;
        let input = &inputs[day_index];

        let solution_start = std::time::Instant::now();
        let result = solution.solve(input.as_ref());
        let solution_time = solution_start.elapsed();

        let millis = 1000.0 * solution_time.as_secs_f64();
        let variant_str = if solution.variant().is_empty() {
            String::new()
        } else {
            format!(" (variant '{}')", solution.variant())
        };

        match result {
            Ok(answer) => println!("{name} (took {millis:10.3} ms): {answer}{variant_str}"),
            Err(err) => println!("{name} (took {millis:10.3} ms): Error: {err}{variant_str}"),
        }
    }
}
