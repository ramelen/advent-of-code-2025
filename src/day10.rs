use crate::util::*;
use rayon::prelude::*;
use std::{collections::HashMap, iter::FilterMap, vec::IntoIter};

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(10, Part::One, &parse, &solve_part_one),
    &Solution::new_variant(10, Part::Two, "long", &parse, &solve_part_two),
    &Solution::new_variant(10, Part::Two, "fancy", &parse, &solve_part_two_fancy),
];

fn parse(input: &str) -> Result<Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u64>)>, String> {
    input.lines().map(helpers::parse_machine).collect()
}

fn solve_part_one(machines: Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u64>)>) -> Result<u64, String> {
    machines
        .into_iter()
        .map(|(lights, buttons, _)| {
            helpers::combos(&buttons)
                .into_iter()
                .filter_map(|(press_count, combo)| {
                    let mut light_state = vec![false; lights.len()];
                    combo.iter().for_each(|&i| light_state[i] = !light_state[i]);
                    (light_state == *lights).then_some(press_count)
                })
                .min()
                .ok_or("all machines must have solutions".to_string())
        })
        .sum()
}

fn solve_part_two(machines: Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u64>)>) -> Result<u64, String> {
    Ok(machines.par_iter().map(helpers::solve_machine).sum())
}

fn solve_part_two_fancy(
    machines: Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u64>)>,
) -> Result<u64, String> {
    machines
        .into_par_iter()
        .map(|(_, mut buttons, mut joltages)| {
            helpers::reduce_machine(&mut buttons, &mut joltages);
            helpers::solve_machine_fancy(&buttons, &joltages, &mut HashMap::new())
                .ok_or("all machines must have a solution".to_string())
        })
        .sum()
}

mod helpers {
    use super::*;

    // parse line into a machine, composed of lights, buttons, and joltage requirements
    pub fn parse_machine(line: &str) -> Result<(Vec<bool>, Vec<Vec<usize>>, Vec<u64>), String> {
        // split by whitespace into lights, a list of buttons, and the joltage requirements
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let [lights_str, buttons_str @ .., joltage_str] = parts.as_slice() else {
            return Err(format!(
                "row must contain lights, buttons, and joltage requirements: got only {len} parts",
                len = parts.len()
            ));
        };

        // parse light indicators
        let lights = lights_str
            .strip_prefix('[')
            .ok_or(format!("diagram doesn't begin with '[': '{lights_str}'"))?
            .strip_suffix(']')
            .ok_or(format!("diagram doesn't end with ']': '{lights_str}'"))?
            .chars()
            .map(|char| match char {
                '.' => Ok(false),
                '#' => Ok(true),
                char => Err(format!("no such light indicator '{char}'")),
            })
            .collect::<Result<Vec<bool>, String>>()?;

        // parse buttons
        let buttons = buttons_str
            .iter()
            .map(|button| {
                button
                    .strip_prefix('(')
                    .ok_or(format!("button doesn't begin with '(': '{button}'"))?
                    .strip_suffix(')')
                    .ok_or(format!("button doesn't end with ')': '{button}'"))?
                    .split(',')
                    .map(parse_int)
                    .collect()
            })
            .collect::<Result<Vec<Vec<usize>>, String>>()?;

        // parse joltage requirements
        let joltages = joltage_str
            .strip_prefix('{')
            .ok_or(format!("joltages don't begin with '{{': '{joltage_str}'"))?
            .strip_suffix('}')
            .ok_or(format!("joltages don't end with '}}': '{joltage_str}'"))?
            .split(',')
            .map(parse_int)
            .collect::<Result<Vec<u64>, String>>()?;

        Ok((lights, buttons, joltages))
    }

