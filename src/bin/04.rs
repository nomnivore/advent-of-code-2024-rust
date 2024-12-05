use nom::{
    branch::alt,
    character::complete::{anychar, char, line_ending},
    combinator::eof,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(4);

type WordSearch = Vec<Vec<char>>;

fn line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(
        many1(alt((char('X'), char('M'), char('A'), char('S')))),
        alt((line_ending, eof)),
    )(input)
}

fn word_search(input: &str) -> IResult<&str, WordSearch> {
    many1(line)(input)
}

type Direction = (i32, i32);

const DIRECTIONS: [Direction; 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

const WORD: &str = "XMAS";

fn adjusted_indexes(needle: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    let x = if dir.0 >= 0 {
        needle.0.checked_add(dir.0 as usize)
    } else {
        let abs_dir = (-dir.0) as usize;
        needle.0.checked_sub(abs_dir)
    };
    let y = if dir.1 >= 0 {
        needle.1.checked_add(dir.1 as usize)
    } else {
        let abs_dir = (-dir.1) as usize;
        needle.1.checked_sub(abs_dir)
    };

    if let Some(idx) = x.zip(y) {
        Some(idx)
    } else {
        None
    }
}

fn scan(
    haystack: &WordSearch,
    needle: (usize, usize),
    length: usize,
    search_dir: Option<Direction>,
) -> u32 {
    // scan neighboring cells for the next letter in the word
    // if length == word length, we've found the word
    // if a direction is specified, that direction is locked in for the remainder of that scan

    // base case
    if length == WORD.len() {
        return 1;
    }

    match search_dir {
        Some((dx, dy)) => {
            // dir is set
            let opt_adj = adjusted_indexes(needle, (dx, dy));
            if let Some(adj) = opt_adj {
                let opt_ele: Option<&char> = haystack.get(adj.0).and_then(|row| row.get(adj.1));

                if let Some(ele) = opt_ele {
                    if *ele == WORD.chars().nth(length).unwrap_or('.') {
                        return scan(haystack, adj, length + 1, Some((dx, dy)));
                    }
                }
            }
        }
        None => {
            // search all directions
            let mut count = 0;

            for (dx, dy) in DIRECTIONS {
                // get element if it exists/is in bounds
                let opt_adj = adjusted_indexes(needle, (dx, dy));
                if let Some(adj) = opt_adj {
                    let opt_ele: Option<&char> = haystack.get(adj.0).and_then(|row| row.get(adj.1));

                    if let Some(ele) = opt_ele {
                        if *ele == WORD.chars().nth(length).unwrap_or('.') {
                            // this branch is promising, check it out

                            let branch = scan(haystack, adj, length + 1, Some((dx, dy)));

                            count += branch;
                        }
                    }
                }
            }

            return count;
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, haystack) = word_search(input).unwrap();

    let mut found: u32 = 0;

    // get word-search width
    let width = haystack[0].len();

    for row in 0..haystack.len() {
        for col in 0..width {
            // this one for sure exists....right?
            let ch = haystack.get(row).and_then(|r| r.get(col)).unwrap();

            // only start if it matches the first letter of the word we're looking for
            if *ch == WORD.chars().nth(0).unwrap_or('.') {
                let check = scan(&haystack, (row, col), 1, None);
                found += check;
            }
        }
    }

    Some(found)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
