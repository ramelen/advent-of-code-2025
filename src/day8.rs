use crate::util::*;

pub use helpers::parse_point;
pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(8, Part::One, &parse, &solve_part_one::<1000>),
    &Solution::new(8, Part::Two, &parse, &solve_part_two),
];

// parse input into list of 3d points
fn parse(input: &str) -> Result<Vec<[u64; 3]>, String> {
    input.lines().map(helpers::parse_point).collect()
}

// find the product of the sizes of the three largest circuits after the closest `NUM_MERGES` pairs of junctions have been merged into the same circuit
fn solve_part_one<const NUM_MERGES: usize>(junctions: Vec<[u64; 3]>) -> Result<u64, String> {
    Ok(junctions
        .iter()
        // create a list of lists that each contain one junction
        .map(|&junction| vec![junction])
        .collect::<Vec<Vec<[u64; 3]>>>()
        // merge some of the circuits together according to the rules
        .mutate(|circuits| helpers::merge_circuits::<NUM_MERGES>(circuits, &junctions))
        // find the product of the top three lengths
        .into_iter()
        // convert to lengths
        .map(|circuit| circuit.len() as u64)
        .collect::<Vec<u64>>()
        // sort by circuit length
        .mutate(|lengths| lengths.sort())
        .into_iter()
        .rev()
        // get the top three
        .take(3)
        // find their product
        .product())
}

// find the product of the x-coordinates of the junctions that will be merged last
fn solve_part_two(junctions: Vec<[u64; 3]>) -> Result<u64, String> {
    // create a list of lists that each contain one junction
    let mut circuits = junctions
        .iter()
        .map(|&junction| vec![junction])
        .collect::<Vec<Vec<[u64; 3]>>>();

    // repeatedly merge circuits until there is only one left
    Ok(junctions
        .iter()
        .enumerate()
        // iterate over pairs of junctions
        .flat_map(|(i, &first)| std::iter::repeat(first).zip(junctions.iter().copied().skip(i + 1)))
        .collect::<Vec<([u64; 3], [u64; 3])>>()
        // sort pairs by their distance to each other
        .mutate(|pairs| pairs.sort_by_cached_key(helpers::square_dist))
        .into_iter()
        // merge each pair until there is only one circuit left
        .find_map(|(first, second)| {
            helpers::merge_pair(&mut circuits, first, second);
            // the product of the x-coordinates of the last pair to be merged
            (circuits.len() == 1).then_some(first[0] * second[0])
        })
        .expect("the loop should eventually merge all circuits"))
}

mod helpers {
    use super::*;

    // parse line into a point with `DIM` coordinates
    pub fn parse_point<const DIM: usize>(line: &str) -> Result<[u64; DIM], String> {
        line.split(',')
            .map(parse_int) // parse coordinates
            .collect::<Result<Vec<u64>, String>>()?
            .try_into() // convert to exact length list
            .map_err(|vec: Vec<u64>| {
                let len = vec.len();
                format!("line must contain exactly {DIM} comma-seperated coordinates (got {len})")
            })
    }

    // merge together the first `NUM_MERGES` pairs of circuits, sorted by the distance between circuit pairs
    pub fn merge_circuits<const NUM_MERGES: usize>(
        circuits: &mut Vec<Vec<[u64; 3]>>,
        junctions: &[[u64; 3]],
    ) {
        junctions
            .iter()
            .enumerate()
            // iterate over pairs of junctions
            .flat_map(|(i, &first)| {
                std::iter::repeat(first).zip(junctions.iter().copied().skip(i + 1))
            })
            .collect::<Vec<([u64; 3], [u64; 3])>>()
            // sort pairs by their distance to each other
            .mutate(|pairs| pairs.sort_by_cached_key(square_dist))
            .into_iter()
            // take the specified number of pairs
            .take(NUM_MERGES)
            // merge each pair of circuits
            .for_each(|(first, second)| merge_pair(circuits, first, second));
    }

    // merge the lists from `circuits` that contain the two circuits `first` and `second`
    pub fn merge_pair(circuits: &mut Vec<Vec<[u64; 3]>>, first: [u64; 3], second: [u64; 3]) {
        // find the position of the first element
        let first_index = circuits
            .iter()
            .position(|vec| vec.contains(&first))
            .expect("junction is in some circuit in the list");

        // find the position of the second element
        let second_index = circuits
            .iter()
            .position(|vec| vec.contains(&second))
            .expect("junction is in some circuit in the list");

        // merge the right circuit into the left one if they are not already merged
        if first_index < second_index {
            let mut second_circuit = circuits.remove(second_index);
            circuits[first_index].append(&mut second_circuit);
        } else if second_index < first_index {
            let mut first_circuit = circuits.remove(first_index);
            circuits[second_index].append(&mut first_circuit);
        }
    }

    // find the squared distance between two points
    pub fn square_dist(&(lhs, rhs): &([u64; 3], [u64; 3])) -> u64 {
        lhs.into_iter()
            .zip(rhs)
            .map(|(a, b)| a.abs_diff(b).pow(2)) // (a - b)^2
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
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

    #[test]
    fn test_parse() {
        let expected = vec![
            [162, 817, 812],
            [57, 618, 57],
            [906, 360, 560],
            [592, 479, 940],
            [352, 342, 300],
            [466, 668, 158],
            [542, 29, 236],
            [431, 825, 988],
            [739, 650, 466],
            [52, 470, 668],
            [216, 146, 977],
            [819, 987, 18],
            [117, 168, 530],
            [805, 96, 715],
            [346, 949, 466],
            [970, 615, 88],
            [941, 993, 340],
            [862, 61, 35],
            [984, 92, 344],
            [425, 690, 689],
        ];
        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(40), parse(INPUT).and_then(solve_part_one::<10>));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(25272), parse(INPUT).and_then(solve_part_two));
    }
}
