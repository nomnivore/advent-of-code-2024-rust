use nom::character::complete::{anychar, digit1};
use nom::combinator::{map_res, opt, value};
use nom::multi::{many0, many_till, separated_list1};
use nom::Parser;
use nom::{
    bytes::complete::tag,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(3);

// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

type MulFn = (u32, u32);

fn num(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn num_pair(input: &str) -> IResult<&str, MulFn> {
    separated_pair(num, tag(","), num)(input)
}

fn mul(input: &str) -> IResult<&str, MulFn> {
    preceded(tag("mul"), delimited(tag("("), num_pair, tag(")")))(input)
}

fn parse_one(input: &str) -> IResult<&str, Vec<MulFn>> {
    many0(many_till(value((), anychar), mul).map(|(_, ab)| ab))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok((_, result)) = parse_one(input) {
        Some(result.into_iter().fold(0u32, |acc, (a, b)| acc + a * b))
    } else {
        None
    }
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
        assert_eq!(result, Some(161u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
