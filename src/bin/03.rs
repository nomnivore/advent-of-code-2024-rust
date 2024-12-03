use nom::branch::alt;
use nom::character::complete::{anychar, digit1};
use nom::combinator::{map, map_res, opt, value};
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

enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

fn num(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn num_pair(input: &str) -> IResult<&str, MulFn> {
    separated_pair(num, tag(","), num)(input)
}

fn fn_mul(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("mul"), delimited(tag("("), num_pair, tag(")"))),
        |(a, b)| Instruction::Mul(a, b),
    )(input)
}

fn fn_do(input: &str) -> IResult<&str, Instruction> {
    map(tag("do()"), |_| Instruction::Do)(input)
}

fn fn_dont(input: &str) -> IResult<&str, Instruction> {
    map(tag("don't()"), |_| Instruction::Dont)(input)
}

fn parse_one(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(many_till(value((), anychar), fn_mul).map(|(_, ab)| ab))(input)
}

fn parse_two(input: &str) -> IResult<&str, Vec<Instruction>> {
    // technically parse_one and parse_two could just be merged at this point

    many0(many_till(value((), anychar), alt((fn_mul, fn_do, fn_dont))).map(|(_, ab)| ab))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok((_, result)) = parse_one(input) {
        Some(result.into_iter().fold(0u32, |acc, inst| match inst {
            Instruction::Mul(a, b) => acc + a * b,
            _ => acc,
        }))
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    if let Ok((_, result)) = parse_two(input) {
        Some(
            result
                .into_iter()
                .fold((Instruction::Do, 0u32), |(flag, acc), inst| match inst {
                    Instruction::Mul(a, b) => match flag {
                        Instruction::Do => (flag, acc + a * b),
                        _ => (flag, acc),
                    },
                    Instruction::Do => (Instruction::Do, acc),
                    Instruction::Dont => (Instruction::Dont, acc),
                })
                .1,
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(input);
        assert_eq!(result, Some(48));
    }
}
