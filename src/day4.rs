use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(4, Part::One, &parse, &solve_part_one),
    &Solution::new(4, Part::Two, &parse, &solve_part_two),
];

// a tile, either empty or containing a roll
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Free,
    Roll,
}

// parse input into 2d array of tiles
fn parse(input: &str) -> Result<Vec<Vec<Tile>>, String> {
    input.lines().map(helpers::parse_row).collect()
}

// remove all rolls with less than four neighbours and return the number of rolls removed
fn solve_part_one(tiles: Vec<Vec<Tile>>) -> Result<u64, String> {
    let rows = tiles.len();
    let cols = width(&tiles)?;
    Ok(tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &tile)| (x, y, tile)))
        // filter for rolls with less than four neighbours
        .filter(|&(x, y, tile)| {
            tile == Tile::Roll && helpers::neighbours(&tiles, rows, cols, x, y) < 4
        })
        .count() as u64)
}

// repeatedly remove all rolls that have less than four neighbours until it stabilizes and return the number of rolls removed
fn solve_part_two(mut tiles: Vec<Vec<Tile>>) -> Result<u64, String> {
    // total rolls reached so far
    let mut reachable_rolls = 0;

    // repeatedly remove rolls until the layout stabilizes
    let rows = tiles.len();
    let cols = width(&tiles)?;
    loop {
        let mut new_data = tiles.to_owned();

        // rolls reachable in this iteration
        let newly_reachable_rolls = tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &tile)| (x, y, tile)))
            // filter for rolls with less than four neighbours
            .filter(|&(x, y, tile)| {
                tile == Tile::Roll && helpers::neighbours(&tiles, rows, cols, x, y) < 4
            })
            // remove rolls from list
            .inspect(|&(x, y, _)| new_data[y][x] = Tile::Free)
            .count() as u64;

        // break if layout has converged
        if newly_reachable_rolls == 0 {
            break;
        }

        // update count and layout
        reachable_rolls += newly_reachable_rolls;
        tiles = new_data;
    }

    Ok(reachable_rolls)
}

mod helpers {
    use super::*;

    // parse a character into a tile type
    pub fn parse_tile(tile_char: char) -> Result<Tile, String> {
        match tile_char {
            '.' => Ok(Tile::Free),
            '@' => Ok(Tile::Roll),
            char => Err(format!("no such tile '{char}'")),
        }
    }

    // parse input into a row of tiles
    pub fn parse_row(row_str: &str) -> Result<Vec<Tile>, String> {
        row_str.chars().map(parse_tile).collect()
    }

    // count how many of the 8 neighbours of a tile are rolls
    pub fn neighbours(data: &[Vec<Tile>], rows: usize, cols: usize, x: usize, y: usize) -> usize {
        let left_x = x.checked_sub(1);
        let center_x = Some(x);
        let right_x = (x + 1 < cols).then_some(x + 1);
        let top_y = y.checked_sub(1);
        let center_y = Some(y);
        let bottom_y = (y + 1 < rows).then_some(y + 1);
        let indices = [
            (left_x, top_y),
            (left_x, center_y),
            (left_x, bottom_y),
            (center_x, top_y),
            (center_x, bottom_y),
            (right_x, top_y),
            (right_x, center_y),
            (right_x, bottom_y),
        ];
        indices
            .into_iter()
            .filter(|&(x, y)| x.zip(y).is_some_and(|(x, y)| data[y][x] == Tile::Roll))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.";

    const R: Tile = Tile::Roll;
    const F: Tile = Tile::Free;

    #[test]
    fn test_parse() {
        let expected = vec![
            vec![F, F, R, R, F, R, R, R, R, F],
            vec![R, R, R, F, R, F, R, F, R, R],
            vec![R, R, R, R, R, F, R, F, R, R],
            vec![R, F, R, R, R, R, F, F, R, F],
            vec![R, R, F, R, R, R, R, F, R, R],
            vec![F, R, R, R, R, R, R, R, F, R],
            vec![F, R, F, R, F, R, F, R, R, R],
            vec![R, F, R, R, R, F, R, R, R, R],
            vec![F, R, R, R, R, R, R, R, R, F],
            vec![R, F, R, F, R, R, R, F, R, F],
        ];

        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(13), parse(INPUT).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(43), parse(INPUT).and_then(solve_part_two));
    }
}
