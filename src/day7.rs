use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(7, Part::One, &parse, &solve_part_one),
    &Solution::new(7, Part::Two, &parse, &solve_part_two),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Splitter,
}

// parse input into a top row of sources and grid of splitters
fn parse(input: &str) -> Result<(Vec<u64>, Vec<Vec<Tile>>), String> {
    let mut lines = input.lines();

    // parse starting condition (number of beams in first row)
    let first_row = lines
        .next()
        .ok_or("input must contain at least one row".to_string())?
        .chars()
        .map(helpers::parse_beam_count)
        .collect::<Result<Vec<u64>, String>>()?;

    // parse splitter positions
    let rest_rows = lines
        .map(|line| line.chars().map(helpers::parse_tile).collect())
        .collect::<Result<Vec<Vec<Tile>>, String>>()?;

    Ok((first_row, rest_rows))
}

// counts total number of splits
fn solve_part_one(input: (Vec<u64>, Vec<Vec<Tile>>)) -> Result<u64, String> {
    helpers::solve(input).map(|(num_splits, _)| num_splits)
}

// counts final number of beams
fn solve_part_two(input: (Vec<u64>, Vec<Vec<Tile>>)) -> Result<u64, String> {
    helpers::solve(input).map(|(_, num_beams)| num_beams)
}

mod helpers {
    use super::*;

    // counts number of beams in a first row tile
    pub fn parse_beam_count(tile_char: char) -> Result<u64, String> {
        match tile_char {
            '.' | '^' => Ok(0),
            'S' => Ok(1),
            tile => Err(format!("no such tile '{tile}'")),
        }
    }

    // parse a character into a tile
    pub fn parse_tile(tile_char: char) -> Result<Tile, String> {
        match tile_char {
            '.' => Ok(Tile::Empty),
            '^' => Ok(Tile::Splitter),
            'S' => Err("tile 'S' may only appear in the first row".into()),
            tile => Err(format!("no such tile '{tile}'")),
        }
    }

    // propagates the beams and return the number of splits and the number of beams
    pub fn solve((first, rest): (Vec<u64>, Vec<Vec<Tile>>)) -> Result<(u64, u64), String> {
        let mut num_splits = 0;
        let mut num_beams = first.iter().sum();

        // repeatedly propagate beams and update the total
        let mut previous_row = [&[0], first.as_slice(), &[0]].concat(); // pad row to avoid out of bounds access
        for row in rest {
            // the beam counts for the current row
            let mut current_row = vec![0; first.len() + 2];

            // propagate beams from the previous row according to the layout of `row`
            for (x, tile) in row.iter().enumerate() {
                let x = x + 1; // shift to account for padding
                match (previous_row[x], *tile) {
                    // no change required
                    (0, _) => {}
                    // propagate beam downwards
                    (incoming, Tile::Empty) => current_row[x] += incoming,
                    // split beam to two adjacent cells and increment split count
                    (incoming, Tile::Splitter) => {
                        num_splits += 1;
                        num_beams += incoming;
                        current_row[x - 1] += incoming;
                        current_row[x + 1] += incoming;
                    }
                }
            }

            previous_row = current_row;
        }

        Ok((num_splits, num_beams))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const E: Tile = Tile::Empty;
    const S: Tile = Tile::Splitter;

    const INPUT: &str = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............";

    #[test]
    fn test_parse() {
        let expected = (
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, S, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, S, E, S, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, S, E, S, E, S, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
                vec![E, E, E, E, S, E, S, E, E, E, S, E, E, E, E],
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
                vec![E, E, E, S, E, S, E, E, E, S, E, S, E, E, E],
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
                vec![E, E, S, E, E, E, S, E, E, E, E, E, S, E, E],
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
                vec![E, S, E, S, E, S, E, S, E, S, E, E, E, S, E],
                vec![E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
            ],
        );

        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(21), parse(INPUT).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(40), parse(INPUT).and_then(solve_part_two));
    }
}
