// #[derive(Debug, PartialEq, Clone)]
// pub struct Day1 {
//     pub deltas: Vec<i32>,
// }

use crate::{Day, Parse, PartOne, PartTwo, test};

test!(day 1, parse:
    "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82"
    => vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]
);

impl Parse<Vec<i32>> for Day<1> {
    fn parse(input: impl AsRef<str>) -> Vec<i32> {
        input
            .as_ref()
            .lines()
            .map(|line| {
                let (turn, dist) = line.split_at(1);
                let distance: i32 = dist.parse().unwrap();
                match turn {
                    "L" => -distance,
                    "R" => distance,
                    _ => panic!("Invalid turn direction"),
                }
            })
            .collect()
    }
}

test!(day 1, part 1:
    "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82"
    => String::from("3")
);

impl PartOne<Vec<i32>> for Day<1> {
    fn part_1(data: &Vec<i32>) -> String {
        let mut position = 50;
        let mut zeros = 0;
        for delta in data {
            position += delta;
            // equivalent to position.rem_euclid(100) because we're only testing for zero.
            if position % 100 == 0 {
                zeros += 1;
            }
        }
        zeros.to_string()
    }
}

test!(day 1, part 2:
    "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82"
    => String::from("6")
);

impl PartTwo<Vec<i32>> for Day<1> {
    fn part_2(data: &Vec<i32>) -> String {
        let mut position = 50;
        let mut zeros = 0;
        for delta in data {
            let guaranteed_clicks = delta / 100; // rounds towards zero
            zeros += guaranteed_clicks.abs();

            let rest = delta % 100;
            let new_position = position + rest;

            if position != 0 && !(1..100).contains(&new_position) {
                zeros += 1;
            }
            position = new_position.rem_euclid(100);
        }
        zeros.to_string()
    }
}
