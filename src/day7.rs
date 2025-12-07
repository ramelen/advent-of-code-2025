use crate::{Day, FromInput, Solve};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Splitter,
}

impl FromInput for (Vec<u64>, Vec<Vec<Tile>>) {
    fn from_input(input: impl AsRef<str>) -> Self {
        let mut lines = input.as_ref().lines();

        let first = lines
            .next()
            .unwrap()
            .chars()
            .map(|char| match char {
                '.' => 0,
                'S' => 1,
                _ => panic!("first line should be empty except for the source"),
            })
            .collect();

        let rest = lines
            .map(|l| {
                l.chars()
                    .map(|char| match char {
                        '.' => Tile::Empty,
                        '^' => Tile::Splitter,
                        _ => panic!("invalid character"),
                    })
                    .collect()
            })
            .collect();

        (first, rest)
    }
}

impl Solve for Day<7> {
    type PartOneData = (Vec<u64>, Vec<Vec<Tile>>);
    type PartTwoData = (Vec<u64>, Vec<Vec<Tile>>);

    fn part_1((first, rest): &Self::PartOneData) -> String {
        let mut previous_row = first.to_owned();
        let len = first.len();
        let mut split_count = 0;

        for row in rest {
            let mut current_row = Vec::new();
            current_row.resize(len, 0);

            for (x, tile) in row.iter().enumerate() {
                match (previous_row[x], *tile) {
                    (incoming, Tile::Empty) => {
                        current_row[x] += incoming;
                    }
                    (0, Tile::Splitter) => {}
                    (incoming, Tile::Splitter) => {
                        split_count += 1;

                        if x > 0 {
                            current_row[x - 1] += incoming;
                        };
                        if x < len - 1 {
                            current_row[x + 1] += incoming;
                        }
                    }
                }
            }
            previous_row = current_row;
        }
        split_count.to_string()
    }

    fn part_2((first, rest): &Self::PartOneData) -> String {
        let mut previous_row = first.to_owned();
        let len = first.len();
        let mut paths = 1;

        for row in rest {
            let mut current_row = Vec::new();
            current_row.resize(len, 0);

            for (x, tile) in row.iter().enumerate() {
                match (previous_row[x], *tile) {
                    (incoming, Tile::Empty) => {
                        current_row[x] += incoming;
                    }
                    (0, Tile::Splitter) => {}
                    (incoming, Tile::Splitter) => {
                        paths += incoming;

                        if x > 0 {
                            current_row[x - 1] += incoming;
                        };
                        if x < len - 1 {
                            current_row[x + 1] += incoming;
                        }
                    }
                }
            }
            previous_row = current_row;
        }
        paths.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const E: Tile = Tile::Empty;
    const S: Tile = Tile::Splitter;

    const INPUT: &'static str = "\
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

    test!(day 7, parse: (Vec<u64>, Vec<Vec<Tile>>);
        INPUT => (vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], vec![
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
        ])
    );

    test!(day 7, part 1; INPUT => String::from("21"));

    test!(day 7, part 2; INPUT => String::from("40"));
}
