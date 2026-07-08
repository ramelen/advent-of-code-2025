use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(9, Part::One, &parse, &solve_part_one),
    &Solution::new(9, Part::Two, &parse, &solve_part_two),
    &Solution::new_variant(9, Part::Two, "fancy", &parse, &solve_part_two_fancy),
];

// list of corners
fn parse(input: &str) -> Result<Vec<[u64; 2]>, String> {
    input.lines().map(crate::day8::parse_point).collect()
}

// find the rectangle with maximum area
fn solve_part_one(tiles: Vec<[u64; 2]>) -> Result<u64, String> {
    tiles
        .iter()
        .enumerate()
        // iterate over pairs of corners
        .flat_map(|(i, first)| std::iter::repeat(first).zip(tiles.iter().skip(i + 1)))
        // get the area
        .map(|(&[x1, y1], &[x2, y2])| (1 + x1.abs_diff(x2)) * (1 + y1.abs_diff(y2)))
        // maximize
        .max()
        .ok_or("input must contain at least two corners".into())
}

// find the rectangle with maximum area that is entirely inside the shape formed by the given tiles
fn solve_part_two(tiles: Vec<[u64; 2]>) -> Result<u64, String> {
    // maximum rectangle area so far
    let mut max_area = 0;

    // iterate over all rectangles to find the one with the maximum area
    let (processed, new_tiles) = helpers::preprocess(tiles)?; // preprocess data to speed up iteration
    for (i, &first) in new_tiles.iter().enumerate() {
        for &second in new_tiles.iter().skip(i + 1) {
            // update the max area only if it is completely contained within the shape
            if let Some(area) = helpers::new_area(&processed, max_area, first, second) {
                max_area = area;
            }
        }
    }

    Ok(max_area)
}

// find the rectangle with maximum area that is entirely inside the shape formed by the given tiles (with an additional check to speed things up)
fn solve_part_two_fancy(tiles: Vec<[u64; 2]>) -> Result<u64, String> {
    // maximum rectangle area so far
    let mut max_area = 0;

    // iterate over all rectangles to find the one with the maximum area
    let (processed, new_tiles) = helpers::preprocess(tiles)?; // preprocess data to speed up iteration
    for (i, &[x1, y1]) in new_tiles.iter().enumerate() {
        let edge_tiles = &processed.2;
        // test points in each quadrant that are guaranteed to be outside of the perimeter
        let (test_x, test_y) = (x1 + 1, y1 + 1);
        let q1_right_x = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x >= test_x && y == test_y).then_some(x))
            .min()
            .unwrap_or(test_x)
            + 1;

        let (test_x, test_y) = (x1 + 1, y1 + 1);
        let q1_up_y = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x == test_x && y >= test_y).then_some(y))
            .min()
            .unwrap_or(test_y)
            + 1;

        let (test_x, test_y) = (x1 - 1, y1 + 1);
        let q2_up_y = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x == test_x && y >= test_y).then_some(y))
            .min()
            .unwrap_or(test_y)
            + 1;

        let (test_x, test_y) = (x1 - 1, y1 + 1);
        let q2_left_x = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x <= test_x && y == test_y).then_some(x))
            .max()
            .unwrap_or(test_x)
            - 1;

        let (test_x, test_y) = (x1 - 1, y1 - 1);
        let q3_left_x = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x <= test_x && y == test_y).then_some(x))
            .max()
            .unwrap_or(test_x)
            - 1;

        let (test_x, test_y) = (x1 - 1, y1 - 1);
        let q3_down_y = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x == test_x && y <= test_y).then_some(y))
            .max()
            .unwrap_or(test_y)
            - 1;

        let (test_x, test_y) = (x1 + 1, y1 - 1);
        let q4_down_y = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x == test_x && y <= test_y).then_some(y))
            .max()
            .unwrap_or(test_y)
            - 1;

        let (test_x, test_y) = (x1 + 1, y1 - 1);
        let q4_right_x = edge_tiles
            .iter()
            .filter_map(|&[x, y]| (x >= test_x && y == test_y).then_some(x))
            .min()
            .unwrap_or(test_x)
            + 1;

        for &[x2, y2] in new_tiles.iter().skip(i + 1) {
            // check if any of the test points are in the interior of the rectangle, which would mean that the rectangle is only partially contained in the perimeter
            if (x1 < x2 && y1 < y2 && (x2 >= q1_right_x || y2 >= q1_up_y))
                || (x2 < x1 && y1 < y2 && (x2 <= q2_left_x || y2 >= q2_up_y))
                || (x2 < x1 && y2 < y1 && (x2 <= q3_left_x || y2 <= q3_down_y))
                || (x1 < x2 && y2 < y1 && (x2 >= q4_right_x || y2 <= q4_down_y))
            {
                continue;
            };

            // update the max area only if it is completely contained within the shape
            if let Some(area) = helpers::new_area(&processed, max_area, [x1, y1], [x2, y2]) {
                max_area = area;
            }
        }
    }

    Ok(max_area)
}