    // simplify buttons and joltage requirements
    pub fn reduce_machine(buttons: &mut Vec<Vec<usize>>, joltages: &mut Vec<u64>) {
        use std::collections::HashSet;

        // collect buttons into sets
        let mut button_sets: Vec<HashSet<usize>> = buttons
            .clone()
            .into_iter()
            .map(HashSet::from_iter)
            .collect();

        // repeatedly perform a simplifying operation until convergence
        'outer: loop {
            let mut new_joltages = joltages.clone();
            for (test_index, joltage) in joltages.iter().enumerate() {
                // indices shared by all buttons that have the test index, implying that we can remove the shared indices from these buttons and reduce the requirement for the shared indices by exactly the requirement for the test index
                let shared: HashSet<usize> = button_sets
                    .iter()
                    .filter(|button_set| button_set.contains(&test_index))
                    .cloned()
                    .reduce(|acc, button_set| acc.intersection(&button_set).copied().collect())
                    .unwrap_or(HashSet::new())
                    .mutate(|shared| shared.remove(&test_index));

                // modify only if the buttons share at least one index
                if shared.is_empty() {
                    continue;
                }

                // lower the shared requirments by the test requirement
                for affected in &shared {
                    new_joltages[*affected] -= joltage;
                }

                // remove the shared indices for each button
                for button_set in &mut button_sets {
                    if button_set.contains(&test_index) {
                        *button_set = button_set.difference(&shared).copied().collect();
                    }
                }

                *joltages = new_joltages;
                continue 'outer;
            }
            break;
        }

