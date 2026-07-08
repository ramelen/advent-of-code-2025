use crate::util::*;
use std::collections::HashMap;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(11, Part::One, &parse, &solve_part_one),
    &Solution::new(11, Part::Two, &parse, &solve_part_two),
];

// parse input into map of nodes and their outputs
fn parse(input: &str) -> Result<HashMap<String, Vec<String>>, String> {
    input.lines().map(helpers::parse_entry).collect()
}

// find the number of paths between the 'you' device and the 'out' device
fn solve_part_one(map: HashMap<String, Vec<String>>) -> Result<u64, String> {
    Ok(helpers::num_paths(&map, "you", "out"))
}

// find the number of paths between the 'svr' device and the 'out' device, taking advantage of the fact that it must pass through both 'svr' or 'fft' to get to 'out'
fn solve_part_two(map: HashMap<String, Vec<String>>) -> Result<u64, String> {
    Ok(helpers::num_paths(&map, "svr", "dac")
        * helpers::num_paths(&map, "dac", "fft")
        * helpers::num_paths(&map, "fft", "out")
        + helpers::num_paths(&map, "svr", "fft")
            * helpers::num_paths(&map, "fft", "dac")
            * helpers::num_paths(&map, "dac", "out"))
}

mod helpers {
    use super::*;

    // parse a line into an entry mapping from a node to its destinations
    pub fn parse_entry(line: &str) -> Result<(String, Vec<String>), String> {
        // split line by colon
        let (first, rest) = line
            .split_once(':')
            .ok_or_else(|| format!("line doesn't follow [node]: [outputs] format: '{line}'"))?;

        // parse node name
        let key = first.to_string();

        // parse destination names
        let values = rest
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        Ok((key, values))
    }

    // find the number of paths between `current_device` and `end`
    pub fn num_paths(map: &HashMap<String, Vec<String>>, current_device: &str, end: &str) -> u64 {
        fn inner(
            memo: &mut HashMap<String, u64>,
            map: &HashMap<String, Vec<String>>,
            current: &str,
            end: &str,
        ) -> u64 {
            memo.get(current).copied().unwrap_or_else(|| {
                let num_paths = if current == end {
                    1 // base case
                } else {
                    // search children and sum
                    map.get(current)
                        .unwrap_or(&Vec::new()) // silently ignore the case where a node has no children
                        .iter()
                        .map(|next| inner(memo, map, next, end))
                        .sum()
                };
                memo.insert(current.to_owned(), num_paths);
                num_paths
            })
        }

        inner(&mut HashMap::new(), map, current_device, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_parse() {
        let expected = HashMap::from([
            ("aaa".into(), vec!["you".into(), "hhh".into()]),
            ("you".into(), vec!["bbb".into(), "ccc".into()]),
            ("bbb".into(), vec!["ddd".into(), "eee".into()]),
            ("ccc".into(), vec!["ddd".into(), "eee".into(), "fff".into()]),
            ("ddd".into(), vec!["ggg".into()]),
            ("eee".into(), vec!["out".into()]),
            ("fff".into(), vec!["out".into()]),
            ("ggg".into(), vec!["out".into()]),
            ("hhh".into(), vec!["ccc".into(), "fff".into(), "iii".into()]),
            ("iii".into(), vec!["out".into()]),
        ]);
        assert_eq!(Ok(expected), parse(INPUT_1));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(5), parse(INPUT_1).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(2), parse(INPUT_2).and_then(solve_part_two));
    }
}
