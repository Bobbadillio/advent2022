use std::char::ParseCharError;
use std::collections::BTreeSet;
use nom::{
    character::complete::{newline, anychar},
    combinator::opt,
    multi::{many1, many0},
    sequence::tuple,
    IResult,
};

use std::fs;
use std::io::ErrorKind;
use std::mem::take;
use std::ops::Not;
use nom::bytes::complete::{take_while, tag};
use nom::combinator::map_res;

const EXAMPLE : &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

#[derive(Debug)]
struct RuckSack {
    left: BTreeSet<char>,
    right: BTreeSet<char>

}

impl RuckSack {
    fn from_str(charline: &str) -> Result<RuckSack, ParseCharError> {
        let midpoint = charline.len()/2;
        let left_bytes = charline.chars().take(midpoint);
        let right_bytes = charline.chars().skip(midpoint).take(midpoint);
        let left: BTreeSet<char> = left_bytes.collect();
        let right: BTreeSet<char> = right_bytes.collect();
        Ok(RuckSack{left, right})
    }

    fn find_overlap(&self) -> Option<char> {
        let overlap = self.left.intersection(&self.right).next()?;
        Some(*overlap)
    }

    fn overlap_from_str(charline: &str) -> Option<char> {
        if let Ok(sack) = RuckSack::from_str(charline) {
            Some(sack.find_overlap()?)
        } else {
            None
        }
    }
}

fn is_not_newline(input: char) -> bool {
    input != '\n'
}

fn line_to_sack(input: &str) -> IResult<&str, RuckSack> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
    }
    let (leftover, (sack, _)) = tuple((map_res(take_while(is_not_newline), RuckSack::from_str), opt(newline)))(input)?;
    Ok((leftover,sack))
}

fn line_to_overlap(input: &str) -> IResult<&str, Option<char>> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
    }
    let (leftover, (sack, _)) = tuple((map_res(take_while(is_not_newline), RuckSack::from_str), opt(newline)))(input)?;

    Ok((leftover,sack.find_overlap()))
}

fn line_to_priority(input: &str) -> IResult<&str, u32> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
    }
    let (leftover, (sack, _)) = tuple((map_res(take_while(is_not_newline), RuckSack::from_str), opt(newline)))(input)?;
    let priority = match sack.find_overlap() {
        None => 0,
        Some(a_char @ 'a'..='z') => a_char as u32 - 96,
        Some(a_char @ 'A'..='Z') => a_char as u32 - 38,
        _ => 0,
    };
    Ok((leftover,priority))
}

fn lines_to_sacks(input: &str) -> IResult<&str, Vec<RuckSack>> {
    let (leftover, sacks) = many0(line_to_sack)(input)?;
    Ok((leftover, sacks))

}

fn lines_to_overlaps(input: &str) -> IResult<&str, Vec<Option<char>>> {
    many0(line_to_overlap)(input)
}

fn lines_to_priorities(input: &str) -> IResult<&str, Vec<u32>> {
    many0(line_to_priority)(input)
}

fn main() {
    // println!("{:?}",line_to_sack(EXAMPLE));
    // println!("{:?}",lines_to_overlaps(EXAMPLE));
    if let Ok((_, result) ) = lines_to_sacks(EXAMPLE) {
        println!("{:?}",result);
    }
    if let Ok((_, result) ) = lines_to_overlaps(EXAMPLE) {
        println!("{:?}",result);
    }
    if let Ok((_, result) ) = lines_to_priorities(EXAMPLE) {
        println!("{:?}",result);
    }
    let puzzle = fs::read_to_string("./AOCDay03.txt").unwrap();

    if let Ok((leftover, priorities)) = lines_to_priorities(&puzzle) {
        println!("{}", priorities.iter().sum::<u32>())
    }

}
