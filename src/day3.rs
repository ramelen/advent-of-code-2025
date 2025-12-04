use crate::{Day, Parse, PartOne, PartTwo, test};

#[cfg(test)]
const INPUT: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

test!(day 3, parse:
    INPUT => vec![
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
        vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
        vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
        vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
    ]
);

test!(day 3, part 1: INPUT => String::from("357"));

test!(day 3, part 2: INPUT => String::from("3121910778619"));

impl Parse<Vec<Vec<u64>>> for Day<3> {
    fn parse(input: impl AsRef<str>) -> Vec<Vec<u64>> {
        input
            .as_ref()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap().into()).collect())
            .collect()
    }
}

impl PartOne<Vec<Vec<u64>>> for Day<3> {
    fn part_1(data: &Vec<Vec<u64>>) -> String {
        let mut total_joltage = 0;
        for bank in data {
            let tens_digit = bank.iter().rev().skip(1).max().unwrap();
            let tens_index = bank.iter().position(|j| j == tens_digit).unwrap();
            let ones_digit = bank.iter().skip(tens_index + 1).max().unwrap();
            total_joltage += 10 * tens_digit + ones_digit;
        }
        total_joltage.to_string()
    }
}

impl PartTwo<Vec<Vec<u64>>> for Day<3> {
    fn part_2(data: &Vec<Vec<u64>>) -> String {
        let mut total_joltage: u64 = 0;
        for bank in data {
            let mut remaining_bank = bank.clone();
            for digit in (0..12).rev() {
                let digit_value: &u64 = remaining_bank.iter().rev().skip(digit).max().unwrap();

                let digit_index = remaining_bank
                    .iter()
                    .position(|j| j == digit_value)
                    .unwrap();

                total_joltage += 10u64.pow(digit.try_into().unwrap()) * digit_value;

                remaining_bank = remaining_bank
                    .iter()
                    .skip(digit_index + 1)
                    .cloned()
                    .collect();
            }
        }
        total_joltage.to_string()
    }
}
