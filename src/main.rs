use std::char::ParseCharError;
use std::collections::BTreeSet;
use nom::{
    character::complete::newline,
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};

use std::fs;
use nom::bytes::complete::take_while;
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

fn get_priority(a_token: Option<char> ) -> u32 {
    match a_token{
        None => 0,
        Some(a_char @ 'a'..='z') => a_char as u32 - 96,
        Some(a_char @ 'A'..='Z') => a_char as u32 - 38,
        _ => 0
    }
}

fn line_to_priority(input: &str) -> IResult<&str, u32> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
    }
    let (leftover, (sack, _)) = tuple((map_res(take_while(is_not_newline), RuckSack::from_str), opt(newline)))(input)?;
    let priority = get_priority( sack.find_overlap());

    Ok((leftover,priority))
}

fn lines_to_sacks(input: &str) -> IResult<&str, Vec<RuckSack>> {
    let (leftover, sacks) = many1(line_to_sack)(input)?;
    Ok((leftover, sacks))

}

fn lines_to_overlaps(input: &str) -> IResult<&str, Vec<Option<char>>> {
    many1(line_to_overlap)(input)
}

fn lines_to_priorities(input: &str) -> IResult<&str, Vec<u32>> {
    many1(line_to_priority)(input)
}

fn line_to_chars(input: &str) -> IResult<&str, BTreeSet<char>> {
    let (leftover, (chars, _)) = tuple((take_while(is_not_newline), opt(newline)))(input)?;
    let a_set :BTreeSet<char> = chars.chars().collect();
    Ok((leftover, a_set))
}

fn parse_trio_badge(input: &str) -> IResult<&str, u32> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
    }
    let (leftover, (a,b,c)) = tuple((line_to_chars, line_to_chars, line_to_chars))(input)?;
    let first = a.intersection(&b);

    for each in first {
        if c.contains(each) {
            return Ok((leftover,get_priority(Some(*each))));
        }
    }
    panic!("no common character");
}

fn parse_trio_badges(input: &str) -> IResult<&str, Vec<u32>> {
    many1(parse_trio_badge)(input)
}

fn main() {
    // println!("{:?}",line_to_sack(EXAMPLE));
    // println!("{:?}",lines_to_overlaps(EXAMPLE));
    // if let Ok((_, result) ) = parse_trio_badge(EXAMPLE) {
    //     println!("{:?}",result);
    // }
    //
    // if let Ok((_, result) ) = line_to_chars(EXAMPLE) {
    //     println!("{:?}",result);
    // }
    // if let Ok((_, result) ) = lines_to_sacks(EXAMPLE) {
    //     println!("{:?}",result);
    // }
    // if let Ok((_, result) ) = lines_to_overlaps(EXAMPLE) {
    //     println!("{:?}",result);
    // }
    // if let Ok((_, result) ) = lines_to_priorities(EXAMPLE) {
    //     println!("{:?}",result);
    // }
    let puzzle = fs::read_to_string("./AOCDay03.txt").unwrap();

    if let Ok((_, priorities)) = lines_to_priorities(&puzzle) {
        println!("{}", priorities.iter().sum::<u32>())
    }

    if let Ok((_, badge_priorities)) = parse_trio_badges(&puzzle) {
        println!("{}", badge_priorities.iter().sum::<u32>())
    };

}
