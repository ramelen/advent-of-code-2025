use crate::{Day, FromInput, Solve};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    Sum(Vec<u64>),
    Product(Vec<u64>),
}

impl Problem {
    fn evaluate(&self) -> u64 {
        match self {
            Problem::Product(nums) => nums.iter().product(),
            Problem::Sum(nums) => nums.iter().sum(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CephalopodProblem {
    Sum(Vec<u64>),
    Product(Vec<u64>),
}

impl CephalopodProblem {
    fn evaluate(&self) -> u64 {
        match self {
            CephalopodProblem::Product(nums) => nums.iter().product(),
            CephalopodProblem::Sum(nums) => nums.iter().sum(),
        }
    }
}

impl FromInput for Vec<Problem> {
    fn from_input(input: impl AsRef<str>) -> Self {
        let lines: Vec<Vec<&str>> = input
            .as_ref()
            .lines()
            .map(|l| l.split_ascii_whitespace().collect())
            .collect();

        let mut problems = Vec::new();

        for i in 0..(&lines).last().unwrap().len() {
            let mut nums = Vec::new();
            for vec in &lines {
                if let Ok(num) = vec[i].parse::<u64>() {
                    nums.push(num);
                } else if vec[i] == "*" {
                    problems.push(Problem::Product(nums));
                    break;
                } else if vec[i] == "+" {
                    problems.push(Problem::Sum(nums));
                    break;
                } else {
                    panic!("Invalid input");
                }
            }
        }
        problems
    }
}

impl FromInput for Vec<CephalopodProblem> {
    fn from_input(input: impl AsRef<str>) -> Self {
        let lines: Vec<Vec<char>> = input
            .as_ref()
            .lines()
            .map(|l| l.chars().collect())
            .collect();

        let rows = lines.len();

        let columns = lines.iter().map(|vec| vec.len()).max().unwrap();

        let mut iters: Vec<_> = lines.into_iter().map(|row| row.into_iter()).collect();

        let transposed: Vec<Vec<char>> = (0..columns)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|row| row.next().unwrap_or(' '))
                    .collect::<Vec<char>>()
            })
            .collect();

        let mut problems = Vec::new();
        let mut nums = Vec::new();

        for column in transposed.into_iter().rev() {
            let first = &column[0..rows - 1];
            let last = column[rows - 1];

            if let Ok(num) = first.iter().collect::<String>().trim().parse::<u64>() {
                nums.push(num);
            } else {
                continue;
            }

            if last == '*' {
                problems.push(CephalopodProblem::Product(nums));
                nums = Vec::new();
            } else if last == '+' {
                problems.push(CephalopodProblem::Sum(nums));
                nums = Vec::new();
            }
        }
        problems
    }
}

impl Solve for Day<6> {
    type PartOneData = Vec<Problem>;
    type PartTwoData = Vec<CephalopodProblem>;

    fn part_1(problems: &Self::PartOneData) -> String {
        problems
            .iter()
            .map(|problem| problem.evaluate())
            .sum::<u64>()
            .to_string()
    }

    fn part_2(problems: &Self::PartTwoData) -> String {
        problems
            .iter()
            .map(|problem| problem.evaluate())
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const INPUT: &str = "\
        123 328  51 64\n \
        45 64  387 23\n  \
        6 98  215 314\n\
        *   +   *   +  ";

    test!(day 6, parse_part_1: Vec<Problem>;
        INPUT => vec![
            Problem::Product(vec![123, 45, 6]),
            Problem::Sum(vec![328, 64, 98]),
            Problem::Product(vec![51, 387, 215]),
            Problem::Sum(vec![64, 23, 314])
        ]
    );

    test!(day 6, parse_part_2: Vec<CephalopodProblem>;
        INPUT => vec![
            CephalopodProblem::Sum(vec![4, 431, 623]),
            CephalopodProblem::Product(vec![175, 581, 32]),
            CephalopodProblem::Sum(vec![8, 248, 369]),
            CephalopodProblem::Product(vec![356, 24, 1]),
        ]
    );

    test!(day 6, part 1; INPUT => String::from("4277556"));

    test!(day 6, part 2; INPUT => String::from("3263827"));
}
