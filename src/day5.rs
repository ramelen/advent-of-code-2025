use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(5, Part::One, &parse, &solve_part_one),
    &Solution::new(5, Part::Two, &parse, &solve_part_two),
];

// parse input into list of id ranges and test ids
fn parse(input: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>), String> {
    // split input into ranges and ids
    let (range_strs, id_strs) = input
        .split_once("\n\n")
        .ok_or("id ranges and test ids must be separated by a blank line")?;

    // parse id ranges
    let ranges = range_strs
        .lines()
        .map(crate::day2::parse_id_range)
        .collect::<Result<Vec<(u64, u64)>, String>>()?;

    // parse ids
    let ids = id_strs
        .lines()
        .map(parse_int)
        .collect::<Result<Vec<u64>, String>>()?;

    Ok((ranges, ids))
}

// return the number of given ids that are fresh (contained in the given ranges)
fn solve_part_one((ranges, ids): (Vec<(u64, u64)>, Vec<u64>)) -> Result<u64, String> {
    Ok(ids
        .iter()
        .filter(|id| ranges.iter().any(|(start, end)| (start..=end).contains(id)))
        .count() as u64)
}

// return the total number of ids that are fresh, without duplicates
fn solve_part_two((mut ranges, _): (Vec<(u64, u64)>, Vec<u64>)) -> Result<u64, String> {
    let mut id_count = 0;

    // sort the ranges first by their start id
    ranges.sort_by_key(|(low, _)| *low);

    // get the first item or return zero
    let [(low, high), rest @ ..] = ranges.as_slice() else {
        return Ok(0);
    };

    // repeatedly merge the ranges and update the count of fresh ids
    let mut current_lowest = low;
    let mut current_highest = high;
    for (next_low, next_high) in rest {
        // shift the bottom range up and add the old range's width to the total
        if current_highest < next_low {
            id_count += 1 + current_highest - current_lowest;
            current_lowest = next_low;
        }
        // shift the top of the range up
        current_highest = current_highest.max(next_high);
    }
    // update the total one more time
    id_count += 1 + current_highest - current_lowest;

    Ok(id_count)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_parse() {
        let expected = (
            vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            vec![1, 5, 8, 11, 17, 32],
        );

        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(3), parse(INPUT).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(14), parse(INPUT).and_then(solve_part_two));
    }
}
