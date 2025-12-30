use crate::{Day, FromInput, Solve};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Light {
    On,
    Off,
}

impl Light {
    fn flipped(self) -> Light {
        match self {
            Light::Off => Light::On,
            Light::On => Light::Off,
        }
    }

    fn flip(&mut self) {
        *self = self.flipped();
    }
}

impl FromInput for Vec<(Vec<Light>, Vec<Vec<u64>>, Vec<u64>)> {
    fn from_input(input: impl AsRef<str>) -> Self {
        let mut parsed = Vec::new();

        for line in input.as_ref().lines() {
            let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let len = parts.len();

            let lights = parts[0]
                .chars()
                .filter_map(|char| match char {
                    '.' => Some(Light::Off),
                    '#' => Some(Light::On),
                    _ => None,
                })
                .collect::<Vec<Light>>();

            let joltages: Vec<u64> = parts[len - 1]
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            let buttons: Vec<Vec<u64>> = parts[1..len - 1]
                .into_iter()
                .map(|button| {
                    button
                        .strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split(',')
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect();

            parsed.push((lights, buttons, joltages));
        }
        parsed
    }
}

impl Solve for Day<10> {
    type PartOneData = Vec<(Vec<Light>, Vec<Vec<u64>>, Vec<u64>)>;
    type PartTwoData = Vec<(Vec<Light>, Vec<Vec<u64>>, Vec<u64>)>;

    fn part_1(machines: &Self::PartOneData) -> String {
        let mut num_presses = 0;

        for (lights, buttons, _joltages) in machines {
            let mut combos: Vec<(u64, Vec<u64>)> = vec![(0, vec![])];
            for button in buttons {
                let new_combos: Vec<(u64, Vec<u64>)> = combos
                    .iter()
                    .map(|(count, combo)| {
                        (
                            count + 1,
                            combo.iter().chain(button.iter()).copied().collect(),
                        )
                    })
                    .collect();
                combos.extend_from_slice(new_combos.as_slice());
            }

            num_presses += combos
                .into_iter()
                .filter(|(_, combo)| {
                    let mut light_state = vec![Light::Off; lights.len()];

                    combo
                        .into_iter()
                        .for_each(|&light| light_state[light as usize].flip());

                    light_state == *lights
                })
                .map(|(press_count, _)| press_count)
                .min()
                .unwrap();
        }

        num_presses.to_string()
    }

    fn part_2(machines: &Self::PartOneData) -> String {
        use rayon::prelude::*;
        machines
            .par_iter()
            .map(|(_lights, buttons, joltages)| solve_machine(buttons, joltages))
            .sum::<u64>()
            .to_string()
    }
}

fn solve_machine(buttons: &Vec<Vec<u64>>, joltages: &Vec<u64>) -> u64 {
    let mut joltage_states = vec![joltages.to_owned()];
    let mut button_states = vec![buttons.to_owned()];
    let mut press_states = vec![0];
    let mut min_presses = u64::MAX;

    'outer: while !joltage_states.is_empty() {
        let joltage_state = joltage_states.pop().unwrap();
        let button_state = button_states.pop().unwrap();
        let press_state = press_states.pop().unwrap();

        let max_joltage = joltage_state.iter().max().unwrap();
        if press_state + max_joltage >= min_presses {
            continue;
        }

        for (i, _) in joltage_state
            .iter()
            .enumerate()
            .filter(|&(_, &joltage)| joltage != 0)
        {
            if !button_state
                .iter()
                .any(|button| button.contains(&(i as u64)))
            {
                continue 'outer;
            }
        }

        let min_joltage = joltage_state
            .iter()
            .filter(|&&joltage| joltage != 0)
            .min()
            .unwrap();

        let min_joltage_pos = joltage_state
            .iter()
            .position(|joltage| joltage == min_joltage)
            .unwrap();

        let (useful_buttons, useless_buttons): (Vec<Vec<u64>>, Vec<Vec<u64>>) = button_state
            .into_iter()
            .partition(|button| button.contains(&(min_joltage_pos as u64)));

        if useful_buttons.is_empty() {
            continue;
        }

        let combos = partitions(useful_buttons.len(), *min_joltage);
        let new_press_state = press_state + min_joltage;
        for combo in combos {
            let mut new_joltage_state = joltage_state.to_owned();
            for (delta, button) in combo.iter().zip(useful_buttons.iter()) {
                for i in button {
                    new_joltage_state[*i as usize] -= delta;
                }
            }

            if new_joltage_state.iter().all(|&item| item == 0) {
                if new_press_state < min_presses {
                    min_presses = new_press_state;
                }
                continue;
            }

            joltage_states.push(new_joltage_state);
            button_states.push(useless_buttons.clone());
            press_states.push(new_press_state);
        }
    }
    min_presses
}

fn partitions(bins: usize, max: u64) -> Vec<Vec<u64>> {
    if max == 0 {
        vec![vec![0; bins]]
    } else if bins == 0 {
        panic!("don't try to give cookies to zero friends")
    } else if bins == 1 {
        vec![vec![max]]
    } else {
        (0..=max)
            .flat_map(|n| {
                partitions(bins - 1, max - n)
                    .into_iter()
                    .map(move |partition| {
                        std::iter::once(n)
                            .chain(partition.into_iter())
                            .collect::<Vec<u64>>()
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const F: Light = Light::Off;
    const T: Light = Light::On;

    const INPUT: &str = "\
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    test!(day 10, parse: Vec<(Vec<Light>, Vec<Vec<u64>>, Vec<u64>)>;
        INPUT => vec![
            (
                vec![F, T, T, F],
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1]
                ],
                vec![3, 5, 4, 7]
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
                vec![7, 5, 12, 7, 2]
            ),
            (
                vec![F, T, T, T, F, T],
                vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2],
                ],
                vec![10, 11, 11, 5, 10, 5]
            ),
        ]
    );

    test!(day 10, part 1; INPUT => String::from("7"));

    test!(day 10, part 2; INPUT => String::from("33"));
}
