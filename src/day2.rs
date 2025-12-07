use crate::{Day, FromInput, Solve};

impl FromInput for Vec<u64> {
    fn from_input(input: impl AsRef<str>) -> Self {
        input
            .as_ref()
            .split(',')
            .flat_map(|range| {
                let (start, end) = range.split_once('-').unwrap();
                let start_num: u64 = start.parse().unwrap();
                let end_num: u64 = end.parse().unwrap();
                start_num..=end_num
            })
            .collect()
    }
}

impl Solve for Day<2> {
    type PartOneData = Vec<u64>;
    type PartTwoData = Vec<u64>;

    fn part_1(ids: &Self::PartOneData) -> String {
        ids.iter()
            .copied() // does this do anything?
            .filter(|num| {
                let num_str = num.to_string();
                let len = num_str.len();
                // checking if the length is even is techically unnecessary
                len % 2 == 0 && num_str[..len / 2] == num_str[len / 2..]
            })
            .sum::<u64>()
            .to_string()
    }

    fn part_2(ids: &Self::PartTwoData) -> String {
        let mut id_sum = 0;
        for num in ids {
            // if number is two numbers repeated twice, add it to the total
            let num_str = num.to_string();
            let len = num_str.len();
            for divisions in 2..=len {
                if len % divisions != 0 {
                    continue;
                }
                let sub_string = &num_str[..len / divisions];
                if sub_string.repeat(divisions) == num_str {
                    id_sum += num;
                    break;
                }
            }
        }
        id_sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const INPUT: &'static str = "\
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

    test!(day 2, parse: Vec<u64>;
        INPUT => vec![
            11..=22,
            95..=115,
            998..=1012,
            1188511880..=1188511890,
            222220..=222224,
            1698522..=1698528,
            446443..=446449,
            38593856..=38593862,
            565653..=565659,
            824824821..=824824827,
            2121212118..=2121212124
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    );

    test!(day 2, part 1; INPUT => String::from("1227775554"));

    test!(day 2, part 2; INPUT => String::from("4174379265"));
}
