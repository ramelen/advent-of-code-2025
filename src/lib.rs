pub mod day1;
pub mod day2;
pub mod day3;

pub struct Day<const N: usize>;

pub trait Parse<T> {
    fn parse(input: impl AsRef<str>) -> T;
}

pub trait PartOne<T>: Parse<T> {
    fn part_1(data: &T) -> String;
}

pub trait PartTwo<T>: Parse<T> {
    fn part_2(data: &T) -> String;
}

#[macro_export]
macro_rules! solve {
    (day $day:expr, part 1: $input:expr) => {{
        println!(
            "Day {} Part 1: {}",
            $day,
            Day::<$day>::part_1(&Day::<$day>::parse($input))
        );
    }};
    (day $day:expr, part 2: $input:expr) => {{
        println!(
            "Day {} Part 2: {}",
            $day,
            Day::<$day>::part_2(&Day::<$day>::parse($input))
        );
    }};
}

#[macro_export]
macro_rules! test {
    (day $day:expr, parse: $input:expr => $output:expr) => {
        #[cfg(test)]
        #[test]
        fn test_parse() {
            let input = $input;
            let parsed = Day::<$day>::parse(&input);
            assert_eq!(parsed, $output);
        }
    };
    (day $day:expr, part 1: $input:expr => $output:expr) => {
        #[cfg(test)]
        #[test]
        fn test_part_1() {
            let input = $input;
            let parsed = Day::<$day>::parse(&input);
            let solution = Day::<$day>::part_1(&parsed);
            assert_eq!(solution, $output);
        }
    };
    (day $day:expr, part 2: $input:expr => $output:expr) => {
        #[cfg(test)]
        #[test]
        fn test_part_2() {
            let input = $input;
            let parsed = Day::<$day>::parse(&input);
            let solution = Day::<$day>::part_2(&parsed);
            assert_eq!(solution, $output);
        }
    };
}
