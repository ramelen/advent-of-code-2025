use crate::{Day, FromInput, Solve};
use std::collections::HashMap;

impl FromInput for HashMap<String, Vec<String>> {
    fn from_input(input: impl AsRef<str>) -> Self {
        let mut map = HashMap::new();

        for line in input.as_ref().lines() {
            let (first, rest) = line.split_once(':').unwrap();
            let key = first.to_string();
            let values = rest
                .split_ascii_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<String>>();
            map.insert(key, values);
        }

        map
    }
}

fn num_paths(
    memo: &mut HashMap<String, usize>,
    map: &HashMap<String, Vec<String>>,
    current_device: &str,
    end: &str,
) -> usize {
    memo.get(current_device).copied().unwrap_or_else(|| {
        let num_paths = if current_device == end {
            1
        } else if let Some(children) = map.get(current_device) {
            children
                .iter()
                .map(|child| num_paths(memo, map, child, end))
                .sum::<usize>()
        } else {
            0
        };
        memo.insert(current_device.to_owned(), num_paths);
        num_paths
    })
}

impl Solve for Day<11> {
    type PartOneData = HashMap<String, Vec<String>>;
    type PartTwoData = HashMap<String, Vec<String>>;

    fn part_1(map: &Self::PartOneData) -> String {
        num_paths(&mut HashMap::new(), map, "you", "out").to_string()
    }

    fn part_2(map: &Self::PartOneData) -> String {
        (num_paths(&mut HashMap::new(), map, "svr", "dac")
            * num_paths(&mut HashMap::new(), map, "dac", "fft")
            * num_paths(&mut HashMap::new(), map, "fft", "out")
            + num_paths(&mut HashMap::new(), map, "svr", "fft")
                * num_paths(&mut HashMap::new(), map, "fft", "dac")
                * num_paths(&mut HashMap::new(), map, "dac", "out"))
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const INPUT_1: &str = "\
        aaa: you hhh\n\
        you: bbb ccc\n\
        bbb: ddd eee\n\
        ccc: ddd eee fff\n\
        ddd: ggg\n\
        eee: out\n\
        fff: out\n\
        ggg: out\n\
        hhh: ccc fff iii\n\
        iii: out";

    const INPUT_2: &str = "\
        svr: aaa bbb\n\
        aaa: fft\n\
        fft: ccc\n\
        bbb: tty\n\
        tty: ccc\n\
        ccc: ddd eee\n\
        ddd: hub\n\
        hub: fff\n\
        eee: dac\n\
        dac: fff\n\
        fff: ggg hhh\n\
        ggg: out\n\
        hhh: out";

    test!(day 11, parse: HashMap<String, Vec<String>>;
        INPUT_1 => HashMap::from([
            ("aaa".into(), vec!["you".into(), "hhh".into()]),
            ("you".into(), vec!["bbb".into(), "ccc".into()]),
            ("bbb".into(), vec!["ddd".into(), "eee".into()]),
            ("ccc".into(), vec!["ddd".into(), "eee".into(), "fff".into()]),
            ("ddd".into(), vec!["ggg".into()]),
            ("eee".into(), vec!["out".into()]),
            ("eee".into(), vec!["out".into()]),
            ("fff".into(), vec!["out".into()]),
            ("ggg".into(), vec!["out".into()]),
            ("hhh".into(), vec!["ccc".into(), "fff".into(), "iii".into()]),
            ("iii".into(), vec!["out".into()]),
        ])
    );

    test!(day 11, part 1; INPUT_1 => String::from("5"));

    test!(day 11, part 2; INPUT_2 => String::from("2"));
}
