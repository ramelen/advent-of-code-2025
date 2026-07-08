use crate::util::*;
use std::collections::HashMap;

pub use helpers::parse_id_range;
pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(2, Part::One, &parse, &solve_part_one),
    &Solution::new_variant(2, Part::One, "fancy", &parse, &solve_part_one_fancy),
    &Solution::new(2, Part::Two, &parse, &solve_part_two),
    &Solution::new_variant(2, Part::Two, "memoized", &parse, &solve_part_two_memoized),
    &Solution::new_variant(2, Part::Two, "fancy", &parse, &solve_part_two_fancy),
];

// parse input into a list of id ranges
fn parse(input: &str) -> Result<Vec<(u64, u64)>, String> {
    // ranges are comma separated without any whitespace
    input.split(',').map(helpers::parse_id_range).collect()
}

// count ids where the first half of the digits are equal to the second half (by filtering)
fn solve_part_one(ids: Vec<(u64, u64)>) -> Result<u64, String> {
    Ok(ids
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        // filter for ids where the first half equals the second half
        .filter(|&id| {
            // convert to string for easy digit comparison
            let id_str = id.to_string();
            let len = id_str.len();

            // checking if the length is even is techically unnecessary
            len % 2 == 0 && id_str[..len / 2] == id_str[len / 2..]
        })
        .sum())
}

// count ids where the first half of the digits are equal to the second half (by directly skipping to next such id repeatedly)
fn solve_part_one_fancy(ranges: Vec<(u64, u64)>) -> Result<u64, String> {
    // sum of all invalid ids found so far
    let mut id_sum = 0;

    for (start, end) in ranges {
        let mut id = start.max(1); // unsure the id is positive for when we get the length
        while id <= end {
            // calculate number of digits without converting to string
            let len = id.ilog10() + 1;

            // check if the id is a repeater with two repeats
            if len % 2 == 0 && helpers::is_repeated_length(id, len / 2) {
                id_sum += id;
            }

            // calculate the closest id with the special property and update the current id
            id = helpers::next_repeater_of_length(id, len.div_ceil(2));
        }
    }

    Ok(id_sum)
}

// count ids that are some sequence of digits repeated at least twice (by filtering)
fn solve_part_two(ids: Vec<(u64, u64)>) -> Result<u64, String> {
    Ok(ids
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .filter(|&id| {
            // convert to string for easy digit comparison
            let id_str = id.to_string();
            let len = id_str.len();

            // iterate over all possible factors and check for repeats
            (2..=len)
                .filter(|factor| len % factor == 0)
                .any(|factor| id_str[..len / factor].repeat(factor) == id_str)
        })
        .sum())
}

// count ids that are some sequence of digits repeated at least twice (by filtering, sped up using memoization)
fn solve_part_two_memoized(ids: Vec<(u64, u64)>) -> Result<u64, String> {
    let mut factor_memo: HashMap<u32, Vec<u32>> = HashMap::new();

    Ok(ids
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .filter(|&id| {
            // calculate number of digits without convering to string
            let len = id.ilog10() + 1;

            // memoize the list of factors for this length
            let factors = factor_memo.entry(len).or_insert_with(|| {
                // iterate over all possible factors
                (1..len)
                    .filter(|&factor| len % factor == 0)
                    .collect::<Vec<u32>>()
            });

            // check if factor is decomposable into repeating substrings of digits
            factors
                .iter()
                .any(|&factor| helpers::is_repeated_length(id, factor))
        })
        .sum())
}

// count ids that are some sequence of digits repeated at least twice (by directly skipping to the next such id repeatedly)
fn solve_part_two_fancy(ids: Vec<(u64, u64)>) -> Result<u64, String> {
    let mut id_sum = 0;

    for (start, end) in ids {
        // set the current id to the smallest repeater greater than or equal to it
        let mut id = start.max(1); // ensure the id is positive for when we get the length

        // advance the current id if it isn't a repeater
        let len = id.ilog10() + 1;
        if !(1..len).any(|factor| len % factor == 0 && helpers::is_repeated_length(id, factor)) {
            id = helpers::next_repeater(id);
        }

        while id <= end {
            id_sum += id; // increment the total by the current id
            id = helpers::next_repeater(id); // advance the id
        }
    }

    Ok(id_sum)
}

