use nom::{
    character::complete::newline,
    combinator::{opt,map_res},
    bytes::complete::take_while,
    multi::many1,
    sequence::tuple,
    IResult,
};

use std::fs;
use std::num::ParseIntError;
use std::time::Instant;
use nom::bytes::complete::tag;

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
    }
        ;
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

fn lines_to_containments(input : &str) -> IResult<&str, Vec<u16>> {
    many1(line_to_containment)(input)
}

fn lines_to_overlaps(input : &str) -> IResult<&str, Vec<u16>> {
    many1(line_to_overlap)(input)
}

fn main() {
    let t0= Instant::now();
    let puzzle = fs::read_to_string("./AOCDay04.txt").unwrap();

    let t1= Instant::now();
    match lines_to_containments(&puzzle) {
        Ok((_, res)) => {println!("pt1: {}", res.iter().sum::<u16>())},
        _ => println!("error!")
    }

    let t2= Instant::now();
    match lines_to_overlaps(&puzzle) {
        Ok((_, res)) => {println!("pt2: {}", res.iter().sum::<u16>())},
        _ => println!("error!")
    }

    let t3= Instant::now();
    println!("timing info:\nload: {}\npt1: {}\npt2: {}",t1.duration_since(t0).as_micros(), t2.duration_since(t1).as_micros(),t3.duration_since(t2).as_micros());
}
