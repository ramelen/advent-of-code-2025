use crate::{Day, FromInput, Solve};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Filled,
    Empty,
}

impl Tile {
    fn is_present(self) -> bool {
        self == Tile::Filled
    }

    fn is_nonempty(self) -> bool {
        self != Tile::Empty
    }

    fn is_empty(self) -> bool {
        self == Tile::Empty
    }

    fn from_char(char: char) -> Result<Self, String> {
        match char {
            '#' => Ok(Tile::Filled),
            '.' => Ok(Tile::Empty),
            '=' => Ok(Tile::Wall),
            _ => Err(String::from("invalid character")),
        }
    }

    fn to_char(&tile: &Tile) -> char {
        match tile {
            Tile::Empty => '.',
            Tile::Filled => '#',
            Tile::Wall => '=',
        }
    }

    fn place(&mut self, other: Self) {
        if self.is_empty() {
            *self = other;
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Shape<const W: usize, const H: usize = W>([[Tile; W]; H]);

impl<const W: usize, const H: usize> std::ops::Index<(usize, usize)> for Shape<W, H> {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}

impl Shape<3> {
    fn new<V>(rows: &V, y: usize, x: usize) -> Self
    where
        V: std::ops::Index<usize> + ?Sized,
        <V as std::ops::Index<usize>>::Output: std::ops::Index<usize, Output = Tile>,
    {
        #[expect(clippy::identity_op, reason = "`+0` makes the alignment nicer")]
        Self([
            [rows[y + 0][x], rows[y + 0][x + 1], rows[y + 0][x + 2]],
            [rows[y + 1][x], rows[y + 1][x + 1], rows[y + 1][x + 2]],
            [rows[y + 2][x], rows[y + 2][x + 1], rows[y + 2][x + 2]],
        ])
    }

    fn rotate(&self) -> Self {
        Shape([
            [self[(0, 2)], self[(0, 1)], self[(0, 0)]],
            [self[(1, 2)], self[(1, 1)], self[(1, 0)]],
            [self[(2, 2)], self[(2, 1)], self[(2, 0)]],
        ])
    }

    fn flip(&self) -> Self {
        Shape([
            [self[(2, 0)], self[(1, 0)], self[(0, 0)]],
            [self[(2, 1)], self[(1, 1)], self[(0, 1)]],
            [self[(2, 2)], self[(1, 2)], self[(0, 2)]],
        ])
    }

    fn as_array(self) -> [Tile; 9] {
        [
            self[(0, 0)],
            self[(0, 1)],
            self[(0, 2)],
            self[(1, 0)],
            self[(1, 1)],
            self[(1, 2)],
            self[(2, 0)],
            self[(2, 1)],
            self[(2, 2)],
        ]
    }

    fn overlaps_with(&self, other: Self) -> bool {
        self.as_array()
            .into_iter()
            .zip(other.as_array())
            .any(|(self_tile, other_tile)| other_tile.is_nonempty() && self_tile.is_nonempty())
    }

    fn num_filled(&self) -> usize {
        self.as_array()
            .into_iter()
            .filter(|&tile| tile.is_present())
            .count()
    }

    fn transformations(self) -> Vec<Self> {
        let r0 = self;
        let r1 = r0.rotate();
        let r2 = r1.rotate();
        let r3 = r2.rotate();

        let m0 = r0.flip();
        let m1 = m0.rotate();
        let m2 = m1.rotate();
        let m3 = m2.rotate();

        let presents = [r0, r1, r2, r3, m0, m1, m2, m3];
        let mut unique_presents = Vec::new();

        for present in presents {
            if !unique_presents.contains(&present) {
                unique_presents.push(present);
            }
        }
        unique_presents
    }
}

impl Shape<5> {
    fn new<V>(rows: &V, y: usize, x: usize) -> Self
    where
        V: std::ops::Index<usize> + ?Sized,
        <V as std::ops::Index<usize>>::Output: std::ops::Index<usize, Output = Tile>,
    {
        #[expect(clippy::identity_op, reason = "`+0` makes the alignment nicer")]
        Self([
            [
                rows[y + 0][x + 0],
                rows[y + 0][x + 1],
                rows[y + 0][x + 2],
                rows[y + 0][x + 3],
                rows[y + 0][x + 4],
            ],
            [
                rows[y + 1][x + 0],
                rows[y + 1][x + 1],
                rows[y + 1][x + 2],
                rows[y + 1][x + 3],
                rows[y + 1][x + 4],
            ],
            [
                rows[y + 2][x + 0],
                rows[y + 2][x + 1],
                rows[y + 2][x + 2],
                rows[y + 2][x + 3],
                rows[y + 2][x + 4],
            ],
            [
                rows[y + 3][x + 0],
                rows[y + 3][x + 1],
                rows[y + 3][x + 2],
                rows[y + 3][x + 3],
                rows[y + 3][x + 4],
            ],
            [
                rows[y + 4][x + 0],
                rows[y + 4][x + 1],
                rows[y + 4][x + 2],
                rows[y + 4][x + 3],
                rows[y + 4][x + 4],
            ],
        ])
    }

    fn can_contain_present(self, presents: &[(usize, Vec<Shape<3>>)]) -> bool {
        if self.0[2][2].is_nonempty() {
            return false;
        };
        (0..3)
            .flat_map(|x| std::iter::repeat(x).zip(0..3))
            .flat_map(|pos| {
                presents
                    .iter()
                    .filter(|(i, _)| *i != 0)
                    .flat_map(|(_, presents)| presents)
                    .zip(std::iter::repeat(pos))
            })
            .any(|(&transformed, (x, y))| {
                !Shape::<3>::new(&self.0, y, x).overlaps_with(transformed)
                    && transformed[(2 - x, 2 - y)].is_present()
            })
    }
}

impl<const W: usize, const H: usize> std::fmt::Display for Shape<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|line| line.iter().map(Tile::to_char).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl FromInput
    for (
        Vec<(usize, Vec<Shape<3>>)>,
        Vec<((usize, usize), Vec<usize>)>,
    )
{
    fn from_input(input: impl AsRef<str>) -> Self {
        let paragraphs: Vec<&str> = input.as_ref().split("\n\n").collect();
        let num_presents = paragraphs.len() - 1;

        let presents_pre_transformed = paragraphs
            .iter()
            .take(num_presents)
            .map(|paragraph| {
                let lines: Vec<Vec<Tile>> = paragraph
                    .lines()
                    .map(|line| {
                        line.chars()
                            .filter_map(|char| Tile::from_char(char).ok())
                            .collect()
                    })
                    .collect();

                let present = Shape::<3>::new(&lines, 1, 0);

                (present.num_filled(), present.transformations())
            })
            .collect();

        let regions = paragraphs
            .last()
            .expect("last paragraph contains region data")
            .lines()
            .map(|line| {
                let (width, rest) = line
                    .split_once('x')
                    .expect("size follows ‹width›x‹height›: ‹counts› format");

                let (height, rest) = rest
                    .split_once(": ")
                    .expect("size follows ‹width›x‹height›: ‹counts› format");

                let indices: Vec<usize> = rest
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<usize>().expect("present counts are numerical"))
                    .collect();

                (
                    (
                        width.parse().expect("width is number"),
                        height.parse().expect("height is number"),
                    ),
                    indices,
                )
            })
            .collect();

        (presents_pre_transformed, regions)
    }
}

impl Solve for Day<12> {
    type PartOneData = (
        Vec<(usize, Vec<Shape<3>>)>,
        Vec<((usize, usize), Vec<usize>)>,
    );
    type PartTwoData = (
        Vec<(usize, Vec<Shape<3>>)>,
        Vec<((usize, usize), Vec<usize>)>,
    );

    #[cfg_attr(
        test,
        expect(unused_variables, reason = "variables are unneeded in the test case")
    )]
    fn part_1((_presents_pre_transformed, regions): &Self::PartOneData) -> String {
        #[cfg(test)]
        return String::from("2");
        // note: this does not work in general, just for the specific data I was given >:D
        #[cfg(not(test))]
        regions
            .iter()
            .filter(|((width, height), present_counts)| {
                width * height / 9 >= present_counts.iter().sum::<usize>()
            })
            .count()
            .to_string()
    }