mod helpers {
    use super::*;

    // parse a line into a range of ids
    pub fn parse_id_range(range_str: &str) -> Result<(u64, u64), String> {
        // start and end are hyphen separated
        let (start_str, end_str) = range_str
            .split_once('-')
            .ok_or_else(|| format!("id range not in [start]-[end] format: '{range_str}')"))?;

        Ok((parse_int(start_str)?, parse_int(end_str)?))
    }

    // tests if an id is a repeater of a given repeat length, where a repeater is some number of repetitions of a certain number of digits (for example 121212 is a repeater of length three)
    pub fn is_repeated_length(mut id: u64, repeat_len: u32) -> bool {
        let modulus = 10u64.pow(repeat_len);

        // the digits that may or may not be repeated
        let repeated_part = id % modulus;

        // repeatedly check that the least significant digits are equal to the suffix and then remove them
        while id != 0 && id % modulus == repeated_part {
            id /= modulus;
        }

        // the number is repeated if there is nothing left over afterwards
        id == 0
    }

    // calculates the smallest repeater greater than the given id (or panics if `id` is zero).
    pub fn next_repeater(id: u64) -> u64 {
        // calculate number of digits without convering to string
        let len = id.ilog10() + 1;

        // calculate the closest id with the special property for any length
        (1..=len)
            .map(|factor| next_repeater_of_length(id, factor))
            .min()
            .expect("all ids have positive length")
    }

    // finds the smallest repeater (with a given length of repeated digits) larger than the given id
    pub fn next_repeater_of_length(mut id: u64, num_digits: u32) -> u64 {
        // increment the id if it is one less than the next power of ten, e.g. 9, 99, 999, since these are always repeaters and they cause an edge case that results in the incorrect number
        if id == 0 || id == 10u64.pow(id.ilog10() + 1) - 1 {
            id += 1;
        }

        // the number of digits in `id`
        let len = id.ilog10() + 1;

        // the number of times the subsequence must be repeated, which must be at least two
        let num_repeats = len.div_ceil(num_digits).max(2);

        // if the repeated part of id has leading zeros increase it to the next power of ten, e.g. 05 43 21 -> 10 00 00
        id = id.max(10u64.pow(num_digits * num_repeats - 1));

        // repeatedly grow the delta to get the difference between repeaters, e.g. 1 -> 101 -> 10101
        let delta = 0.mutate(|delta| {
            for _ in 0..num_repeats {
                *delta = *delta * 10u64.pow(num_digits) + 1; // e.g. 101 -> 101 * 100 + 1 = 10101
            }
        });

        // next repeater is id + (distance between repeaters) - (distance to last repeater)
        id + (delta) - (id % delta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        11-22,\
        95-115,\
        998-1012,\
        1188511880-1188511890,\
        222220-222224,\
        1698522-1698528,\
        446443-446449,\
        38593856-38593862,\
        565653-565659,\
        824824821-824824827,\
        2121212118-2121212124";

    #[test]
    fn test_parse() {
        let expected = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];

        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(1227775554), parse(INPUT).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_one_fancy() {
        assert_eq!(Ok(1227775554), parse(INPUT).and_then(solve_part_one_fancy));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(4174379265), parse(INPUT).and_then(solve_part_two));
    }

    #[test]
    fn test_solve_part_two_memoized() {
        assert_eq!(
            Ok(4174379265),
            parse(INPUT).and_then(solve_part_two_memoized)
        );
    }

    #[test]
    fn test_solve_part_two_fancy() {
        assert_eq!(Ok(4174379265), parse(INPUT).and_then(solve_part_two_fancy));
    }

    #[test]
    fn test_next_repeater() {
        let cases = [
            (1, 11),
            (2, 11),
            (3, 11),
            (9, 11),
            (10, 11),
            (11, 22),
            (21, 22),
            (22, 33),
            (23, 33),
            (98, 99),
            (99, 111),
            (100, 111),
            (9999, 11111),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, helpers::next_repeater(input));
        }
    }
}
