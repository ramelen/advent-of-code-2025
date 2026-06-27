use crate::{Day, FromInput, Solve};

impl FromInput for Vec<(u64, u64)> {
    fn from_input(input: impl AsRef<str>) -> Self {
        input
            .as_ref()
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
            .collect()
    }
}

impl Solve for Day<9> {
    type PartOneData = Vec<(u64, u64)>;
    type PartTwoData = Vec<(u64, u64)>;

    fn part_1(tiles: &Self::PartOneData) -> String {
        tiles
            .iter()
            .enumerate()
            .flat_map(|(i, first)| std::iter::repeat(first).zip(tiles.iter().skip(i + 1)))
            .map(|(&(x1, y1), &(x2, y2))| (1 + x1.abs_diff(x2)) * (1 + y1.abs_diff(y2)))
            .max()
            .unwrap()
            .to_string()
    }

    fn part_2(tiles: &Self::PartTwoData) -> String {
        let x_coords = {
            let mut vec: Vec<u64> = tiles.iter().map(|&(x, _)| x).collect();
            vec.sort();
            vec.dedup();
            vec
        };
        let y_coords = {
            let mut vec: Vec<u64> = tiles.iter().map(|&(_, y)| y).collect();
            vec.sort();
            vec.dedup();
            vec
        };

        let new_tiles: Vec<(u64, u64)> = tiles
            .iter()
            .map(|&(x, y)| {
                (
                    2 * x_coords.iter().filter(|&&sorted| sorted < x).count() as u64,
                    2 * y_coords.iter().filter(|&&sorted| sorted < y).count() as u64,
                )
            })
            .collect();

        let rects = new_tiles
            .iter()
            .enumerate()
            .flat_map(|(i, first)| std::iter::repeat(first).zip(new_tiles.iter().skip(i + 1)));

        let firsts = new_tiles.clone();
        let seconds = {
            let mut vec = new_tiles.clone();
            vec.rotate_left(1);
            vec
        };

        let edges = firsts.into_iter().zip(seconds);
        let mut edge_tiles: Vec<(u64, u64)> = Vec::new();

        for ((x1, y1), (x2, y2)) in edges {
            if x1 == x2 {
                let max_y = y1.max(y2);
                let min_y = y1.min(y2);
                edge_tiles.extend(
                    ((min_y + 1)..=(max_y - 1))
                        .map(|y| (x1, y))
                        .collect::<Vec<(u64, u64)>>(),
                );
            } else if y1 == y2 {
                let max_x = x1.max(x2);
                let min_x = x1.min(x2);
                edge_tiles.extend(
                    ((min_x + 1)..=(max_x - 1))
                        .map(|x| (x, y1))
                        .collect::<Vec<(u64, u64)>>(),
                );
            } else {
                unreachable!()
            }
        }

        let mut max_area = 0;
        for ((x1, y1), (x2, y2)) in rects {
            if x1 == x2 || y1 == y2 {
                continue;
            }

            let max_x = *x1.max(x2);
            let min_x = *x1.min(x2);
            let max_y = *y1.max(y2);
            let min_y = *y1.min(y2);

            let area = (1 + x_coords[max_x as usize / 2] - x_coords[min_x as usize / 2])
                * (1 + y_coords[max_y as usize / 2] - y_coords[min_y as usize / 2]);

            if area <= max_area {
                continue;
            }

            let inner_x = (min_x + 1)..=(max_x - 1);
            let inner_y = (min_y + 1)..=(max_y - 1);

            if edge_tiles
                .iter()
                .any(|(x, y)| inner_x.contains(x) && inner_y.contains(y))
            {
                continue;
            }

            let test_x = min_x + 1;
            let test_y = min_y + 1;

            let right_intersections = edge_tiles
                .iter()
                .filter(|&&(x, y)| test_y == y && test_x <= x)
                .count();

            if right_intersections % 2 == 1 {
                max_area = area;
            };
        }

        max_area.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const INPUT: &str = "\
        7,1\n\
        11,1\n\
        11,7\n\
        9,7\n\
        9,5\n\
        2,5\n\
        2,3\n\
        7,3";

    test!(day 9, parse: Vec<(u64, u64)>;
        INPUT => vec![
            (7, 1),
            (11,1),
            (11,7),
            (9,7),
            (9,5),
            (2,5),
            (2,3),
            (7,3)
        ]
    );

    test!(day 9, part 1; INPUT => String::from("50"));

    test!(day 9, part 2; INPUT => String::from("24"));
}