mod helpers {
    use super::*;

    // process the input data to make it faster to iterate over
    pub fn preprocess(
        tiles: Vec<[u64; 2]>,
    ) -> Result<((Vec<u64>, Vec<u64>, Vec<[usize; 2]>), Vec<[usize; 2]>), String> {
        // sorted lists of x and y coordinates
        let x_coords = tiles
            .iter()
            .map(|&[x, _]| x)
            .collect::<Vec<u64>>()
            .mutate(|coords| coords.sort())
            .mutate(Vec::dedup);

        let y_coords = tiles
            .iter()
            .map(|&[_, y]| y)
            .collect::<Vec<u64>>()
            .mutate(|coords| coords.sort())
            .mutate(Vec::dedup);

        // tiles compressed to smaller coordinates without losing geometrical information
        let new_tiles: Vec<[usize; 2]> = tiles
            .iter()
            .map(|&[x, y]| {
                let new_x = 2 * x_coords
                    .iter()
                    .position(|&sorted| sorted == x)
                    .expect("x-coordinate is in list");
                let new_y = 2 * y_coords
                    .iter()
                    .position(|&sorted| sorted == y)
                    .expect("y-coordinate is in list");
                [new_x, new_y]
            })
            .collect();

        // list of edges
        let firsts = new_tiles.clone();
        let seconds = new_tiles.clone().mutate(|tiles| tiles.rotate_left(1));
        let edges = firsts.into_iter().zip(seconds);

        // list of tiles on some edge
        let mut edge_tiles: Vec<[usize; 2]> = Vec::new();

        for ([x1, y1], [x2, y2]) in edges {
            if x1 == x2 {
                edge_tiles.extend(
                    (y1.min(y2) + 1..=y1.max(y2) - 1)
                        .step_by(2)
                        .map(|y| [x1, y]),
                );
            } else if y1 == y2 {
                edge_tiles.extend(
                    (x1.min(x2) + 1..=x1.max(x2) - 1)
                        .step_by(2)
                        .map(|x| [x, y1]),
                );
            } else {
                return Err(format!(
                    "adjacent tiles must be orthogonal, got {:?} and {:?}",
                    [x1, y2],
                    [x2, y2]
                ));
            }
        }

        Ok(((x_coords, y_coords, edge_tiles), new_tiles))
    }

    // the area of a rectangle; some if it is the lowest found so far and is inside the shape, none otherwise
    pub fn new_area(
        (x_coords, y_coords, edge_tiles): &(Vec<u64>, Vec<u64>, Vec<[usize; 2]>),
        max_area: u64,
        [x1, y1]: [usize; 2],
        [x2, y2]: [usize; 2],
    ) -> Option<u64> {
        let area = (1 + x_coords[x1 / 2].abs_diff(x_coords[x2 / 2]))
            * (1 + y_coords[y1 / 2].abs_diff(y_coords[y2 / 2]));

        // bail if the area isn't large to update the max area anyway
        if area <= max_area {
            return None;
        }

        let max_x = x1.max(x2);
        let min_x = x1.min(x2);
        let max_y = y1.max(y2);
        let min_y = y1.min(y2);

        // bail if any edge tiles are in the interior of the rectangle, which would mean that the rectangle is only partially contained in the perimeter
        let inner_x = min_x + 1..=max_x - 1;
        let inner_y = min_y + 1..=max_y - 1;
        let contains_edge = edge_tiles
            .iter()
            .any(|[x, y]| inner_x.contains(x) && inner_y.contains(y));
        if contains_edge {
            return None;
        }

        // find the number of intersections of the perimeter with a line going right from the bottom-left corner of the rectangle, which will be zero if the rectangle is outside the perimeter and one if the rectangle is entirely contained in it
        let (test_x, test_y) = (min_x + 1, min_y + 1);
        let right_intersections = edge_tiles
            .iter()
            .filter(|&&[x, y]| test_x == y && test_y <= x)
            .count();

        // bail if the number of intersections is zero since the rect is entirely outside the shape
        if right_intersections % 2 == 0 {
            return None;
        };

        Some(area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        7,1\n\
        11,1\n\
        11,7\n\
        9,7\n\
        9,5\n\
        2,5\n\
        2,3\n\
        7,3";

    #[test]
    fn test_parse() {
        let expected = vec![
            [7, 1],
            [11, 1],
            [11, 7],
            [9, 7],
            [9, 5],
            [2, 5],
            [2, 3],
            [7, 3],
        ];
        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(50), parse(INPUT).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(24), parse(INPUT).and_then(solve_part_two));
    }
}