        *buttons = button_sets.into_iter().map(Vec::from_iter).collect()
    }

    // solve a machine using a (brute force) depth-first search of all the ways to fill the joltage requirements
    pub fn solve_machine((_, buttons, joltages): &(Vec<bool>, Vec<Vec<usize>>, Vec<u64>)) -> u64 {
        // minimum required presses to solve machine so far
        let mut min_presses = u64::MAX;

        // worklist containing the number of presses, the joltage state, and the list of useful buttons for each scenario
        let mut worklist = vec![(0, joltages.to_owned(), Vec::from_iter(buttons))];
        while let Some((press_count, joltage_state, button_state)) = worklist.pop() {
            // bail if the current state can't improve on the best press count so far
            if press_count + joltage_state.iter().max().unwrap() >= min_presses {
                continue;
            }

            // bail if there is a joltage requirement that isn't affected by any button
            let all_joltages_fillable = joltage_state.iter().enumerate().all(|(i, joltage)| {
                *joltage == 0 || button_state.iter().any(|button| button.contains(&i))
            });
            if !all_joltages_fillable {
                continue;
            }

            // the smallest joltage requirement yet to be filled
            let min_joltage = joltage_state
                .iter()
                .filter(|&&joltage| joltage != 0)
                .min()
                .expect("all zero joltages caught in previous iterations of loop");

            // the index of the minimum joltage requirement
            let min_joltage_pos = joltage_state.element_offset(min_joltage).unwrap();

            // the buttons that do and do not affect the target joltage
            let (useful_buttons, useless_buttons): (Vec<_>, Vec<_>) = button_state
                .into_iter()
                .partition(|button| button.contains(&min_joltage_pos));

            // the number of presses
            let new_press_count = press_count + min_joltage;
            for combo in partitions(useful_buttons.len(), *min_joltage) {
                // the joltage state after pressing the buttons in the combo
                let new_joltage_state = joltage_state.to_owned().mutate(|state| {
                    for (delta, &button) in combo.iter().zip(useful_buttons.iter()) {
                        for i in button {
                            state[*i] -= delta;
                        }
                    }
                });

                if new_joltage_state.iter().all(|&item| item == 0) {
                    // update the min if the machine is solved
                    min_presses = min_presses.min(new_press_count);
                } else {
                    // once we have tried all the ways to exactly fill the smallest joltage requirement, we cannot press any of the 'useful' buttons anymore since that would go over the joltage requirement, so we can remove those buttons and do the next iteration with only the 'useless' buttons
                    worklist.push((new_press_count, new_joltage_state, useless_buttons.clone()));
                }
            }
        }

        min_presses
    }

    // solve a machine by recursively reducing the machine by approximately half to save on redundant search iterations
    pub fn solve_machine_fancy(
        buttons: &[Vec<usize>],
        joltages: &[u64],
        memo: &mut HashMap<Vec<bool>, Vec<(u64, Vec<u64>)>>,
    ) -> Option<u64> {
        // return early if the joltage requirements have already been met
        if joltages.iter().all(|&joltage| joltage == 0) {
            return Some(0);
        }

        // each element is true if the joltage requirement is odd and false if it is even
        let parities: Vec<bool> = joltages.iter().map(|joltage| joltage % 2 == 0).collect();

        // for each way to correct the joltage parities, solve a machine with half of the corrected requirements (which are all even) and double the press count, effectively finding a solution where you press each button an even number of times and then at most once
        memo.entry(parities.clone())
            .or_insert_with(|| parity_correcting_combos(&parities, buttons, joltages).collect())
            .to_owned()
            .into_iter()
            .filter_map(|(press_count, joltage_diffs)| {
                // joltage requirements for a reduced machine
                let reduced_joltages = joltages
                    .iter()
                    .zip(joltage_diffs)
                    // subtract the diff and divide by two to get the reduced requirements
                    .map(|(original, change)| original.checked_sub(change).map(|val| val / 2))
                    .collect::<Option<Vec<u64>>>()?;

                // solve the reduced machine
                solve_machine_fancy(buttons, &reduced_joltages, memo)
                    // double the count reduced solution and add the count for the parity correction
                    .map(|count| 2 * count + press_count)
            })
            // also try to solve the machine without a reduction as a base case
            .chain(solve_machine_base_case(buttons, joltages))
            .min()
    }

    // return every combo that reduces the joltage state to only even requirements, using at most one press of each button
    fn parity_correcting_combos(
        parities: &[bool],
        buttons: &[Vec<usize>],
        joltages: &[u64],
    ) -> FilterMap<
        IntoIter<(u64, Vec<usize>)>,
        impl FnMut((u64, Vec<usize>)) -> Option<(u64, Vec<u64>)>,
    > {
        combos(buttons).into_iter().filter_map(|(count, combo)| {
            let joltage_state = vec![0; joltages.len()]
                .mutate(|joltages| combo.iter().for_each(|&index| joltages[index] += 1));
            joltage_state
                .iter()
                .map(|joltage| joltage % 2 == 0)
                .eq(parities.iter().copied())
                .then_some((count, joltage_state))
        })
    }

    // attempt to solve a machine using at most one press of each button
    fn solve_machine_base_case(buttons: &[Vec<usize>], joltages: &[u64]) -> Option<u64> {
        combos(buttons)
            .into_iter()
            .filter_map(|(count, combo)| {
                let joltage_state = vec![0; joltages.len()]
                    .mutate(|joltages| combo.iter().for_each(|&i| joltages[i] += 1));
                (joltage_state == *joltages).then_some(count)
            })
            .min()
    }

    // returns a list of button press counts for all possible ways to press `bins` buttons that add up to `max`
    fn partitions(bins: usize, max: u64) -> Vec<Vec<u64>> {
        fn inner(total_bins: usize, bins_left: usize, max: u64) -> Vec<Vec<u64>> {
            if bins_left < 2 || max == 0 {
                // base case: there is only one way to partition the elements
                let mut partition = Vec::with_capacity(total_bins);
                partition.extend(std::iter::repeat_n(max, bins_left));
                vec![partition]
            } else {
                // recurse over all possible choices for the first button
                (0..=max)
                    .flat_map(|n| {
                        inner(total_bins, bins_left - 1, max - n).into_iter().map(
                            move |mut partition| {
                                partition.push(n);
                                partition
                            },
                        )
                    })
                    .collect()
            }
        }
        inner(bins, bins, max)
    }

    // list all subsets of the given list of buttons
    pub fn combos(buttons: &[Vec<usize>]) -> Vec<(u64, Vec<usize>)> {
        // initialize with the empty set
        let mut combos = vec![(0, Vec::new())];

        for button in buttons {
            // re-add every combo to the list but with an additional button, so that there is now every combo without the button and every combo with the button somewhere in the list
            combos.extend(
                combos.clone().into_iter().map(|(count, combo)| {
                    (count + 1, combo.iter().chain(button).copied().collect())
                }),
            );
        }

        combos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const F: bool = false;
    const T: bool = true;

    const INPUT: &str = "\
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_parse() {
        let expected = vec![
            (
                vec![F, T, T, F],
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                vec![3, 5, 4, 7],
            ),
            (
                vec![F, F, F, T, F],
                vec![
                    vec![0, 2, 3, 4],
                    vec![2, 3],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4],
                ],
                vec![7, 5, 12, 7, 2],
            ),
            (
                vec![F, T, T, T, F, T],
                vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2],
                ],
                vec![10, 11, 11, 5, 10, 5],
            ),
        ];
        assert_eq!(Ok(expected), parse(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(7), parse(INPUT).and_then(solve_part_one));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(33), parse(INPUT).and_then(solve_part_two));
    }
}
