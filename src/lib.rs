pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub struct Day<const N: usize>;

pub trait FromInput {
    fn from_input(input: impl AsRef<str>) -> Self;
}

pub trait ToData<T> {
    fn to_data(self) -> T;
}

impl<I: AsRef<str>, T: FromInput> ToData<T> for I {
    fn to_data(self) -> T {
        T::from_input(self)
    }
}

pub trait Solve {
    type PartOneData: FromInput;
    type PartTwoData: FromInput;

    fn part_1(_input: &Self::PartOneData) -> String {
        String::from("This part hasn't been implemented :o")
    }
    fn part_2(_input: &Self::PartTwoData) -> String {
        String::from("This part hasn't been implemented :o")
    }
}

pub fn solve<const N: usize>(input: impl AsRef<str> + Copy)
where
    Day<N>: Solve,
{
    use std::time::Instant;

    let part_1_start = Instant::now();
    let solution_1 = Day::<N>::part_1(&input.to_data());
    println!(
        "Day {N:2} Part 1 (took {:8.3} ms): {solution_1}",
        1000.0 * part_1_start.elapsed().as_secs_f64()
    );

    let part_2_start = Instant::now();
    let solution_2 = Day::<N>::part_2(&input.to_data());
    println!(
        "Day {N:2} Part 2 (took {:8.3} ms): {solution_2}",
        1000.0 * part_2_start.elapsed().as_secs_f64()
    );
}

#[macro_export]
macro_rules! test {
    (day $day:expr, part 1; $input:expr => $output:expr) => {
        #[test]
        fn part_1() {
            use crate::ToData;
            assert_eq!(Day::<$day>::part_1(&($input).to_data()), $output);
        }
    };
    (day $day:expr, part 2; $input:expr => $output:expr) => {
        #[test]
        fn part_2() {
            use crate::ToData;
            assert_eq!(Day::<$day>::part_2(&($input).to_data()), $output);
        }
    };
    (day $day:expr, $name:ident: $data:ty; $input:expr => $parsed:expr) => {
        #[test]
        fn $name() {
            use crate::ToData;
            let parsed: $data = ($input).to_data();
            assert_eq!(parsed, ($parsed));
        }
    };
}
