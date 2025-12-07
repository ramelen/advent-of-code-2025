use crate::{Day, FromInput, Solve};
use std::ops::RangeInclusive;

impl FromInput for (Vec<RangeInclusive<u64>>, Vec<u64>) {
    fn from_input(input: impl AsRef<str>) -> Self {
        let (ranges, ids) = input.as_ref().split_once("\n\n").unwrap();

        let parsed_ranges = ranges
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .map(|(start, end)| (start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()));

        let parsed_ids = ids.lines().map(|l| l.parse::<u64>().unwrap());

        (parsed_ranges.collect(), parsed_ids.collect())
    }
}

impl FromInput for ((u64, u64), Vec<(u64, u64)>) {
    fn from_input(input: impl AsRef<str>) -> Self {
        let (ranges, _) = input.as_ref().split_once("\n\n").unwrap();

        let mut parsed_ranges: Vec<(u64, u64)> = ranges
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            .collect();

        parsed_ranges.sort_by_key(|&(start, _)| start);

        (parsed_ranges[0], parsed_ranges[1..].to_vec()) // will panic if empty
    }
}

impl Solve for Day<5> {
    type PartOneData = (Vec<RangeInclusive<u64>>, Vec<u64>);
    type PartTwoData = ((u64, u64), Vec<(u64, u64)>);

    fn part_1((ranges, ids): &Self::PartOneData) -> String {
        ids.iter()
            .filter(|id| ranges.iter().any(|range| range.contains(&id)))
            .count()
            .to_string()
    }

    fn part_2(((low, high), rest): &Self::PartTwoData) -> String {
        let mut count = 0;
        let mut current_lowest = *low;
        let mut current_highest = *high;
        for &(next_low, next_high) in rest {
            if current_highest < next_low {
                count += 1 + current_highest - current_lowest;
                current_lowest = next_low;
                current_highest = next_high;
            } else {
                current_highest = current_highest.max(next_high);
            }
        }
        count += 1 + current_highest - current_lowest;
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const INPUT: &str = "\
    3-5\n\
    10-14\n\
    16-20\n\
    12-18\n\
    \n\
    1\n\
    5\n\
    8\n\
    11\n\
    17\n\
    32";

    test!(day 5, parse_part_1: (Vec<RangeInclusive<u64>>, Vec<u64>);
        INPUT => (vec![3..=5, 10..=14, 16..=20, 12..=18], vec![1, 5, 8, 11, 17, 32])
    );

    test!(day 5, parse_part_2: ((u64, u64), Vec<(u64, u64)>);
        INPUT => ((3, 5), vec![(10, 14), (12, 18), (16, 20)])
    );

    test!(day 5, part 1; INPUT => String::from("3"));

    test!(day 5, part 2; INPUT => String::from("14"));
}
