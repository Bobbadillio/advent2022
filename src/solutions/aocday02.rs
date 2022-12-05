use nom::{
    character::complete::{newline, anychar},
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};

use std::fs;
use std::time::Instant;
use nom::bytes::complete::tag;

#[derive(Debug)]
enum RPSKind {
    Rock,
    Paper,
    Scissor
}

fn line_to_kinds_pt1(input: &str) -> IResult<&str, (RPSKind, RPSKind)> {
    let (leftover, (p1_symbol,_,p2_symbol,_)) = tuple((anychar,tag(" "), anychar, opt(newline)))(input)?;
    let p1 = match p1_symbol {
        'A' => RPSKind::Rock,
        'B' => RPSKind::Paper,
        _ => RPSKind::Scissor
    };
    let p2 = match p2_symbol {
        'X' => RPSKind::Rock,
        'Y' => RPSKind::Paper,
        _ => RPSKind::Scissor
    };
    Ok((leftover,(p1,p2)))
}

fn line_to_kinds_pt2(input: &str) -> IResult<&str, (RPSKind, RPSKind)> {
    let (leftover, (p1_symbol,_,p2_symbol,_)) = tuple((anychar,tag(" "), anychar, opt(newline)))(input)?;
    let (p1, p2) = match (p1_symbol, p2_symbol) {
        ('A', 'X') => (RPSKind::Rock,    RPSKind::Scissor),
        ('A', 'Y') => (RPSKind::Rock,    RPSKind::Rock),
        ('A', 'Z') => (RPSKind::Rock,    RPSKind::Paper),
        ('B', 'X') => (RPSKind::Paper,   RPSKind::Rock),
        ('B', 'Y') => (RPSKind::Paper,   RPSKind::Paper),
        ('B', 'Z') => (RPSKind::Paper,   RPSKind::Scissor),
        ('C', 'X') => (RPSKind::Scissor, RPSKind::Paper),
        ('C', 'Y') => (RPSKind::Scissor, RPSKind::Scissor),
        ('C', 'Z') => (RPSKind::Scissor, RPSKind::Rock),
        _other => panic!("this shouldn't happen...")
    };
    Ok((leftover,(p1,p2)))
}

fn lines_to_kinds_pt1(input: &str) -> IResult<&str, Vec<(RPSKind, RPSKind)>> {
    many1(line_to_kinds_pt1)(input)
}

fn lines_to_kinds_pt2(input: &str) -> IResult<&str, Vec<(RPSKind, RPSKind)>> {
    many1(line_to_kinds_pt2)(input)
}

fn kind_pair_to_points((p1, p2) : (&RPSKind, &RPSKind)) ->u32 {
    let winning_points = match (p1,p2) {
        (RPSKind::Scissor, RPSKind::Rock   )| (RPSKind::Paper, RPSKind::Scissor)| (RPSKind::Rock, RPSKind::Paper) => 6,
        (RPSKind::Scissor, RPSKind::Scissor)| (RPSKind::Paper, RPSKind::Paper  )| (RPSKind::Rock, RPSKind::Rock ) => 3,
        _ => 0
    };
    let throw_points = match p2 {
        RPSKind::Rock => 1,
        RPSKind::Paper => 2,
        RPSKind::Scissor => 3
    };
    winning_points+throw_points
}

fn kinds_to_points(pairs : &[(RPSKind, RPSKind)]) ->u32 {
    pairs.iter().map(|(p1, p2) |kind_pair_to_points((p1, p2))).sum()
}

pub fn solve_and_print() {
    println!("\nsolving day 02:");
    let t0= Instant::now();
    let puzzle = include_str!("../../AOCDay02.txt");

    let t1= Instant::now();
    let (leftover, kinds) = lines_to_kinds_pt1(puzzle).unwrap();
    println!("pt1 solution: {} with leftover [{}]", kinds_to_points(&kinds), leftover);

    let t2= Instant::now();
    let (leftover_pt2, kinds2) = lines_to_kinds_pt2(puzzle).unwrap();
    println!("pt2 solution: {:?} with leftover [{}]", kinds_to_points(&kinds2), leftover_pt2);

    let t3= Instant::now();
    println!("\nday 02 timing info:\nload: {}\npt1: {}\npt2: {}",
             t1.duration_since(t0).as_micros(),
             t2.duration_since(t1).as_micros(),
             t3.duration_since(t2).as_micros());
}
