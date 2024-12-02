use nom::{
    branch::alt,
    character::complete::{digit1, line_ending, space1},
    combinator::{eof, map_res},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(2);

fn num(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn report(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, num)(input)
}

fn report_lines(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(alt((line_ending, eof)), report)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, lines) = report_lines(input).unwrap();

    let res = lines
        .into_iter()
        .filter(|line| {
            // check if report is safe
            // must be all decreasing or increasing
            // diffs must be 1,2, or 3
            let mut increasing: Option<bool> = None;
            let mut prev: Option<&u32> = None;

            for y in line {
                // establish prev value
                if prev.is_none() {
                    prev = Some(y);
                    continue;
                }

                let x = prev.unwrap();

                // check diff between elements
                let diff = x.abs_diff(*y);
                if !(1..=3).contains(&diff) {
                    return false;
                }

                // handle inc/dec trend
                // early return if fail condition
                match increasing {
                    None => increasing = Some(y > x),
                    Some(true) => {
                        if x > y {
                            return false;
                        };
                    }
                    Some(false) => {
                        if y > x {
                            return false;
                        }
                    }
                };

                // assign next
                prev = Some(y);
            }

            true
        })
        .count();

    // bad error handling
    Some(res.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
