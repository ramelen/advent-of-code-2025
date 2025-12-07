use crate::{Day, FromInput, Solve};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Free,
    Roll,
}

impl FromInput for Vec<Vec<Tile>> {
    fn from_input(input: impl AsRef<str>) -> Self {
        input
            .as_ref()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Free,
                        '@' => Tile::Roll,
                        _ => panic!("Invalid tile"),
                    })
                    .collect()
            })
            .collect()
    }
}

fn neighbours_count(data: &Vec<Vec<Tile>>, rows: usize, cols: usize, x: usize, y: usize) -> usize {
    let left_x = x.checked_sub(1);
    let center_x = Some(x);
    let right_x = if x + 1 < cols { Some(x + 1) } else { None };
    let top_y = y.checked_sub(1);
    let center_y = Some(y);
    let bottom_y = if y + 1 < rows { Some(y + 1) } else { None };
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
        .filter_map(|pos| {
            if let (Some(x), Some(y)) = pos {
                Some((x, y))
            } else {
                None
            }
        })
        .map(|(x, y)| data[y][x])
        .filter(|tile| tile == &Tile::Roll)
        .count()
}

impl Solve for Day<4> {
    type PartOneData = Vec<Vec<Tile>>;
    type PartTwoData = Vec<Vec<Tile>>;

    fn part_1(tiles: &Self::PartOneData) -> String {
        let mut reachable_rolls = 0;
        let rows = tiles.len();
        let cols = tiles[0].len();
        for (y, row) in tiles.iter().enumerate() {
            reachable_rolls += row
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, tile)| tile == Tile::Roll)
                .filter(|&(x, _)| neighbours_count(tiles, rows, cols, x, y) < 4)
                .count();
        }
        reachable_rolls.to_string()
    }

    fn part_2(tiles: &Self::PartTwoData) -> String {
        let mut reachable_rolls = 0;
        let rows = tiles.len();
        let cols = tiles[0].len();
        let mut data = tiles.clone();
        loop {
            let mut newly_reachable_rolls = 0;
            let mut new_data = data.clone();
            for (y, row) in data.iter().enumerate() {
                newly_reachable_rolls += row
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|&(_, tile)| tile == Tile::Roll)
                    .filter(|&(x, _)| neighbours_count(&data, rows, cols, x, y) < 4)
                    .inspect(|&(x, _)| new_data[y][x] = Tile::Free)
                    .count();
            }
            if newly_reachable_rolls == 0 {
                break;
            } else {
                reachable_rolls += newly_reachable_rolls;
            }
            data = new_data;
        }
        reachable_rolls.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

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

    test!(day 4, parse: Vec<Vec<Tile>>;
        INPUT => vec![
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
        ]
    );

    test!(day 4, part 1; INPUT => String::from("13"));

    test!(day 4, part 2; INPUT => String::from("43"));
}
