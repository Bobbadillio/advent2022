use nom::{
    character::complete::newline,
    combinator::{opt,map_res},
    bytes::complete::{take_while,tag},
    multi::fold_many1,
    sequence::tuple,
    IResult,
};

use std::num::ParseIntError;
use std::time::Instant;

fn from_digits(input: &str) -> Result<u16, ParseIntError> {
    input.parse::<u16>()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn digit_parser(input: &str) -> IResult<&str, u16> {
    map_res(take_while(is_digit), from_digits)(input)
}

fn line_to_containment(input: &str) -> IResult<&str, u16> {
    let (leftover, (a,_,b,_,c,_,d,_)) = tuple(
        (digit_parser, tag("-"), digit_parser, tag(","),digit_parser, tag("-"), digit_parser, opt(newline))
    )(input)?;
    let is_contained = match (a,b,c,d) {
        (a,b,c,d) if (((a<=c) &( d<=b)) | ((c<=a) & (b<=d))) => 1,
        _ => 0
    };
    Ok((leftover,is_contained))
}

fn line_to_overlap(input: &str) -> IResult<&str, u16> {
    let (leftover, (a,_,b,_,c,_,d,_)) = tuple(
        (digit_parser, tag("-"), digit_parser, tag(","),digit_parser, tag("-"), digit_parser, opt(newline))
    )(input)?;

    let is_overlap = match (a, b, c, d) {
        (a,b,c,d) if ((c<=b) & (d>=a)) => 1,
        _ => 0
    };
    Ok((leftover, is_overlap))
}

fn fold_lines_to_containments(input : &str) -> IResult<&str, u16> {
    fold_many1(line_to_containment, ||0, |a,b| a+b)(input)
}

fn fold_lines_to_overlaps(input : &str) -> IResult<&str, u16 > {
    fold_many1(line_to_overlap, || 0, |a,b| a+b)(input)
}

pub fn solve_and_print() {
    println!("\nsolving day 04:");
    let t0= Instant::now();
    let puzzle = include_str!("../../AOCDay04.txt");

    let t1= Instant::now();
    match fold_lines_to_containments(puzzle) {
        Ok((_, res)) => {println!("solution pt1: {}", res)},
        _ => println!("error!")
    }

    let t2= Instant::now();

    match fold_lines_to_overlaps(puzzle) {
        Ok((_, res)) => {println!("solution pt2: {}", res)},
        _ => println!("error!")
    }

    let t3= Instant::now();
    println!("\nday 04 timing info:\nload: {}\npt1: {}\npt2: {}",
             t1.duration_since(t0).as_micros(),
             t2.duration_since(t1).as_micros(),
             t3.duration_since(t2).as_micros());
}

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}


