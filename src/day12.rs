use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[&Solution::new(12, Part::One, &parse, &solve)];

// parse input into list of presents and list of regions
fn parse(input: &str) -> Result<(Vec<u64>, Vec<((u64, u64), Vec<u64>)>), String> {
    // split input by blank lines
    let blocks = input.split("\n\n").collect::<Vec<&str>>();

    // regions are in last block, presents are in the rest
    let [present_strs @ .., region_strs] = blocks.as_slice() else {
        return Err("input must contain presents and regions seperated by a blank line".into());
    };

    // parse presents and generate their rotated and flipped versions
    let presents = present_strs
        .iter()
        .copied()
        .map(helpers::parse_present)
        .collect::<Result<Vec<u64>, String>>()?;

    // parse regions
    let regions = region_strs
        .lines()
        .map(helpers::parse_region)
        .collect::<Result<Vec<((u64, u64), Vec<u64>)>, String>>()?;

    Ok((presents, regions))
}

// find the number of regions that can hold all of their presents (using a depth-first search)
fn solve((presents, regions): (Vec<u64>, Vec<((u64, u64), Vec<u64>)>)) -> Result<u64, String> {
    regions
        .into_iter()
        .map(|((width, height), present_counts)| {
            // total number of tiles for each type of present
            let tiles_to_fill = present_counts
                .iter()
                .zip(&presents)
                .map(|(count, num_filled)| count * num_filled)
                .sum();

            if (width / 3) * (height / 3) >= present_counts.iter().sum() {
                // definitely possible if each present can sit in its own 3x3 square
                Ok(1)
            } else if width * height < tiles_to_fill {
                // definitely impossible if the total number of tiles to place is less than the region's area
                Ok(0)
            } else {
                Err(format!(
                    "region fillability not easily determined: {width}x{height}: {present_counts:?}"
                ))
            }
        })
        .sum()
}

mod helpers {
    use super::*;

    // parse input into the number of tiles a present takes up
    pub fn parse_present(present_str: &str) -> Result<u64, String> {
        // parse into grid of tiles
        present_str
            .lines()
            .skip(1) // first line is identifier and is silently ignored
            .flat_map(|line| line.chars())
            .map(|char| match char {
                '.' => Ok(0),
                '#' => Ok(1),
                char => Err(format!("no such char '{char}'")),
            })
            .sum()
    }

    // parse line into a `Region`
    pub fn parse_region(region_str: &str) -> Result<((u64, u64), Vec<u64>), String> {
        // split size and present counts by ':'
        let (size, rest) = region_str
            .split_once(": ")
            .ok_or_else(|| format!("region not in '[size]: [counts]' format: '{region_str}'"))?;

        // parse list of numbers into present counts
        let present_counts = rest
            .split_ascii_whitespace()
            .map(parse_int::<u64>)
            .collect::<Result<Vec<u64>, String>>()?;

        // split region size into width and height
        let (width, height) = size
            .split_once('x')
            .ok_or_else(|| format!("size not in '[width]x[height]' format: '{size}'"))?;

        Ok(((parse_int(width)?, parse_int(height)?), present_counts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        0:\n###\n##.\n##.\n\n\
        1:\n###\n##.\n.##\n\n\
        2:\n.##\n###\n##.\n\n\
        3:\n##.\n###\n##.\n\n\
        4:\n###\n#..\n###\n\n\
        5:\n###\n.#.\n###\n\n\
        4x4: 0 0 0 0 2 0\n\
        12x5: 1 0 1 0 2 2\n\
        12x5: 1 0 1 0 3 2";

    #[test]
    fn test_parse() {
        let expected = (
            vec![7, 7, 7, 7, 7, 7],
            vec![
                ((4, 4), vec![0, 0, 0, 0, 2, 0]),
                ((12, 5), vec![1, 0, 1, 0, 2, 2]),
                ((12, 5), vec![1, 0, 1, 0, 3, 2]),
            ],
        );
        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_rigorous() {
        assert_eq!(Ok(2), parse(INPUT).and_then(solve));
    }
}
