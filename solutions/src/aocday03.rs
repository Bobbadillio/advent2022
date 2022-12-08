use std::collections::HashSet;
use nom::{
    character::complete::newline,
    combinator::opt,
    sequence::tuple,
    IResult,
};

use std::time::Instant;
use nom::bytes::complete::take_while;
use nom::combinator::map_res;
use nom::multi::fold_many1;

#[derive(Debug)]
struct RuckSack {
}

impl RuckSack {
    fn check_overlap(charline: &str) -> Result<char,&str> {
        let midpoint = charline.len()/2;
        let mut chars = HashSet::with_capacity(charline.len());
        for each_byte in charline.chars().take(midpoint) {
            chars.insert(each_byte);
        }
        for each_byte in charline.chars().skip(midpoint).take(midpoint){
            if chars.contains(&each_byte) { return Ok(each_byte)}
        }
        Ok('a')
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
    let (leftover, (token, _)) = tuple(
        (map_res(take_while(is_not_newline), RuckSack::check_overlap),
         opt(newline))
    )(input)?;
    let priority = get_priority( Some(token));

    Ok((leftover,priority))
}

fn line_to_chars(input: &str) -> IResult<&str, &str> {
    let (leftover, (chars, _)) = tuple((take_while(is_not_newline), opt(newline)))(input)?;
    Ok((leftover, chars))
}

fn parse_trio_badge(input: &str) -> IResult<&str, u32> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
    }
    let (leftover, (a,b,c)) = tuple((line_to_chars, line_to_chars, line_to_chars))(input)?;
    let first_chars: HashSet<char> = a.chars().collect();
    let second_chars :HashSet<char> = b.chars().filter(|each_char| first_chars.contains(each_char)).collect();
    for each_char in c.chars() {
        if second_chars.contains(&each_char) {return Ok((leftover, get_priority(Some(each_char))))}
    }
    panic!("no common character");
}


pub fn solve_and_print() {
    println!("\nsolving day 03:");
    let t0= Instant::now();
    let puzzle = include_str!("../../AOCDay03.txt");
    // let puzzle = fs::read_to_string("./AOCDay03.txt").unwrap();

    let t1= Instant::now();
    let sum_priorities_1 = solve_part_1(puzzle);
    println!("solution pt 1: {}", sum_priorities_1);


    let t2= Instant::now();

    let sum_priorities_2 = solve_part_2(puzzle);
    println!("solution pt 2: {}", sum_priorities_2);

    let t3= Instant::now();
    println!("\nday 03 timing info:\nload: {}\npt1: {}\npt2: {}",
             t1.duration_since(t0).as_micros(),
             t2.duration_since(t1).as_micros(),
             t3.duration_since(t2).as_micros());
}

pub fn solve_part_1(puzzle: &str) -> u32 {
    if let Ok((_, result)) = fold_many1(line_to_priority, ||0, |a,b| a+b)(puzzle) {
        result
    } else {
        0
    }
}

pub fn solve_part_2(puzzle: &str) -> u32 {
    if let Ok((_, result)) = fold_many1(parse_trio_badge, ||0, |a,b| a+b)(puzzle) {
        result
    } else {
        0
    }
}
