use nom::{
    branch::alt,
    character::complete::{digit1, line_ending, space1},
    combinator::eof,
    multi::fold_many0,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(1);

fn into_vecs(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    fold_many0(
        terminated(
            separated_pair(digit1, space1, digit1),
            alt((line_ending, eof)),
        ),
        || (vec![], vec![]),
        |mut acc: (Vec<u32>, Vec<u32>), (left, right): (&str, &str)| {
            // add to each array
            acc.0.push(left.parse().unwrap());
            acc.1.push(right.parse().unwrap());

            acc
        },
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (mut left, mut right)) = into_vecs(input).unwrap();

    left.sort();
    right.sort();

    let result = left.into_iter().zip(right).fold(0, |acc, (a, b)| {
        // calc distance
        acc + a.abs_diff(b)
    });

    Some(result)
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
        assert_eq!(result, Some(11u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
