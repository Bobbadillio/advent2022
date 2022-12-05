use std::char::ParseCharError;
use std::collections::{BTreeSet, HashSet};
use nom::{
    character::complete::newline,
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};

use std::fs;
use std::hash::Hash;
use std::time::Instant;
use nom::bytes::complete::take_while;
use nom::combinator::map_res;

#[derive(Debug)]
struct RuckSack {
    left: HashSet<char>,
    right: HashSet<char>

}

impl RuckSack {
    fn from_str(charline: &str) -> Result<RuckSack, ParseCharError> {
        let midpoint = charline.len()/2;
        let left_bytes = charline.chars().take(midpoint);
        let right_bytes = charline.chars().skip(midpoint).take(midpoint);
        let left: HashSet<char> = left_bytes.collect();
        let right: HashSet<char> = right_bytes.collect();
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

pub fn solve_and_print() {
    println!("\nsolving day 03:");
    let t0= Instant::now();
    let puzzle = include_str!("../../AOCDay03.txt");
    // let puzzle = fs::read_to_string("./AOCDay03.txt").unwrap();

    let t1= Instant::now();
    if let Ok((_, priorities)) = lines_to_priorities(puzzle) {
        println!("solution pt 1: {}", priorities.iter().sum::<u32>())
    }

    let t2= Instant::now();
    if let Ok((_, badge_priorities)) = parse_trio_badges(puzzle) {
        println!("solution pt 2: {}", badge_priorities.iter().sum::<u32>())
    };

    let t3= Instant::now();
    println!("\nday 03 timing info:\nload: {}\npt1: {}\npt2: {}",
             t1.duration_since(t0).as_micros(),
             t2.duration_since(t1).as_micros(),
             t3.duration_since(t2).as_micros());
}
