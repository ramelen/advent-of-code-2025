use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(1, Part::One, &parse, &solve_part_one),
    &Solution::new(1, Part::Two, &parse, &solve_part_two),
];

// parse input into a list of signed dial turn instructions
fn parse(input: &str) -> Result<Vec<i32>, String> {
    input.lines().map(helpers::parse_instruction).collect()
}

// count the number of times the dial lands on zero
fn solve_part_one(deltas: Vec<i32>) -> Result<u64, String> {
    Ok(deltas
        .into_iter()
        .scan(50, |pos, delta| {
            *pos += delta; // update the dial position
            Some(*pos % 100) // wrap to range [-99, 99]
        })
        .filter(|&pos| pos == 0)
        .count() as u64)
}

// count the number of times the dial passes over zero
fn solve_part_two(deltas: Vec<i32>) -> Result<u64, String> {
    // the number of times the dial has passed over zero
    let mut zero_count = 0;

    // excecute each turn instruction
    let mut position = 50; // current dial position
    for delta in deltas {
        // number of whole revolutions (rounded towards zero) for this delta
        zero_count += (delta / 100).unsigned_abs() as u64;

        // add the remainder to the current position
        let new_pos = position + delta % 100;

        // don't double count when the old position was zero, and don't count if the new position hasn't actually passed over zero
        if !(position == 0 || (1..100).contains(&new_pos)) {
            zero_count += 1;
        }

        // wrap the number to the range [0, 99]
        position = new_pos.rem_euclid(100);
    }

    Ok(zero_count)
}

mod helpers {
    use super::*;

    // parse a line into a signed dial turn instruction
    pub fn parse_instruction(input: &str) -> Result<i32, String> {
        // split into direction character (L | R) and distance (natural)
        let (direction_str, distance_str) = input.split_at(1);

        // parse distance as an unsigned int (so that negative inputs are errors) and convert to a signed int
        let distance = parse_int::<u32>(distance_str)?
            .try_into()
            .map_err(|_| format!("distance too large: '{distance_str}'"));

        // negate the distance if the turn direction is left
        match direction_str {
            "L" => distance.map(i32::strict_neg),
            "R" => distance,
            direction_str => Err(format!("no such turn direction '{direction_str}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82";

    #[test]
    fn test_parse() {
        assert_eq!(
            Ok(vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]),
            parse(INPUT)
        );
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(3), parse(INPUT).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(6), parse(INPUT).and_then(solve_part_two));
    }
}
