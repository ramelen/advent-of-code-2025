use crate::{Day, FromInput, Solve};

impl FromInput for Vec<i32> {
    fn from_input(input: impl AsRef<str>) -> Vec<i32> {
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

impl Solve for Day<1> {
    type PartOneData = Vec<i32>;
    type PartTwoData = Vec<i32>;

    fn part_1(deltas: &Self::PartOneData) -> String {
        let mut position = 50;
        let mut zeros = 0;
        for delta in deltas {
            position += delta;
            // equivalent to position.rem_euclid(100) because we're only testing for zero.
            if position % 100 == 0 {
                zeros += 1;
            }
        }
        zeros.to_string()
    }

    fn part_2(deltas: &Self::PartTwoData) -> String {
        let mut position = 50;
        let mut zeros = 0;
        for delta in deltas {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const INPUT: &'static str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    test!(day 1, parse: Vec<i32>; INPUT => vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82] );

    test!(day 1, part 1; INPUT => String::from("3"));

    test!(day 1, part 2; INPUT => String::from("6"));
}
