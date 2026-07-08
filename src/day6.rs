use std::str::SplitAsciiWhitespace;

use crate::util::*;

pub const SOLUTIONS: &[&dyn Solver] = &[
    &Solution::new(6, Part::One, &parse_part_one, &solve),
    &Solution::new(6, Part::Two, &parse_part_two, &solve),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    Sum(Vec<u64>),
    Product(Vec<u64>),
}

impl Problem {
    // solve the given problem
    fn evaluate(&self) -> u64 {
        match self {
            Problem::Product(nums) => nums.iter().product(),
            Problem::Sum(nums) => nums.iter().sum(),
        }
    }
}

// parse input into list of regular problems
fn parse_part_one(input: &str) -> Result<Vec<Problem>, String> {
    // split into 2d array of numbers/operations
    let mut lines: Vec<_> = input.lines().map(str::split_ascii_whitespace).collect();

    // iterate over columns by repeatedly advancing each iterator
    std::iter::from_fn(|| lines.iter_mut().map(SplitAsciiWhitespace::next).collect())
        .map(helpers::parse_regular_problem) // parse each column
        .collect()
}

// parse input into list of cephalopod problems
fn parse_part_two(input: &str) -> Result<Vec<Problem>, String> {
    // 2d grid of characters
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // the maximum width of the rows of the input, so that parsing works correctly without trailing spaces
    let width = lines
        .iter()
        .map(Vec::len)
        .max()
        .ok_or("input must contain at least one row")?;

    // transpose the character array by repeatedly advancing iterators for each row
    let mut iters: Vec<_> = lines.into_iter().map(Vec::into_iter).collect();
    let transposed: Vec<Vec<char>> = (0..width)
        .map(|_| {
            iters
                .iter_mut()
                .map(|row| row.next().unwrap_or(' '))
                .collect::<Vec<char>>()
        })
        .collect();

    // list of problems
    let mut problems = Vec::new();

    // parse the columns into a list of problems, adding a problem every time an operation is reached
    let mut args = Vec::new(); // list of args for the current problem
    for column in transposed.into_iter().rev() {
        let [arg_chars @ .., op_char] = column.as_slice() else {
            return Err("input must contain at least one row".into());
        };

        // collect the digits in the column and parse
        let arg_str = String::from_iter(arg_chars);
        let trimmed = arg_str.trim();

        // add this column's number to the list of numbers
        if !trimmed.is_empty() {
            args.push(parse_int(trimmed)?);
        }

        // if the operator char is '+' or '*' then we have reached the end of the block and can now collect the nums into the arguments for a problem
        match *op_char {
            ' ' => {}
            '*' => problems.push(Problem::Product(std::mem::take(&mut args))),
            '+' => problems.push(Problem::Sum(std::mem::take(&mut args))),
            op => return Err(format!("no such operation '{op}'")),
        };
    }

    Ok(problems)
}

// solve each problem and return the total
fn solve(problems: Vec<Problem>) -> Result<u64, String> {
    Ok(problems.iter().map(Problem::evaluate).sum::<u64>())
}

mod helpers {
    use super::*;

    // parse a list of arguments terminated by an operation into a problem
    pub fn parse_regular_problem(problem_str: Vec<&str>) -> Result<Problem, String> {
        // split into arguments and an operation
        let [arg_strs @ .., op_str] = problem_str.as_slice() else {
            return Err("input must contain at least one row".into());
        };

        // parse arguments
        let args = arg_strs
            .iter()
            .copied()
            .map(parse_int)
            .collect::<Result<Vec<u64>, String>>()?;

        // parse operation and return problem
        match *op_str {
            "*" => Ok(Problem::Product(args)),
            "+" => Ok(Problem::Sum(args)),
            op => Err(format!("no such operation '{op}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        123 328  51 64\n \
         45 64  387 23\n  \
          6 98  215 314\n\
        *   +   *   +  ";

    #[test]
    fn test_parse_part_one() {
        let expected = vec![
            Problem::Product(vec![123, 45, 6]),
            Problem::Sum(vec![328, 64, 98]),
            Problem::Product(vec![51, 387, 215]),
            Problem::Sum(vec![64, 23, 314]),
        ];
        assert_eq!(Ok(expected), parse_part_one(INPUT));
    }

    #[test]
    fn test_parse_part_two() {
        let expected = vec![
            Problem::Sum(vec![4, 431, 623]),
            Problem::Product(vec![175, 581, 32]),
            Problem::Sum(vec![8, 248, 369]),
            Problem::Product(vec![356, 24, 1]),
        ];
        assert_eq!(Ok(expected), parse_part_two(INPUT));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(Ok(4277556), parse_part_one(INPUT).and_then(solve));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(Ok(3263827), parse_part_two(INPUT).and_then(solve));
    }
}
