use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(3, Part::One, &parse, &solve::<2>),
    &Solution::new(3, Part::Two, &parse, &solve::<12>),
];

// parse input into a list of banks
fn parse(input: &str) -> Result<Vec<Vec<u64>>, String> {
    input.lines().map(helpers::parse_bank).collect()
}

// calculate the maximum joltage for each bank, which is the highest N-digit number formed by concatenating N of its batteries' joltage ratings together
fn solve<const N: usize>(banks: Vec<Vec<u64>>) -> Result<u64, String> {
    banks
        .into_iter()
        .flat_map(|mut bank| {
            // reverse the bank so that more significant digits come first
            bank.reverse();
            // maximize the most significant digit, then the second most significant, etc.
            (0..N).rev().scan(bank, |reversed_bank, i| {
                Some(helpers::max_ith_digit::<N>(reversed_bank, i))
            })
        })
        .sum()
}

mod helpers {
    // parse a base ten digit into a battery's 'joltage rating'
    pub fn parse_digit(digit_char: char) -> Result<u64, String> {
        digit_char
            .to_digit(10)
            .ok_or_else(|| format!("no such digit '{digit_char}'"))
            .map(u64::from)
    }

    // parse a string of digits (joltage ratings) into a bank
    pub fn parse_bank(bank_str: &str) -> Result<Vec<u64>, String> {
        bank_str.chars().map(parse_digit).collect()
    }

    // find the battery with the highest joltage rating and reduce the size of the bank
    pub fn max_ith_digit<const N: usize>(bank: &mut Vec<u64>, i: usize) -> Result<u64, String> {
        // get the index and joltage of the battery with the highest rating
        let (index, rating) = bank
            .iter()
            .skip(i) // the skipped elements will be needed for less significant digits
            .max() // maximize the rating
            .map(|element| (bank.element_offset(element).unwrap(), *element)) // get the battery's index as well
            .ok_or_else(|| format!("there must be at least {N} joltage ratings in each cell"))?;

        // truncate the reversed bank to hold only the remaining batteries
        bank.truncate(index);

        // the joltage rating multiplied by its place value
        Ok(rating * 10u64.pow(i as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111";

    #[test]
    fn test_parse() {
        let expected = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];

        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(357), parse(INPUT).and_then(solve::<2>));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(3121910778619), parse(INPUT).and_then(solve::<12>));
    }
}
