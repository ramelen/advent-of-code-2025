use crate::{Day, Parse, PartOne, PartTwo, test};

test!(day 2, parse:
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    => vec![
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

test!(day 2, part 1:
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    => String::from("1227775554")
);

test!(day 2, part 2:
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    => String::from("4174379265")
);

impl Parse<Vec<u64>> for Day<2> {
    fn parse(input: impl AsRef<str>) -> Vec<u64> {
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

impl PartOne<Vec<u64>> for Day<2> {
    fn part_1(data: &Vec<u64>) -> String {
        data.iter()
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
}

impl PartTwo<Vec<u64>> for Day<2> {
    fn part_2(data: &Vec<u64>) -> String {
        let mut id_sum = 0;
        for num in data {
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