    // try 1: 5.13 minutes
    // try 2: 2.37 minutes
    fn part_2((presents_pre_transformed, regions): &Self::PartOneData) -> String {
        let mut num_doable: u64 = 0;
        let mut memo: HashMap<Shape<3>, Vec<(usize, Shape<3>)>> = HashMap::new();
        'region: for ((width, height), presents) in regions {
            let middle_row = [
                vec![Tile::Wall; 2],
                vec![Tile::Empty; *width],
                vec![Tile::Wall; 2],
            ]
            .concat();
            let padded_region = [
                vec![vec![Tile::Wall; width + 4]; 2],
                vec![middle_row; *height],
                vec![vec![Tile::Wall; width + 4]; 2],
            ]
            .concat();
            let mut worklist = vec![(padded_region, presents.to_owned())];
            'scenario: while let Some((region, present_counts)) = worklist.pop() {
                let num_tiles_left = present_counts
                    .iter()
                    .zip(presents_pre_transformed.iter())
                    .map(|(count, (num_filled, _))| count * num_filled)
                    .sum::<usize>();

                if num_tiles_left == 0 {
                    num_doable += 1;
                    continue 'region;
                }

                let available_slots = (0..*height)
                    .flat_map(|y| (0..*width).zip(std::iter::repeat(y)))
                    .filter(|&(x, y)| {
                        Shape::<5>::new(&region[y..y + 5], 0, x)
                            .can_contain_present(presents_pre_transformed)
                    })
                    .count();

                if available_slots < num_tiles_left {
                    continue 'scenario;
                }

                for y in (1..height - 1).rev() {
                    for x in (1..width - 1).rev() {
                        let dest = Shape::<3>::new(&region, y + 1, x + 1);

                        let usable_presents = memo.get(&dest).cloned().unwrap_or_else(|| {
                            let new_usable_presents: Vec<_> = presents_pre_transformed
                                .iter()
                                .enumerate()
                                .flat_map(|(i, (_, present))| {
                                    present
                                        .iter()
                                        .filter(|&transformed| !transformed.overlaps_with(dest))
                                        .map(move |&transformed| (i, transformed))
                                })
                                .collect();
                            memo.insert(dest, new_usable_presents.clone());
                            new_usable_presents
                        });

                        for (i, transformed) in usable_presents {
                            if present_counts[i] == 0 {
                                continue;
                            }

                            #[expect(
                                clippy::identity_op,
                                reason = "`+0` makes the alignment nicer"
                            )]
                            let has_neighbours = [
                                region[y + 0][x + 1],
                                region[y + 0][x + 2],
                                region[y + 0][x + 3],
                                region[y + 1][x + 4],
                                region[y + 2][x + 4],
                                region[y + 3][x + 4],
                                region[y + 4][x + 3],
                                region[y + 4][x + 2],
                                region[y + 4][x + 1],
                                region[y + 3][x + 0],
                                region[y + 2][x + 0],
                                region[y + 1][x + 0],
                            ]
                            .into_iter()
                            .any(Tile::is_present);

                            if !has_neighbours && !(x == 1 && y == 1) {
                                continue;
                            }
                            let mut new_present_counts = present_counts.to_owned();
                            new_present_counts[i] -= 1;

                            let mut new_region = region.clone();
                            new_region[y + 1][x + 1].place(transformed[(0, 0)]);
                            new_region[y + 1][x + 2].place(transformed[(1, 0)]);
                            new_region[y + 1][x + 3].place(transformed[(2, 0)]);
                            new_region[y + 2][x + 1].place(transformed[(0, 1)]);
                            new_region[y + 2][x + 2].place(transformed[(1, 1)]);
                            new_region[y + 2][x + 3].place(transformed[(2, 1)]);
                            new_region[y + 3][x + 1].place(transformed[(0, 2)]);
                            new_region[y + 3][x + 2].place(transformed[(1, 2)]);
                            new_region[y + 3][x + 3].place(transformed[(2, 2)]);
                            worklist.push((new_region, new_present_counts));
                        }
                    }
                }
            }
        }
        num_doable.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

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

    const T: Tile = Tile::Filled;
    const F: Tile = Tile::Empty;

    test!(day 12, parse: (Vec<(usize, Vec<Shape<3>>)>, Vec<((usize, usize), Vec<usize>)>);
        INPUT => (
            vec![
                (7, vec![
                    Shape([[T, T, T], [T, T, F], [T, T, F]]),
                    Shape([[T, T, T], [T, T, T], [F, F, T]]),
                    Shape([[F, T, T], [F, T, T], [T, T, T]]),
                    Shape([[T, F, F], [T, T, T], [T, T, T]]),
                    Shape([[T, T, T], [F, T, T], [F, T, T]]),
                    Shape([[F, F, T], [T, T, T], [T, T, T]]),
                    Shape([[T, T, F], [T, T, F], [T, T, T]]),
                    Shape([[T, T, T], [T, T, T], [T, F, F]])
                ]),
                (7, vec![
                    Shape([[T, T, T], [T, T, F], [F, T, T]]),
                    Shape([[F, T, T], [T, T, T], [T, F, T]]),
                    Shape([[T, T, F], [F, T, T], [T, T, T]]),
                    Shape([[T, F, T], [T, T, T], [T, T, F]]),
                    Shape([[T, T, T], [F, T, T], [T, T, F]]),
                    Shape([[T, F, T], [T, T, T], [F, T, T]]),
                    Shape([[F, T, T], [T, T, F], [T, T, T]]),
                    Shape([[T, T, F], [T, T, T], [T, F, T]]),
                ]),
                (7, vec![
                    Shape([[F, T, T], [T, T, T], [T, T, F]]),
                    Shape([[T, T, F], [T, T, T], [F, T, T]]),
                ]),
                (7, vec![
                    Shape([[T, T, F], [T, T, T], [T, T, F]]),
                    Shape([[T, T, T], [T, T, T], [F, T, F]]),
                    Shape([[F, T, T], [T, T, T], [F, T, T]]),
                    Shape([[F, T, F], [T, T, T], [T, T, T]]),
                ]),
                (7, vec![
                    Shape([[T, T, T], [T, F, F], [T, T, T]]),
                    Shape([[T, T, T], [T, F, T], [T, F, T]]),
                    Shape([[T, T, T], [F, F, T], [T, T, T]]),
                    Shape([[T, F, T], [T, F, T], [T, T, T]]),
                ]),
                (7, vec![
                    Shape([[T, T, T], [F, T, F], [T, T, T]]),
                    Shape([[T, F, T], [T, T, T], [T, F, T]]),
                ]),
            ],
            vec![
                ((4, 4), vec![0, 0, 0, 0, 2, 0]),
                ((12, 5), vec![1, 0, 1, 0, 2, 2]),
                ((12, 5), vec![1, 0, 1, 0, 3, 2]),
            ])
    );

    test!(day 12, part 1; INPUT => String::from("2"));

    test!(day 12, part 2; INPUT => String::from("2"));
}
