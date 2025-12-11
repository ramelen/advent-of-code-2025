use crate::{Day, FromInput, Solve};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Splitter,
}

impl FromInput for Vec<(u64, u64, u64)> {
    fn from_input(input: impl AsRef<str>) -> Self {
        input
            .as_ref()
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .map(|l| (l[0], l[1], l[2]))
            .collect()
    }
}

fn square_dist((x1, y1, z1): (u64, u64, u64), (x2, y2, z2): (u64, u64, u64)) -> u64 {
    x1.abs_diff(x2) * x1.abs_diff(x2)
        + y1.abs_diff(y2) * y1.abs_diff(y2)
        + z1.abs_diff(z2) * z1.abs_diff(z2)
}

impl Solve for Day<8> {
    type PartOneData = Vec<(u64, u64, u64)>;
    type PartTwoData = Vec<(u64, u64, u64)>;

    fn part_1(junctions: &Self::PartOneData) -> String {
        let pairs = {
            let mut vec: Vec<((u64, u64, u64), (u64, u64, u64))> = junctions
                .iter()
                .enumerate()
                .flat_map(|(i, &first)| {
                    std::iter::repeat(first).zip(junctions.iter().copied().skip(i + 1))
                })
                .collect();

            vec.sort_by_cached_key(|&(first, second)| square_dist(first, second));
            vec
        };

        let mut circuits: Vec<Vec<(u64, u64, u64)>> =
            junctions.iter().map(|&junction| vec![junction]).collect();

        #[cfg(test)]
        const TAKE_AMOUNT: usize = 10;

        #[cfg(not(test))]
        const TAKE_AMOUNT: usize = 1000;

        for (first, second) in pairs.iter().take(TAKE_AMOUNT) {
            let first_circuit_i = circuits
                .iter()
                .position(|vec| vec.contains(&first))
                .unwrap();
            let mut first_circuit = circuits.remove(first_circuit_i);

            if let Some(second_circuit_i) = circuits.iter().position(|vec| vec.contains(&second)) {
                circuits[second_circuit_i].append(&mut first_circuit);
            } else {
                circuits.push(first_circuit);
            }
        }

        let mut lengths: Vec<usize> = circuits.into_iter().map(|circuit| circuit.len()).collect();

        lengths.sort();

        lengths
            .into_iter()
            .rev()
            .take(3)
            .product::<usize>()
            .to_string()
    }

    fn part_2(junctions: &Self::PartOneData) -> String {
        let pairs = {
            let mut vec: Vec<((u64, u64, u64), (u64, u64, u64))> = junctions
                .iter()
                .enumerate()
                .flat_map(|(i, &first)| {
                    std::iter::repeat(first).zip(junctions.iter().copied().skip(i + 1))
                })
                .collect();

            vec.sort_by_cached_key(|&(first, second)| square_dist(first, second));
            vec
        };

        let mut circuits: Vec<Vec<(u64, u64, u64)>> =
            junctions.iter().map(|&junction| vec![junction]).collect();

        for (first, second) in pairs {
            let first_circuit_i = circuits
                .iter()
                .position(|vec| vec.contains(&first))
                .unwrap();
            let mut first_circuit = circuits.remove(first_circuit_i);

            if let Some(second_circuit_i) = circuits.iter().position(|vec| vec.contains(&second)) {
                circuits[second_circuit_i].append(&mut first_circuit);
            } else {
                circuits.push(first_circuit);
            }

            if circuits.len() == 1 {
                return (first.0 * second.0).to_string();
            }
        }
        unreachable!("the loop should eventually merge all junctions")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const INPUT: &'static str = "\
        162,817,812\n\
        57,618,57\n\
        906,360,560\n\
        592,479,940\n\
        352,342,300\n\
        466,668,158\n\
        542,29,236\n\
        431,825,988\n\
        739,650,466\n\
        52,470,668\n\
        216,146,977\n\
        819,987,18\n\
        117,168,530\n\
        805,96,715\n\
        346,949,466\n\
        970,615,88\n\
        941,993,340\n\
        862,61,35\n\
        984,92,344\n\
        425,690,689";

    test!(day 8, parse: Vec<(u64, u64, u64)>;
        INPUT => vec![
            (162, 817, 812),
            (57, 618, 57),
            (906, 360, 560),
            (592, 479, 940),
            (352, 342, 300),
            (466, 668, 158),
            (542, 29, 236),
            (431, 825, 988),
            (739, 650, 466),
            (52, 470, 668),
            (216, 146, 977),
            (819, 987, 18),
            (117, 168, 530),
            (805, 96, 715),
            (346, 949, 466),
            (970, 615, 88),
            (941, 993, 340),
            (862, 61, 35),
            (984, 92, 344),
            (425, 690, 689)
        ]
    );

    test!(day 8, part 1; INPUT => String::from("40"));

    test!(day 8, part 2; INPUT => String::from("25272"));
}
