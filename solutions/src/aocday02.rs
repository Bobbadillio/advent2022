use nom::{
    character::complete::{newline, anychar},
    combinator::opt,
    sequence::tuple,
    IResult,
};

use std::time::Instant;
use nom::bytes::complete::tag;
use nom::multi::fold_many1;

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

// fn lines_to_kinds_pt1(input: &str) -> IResult<&str, Vec<(RPSKind, RPSKind)>> {
//     many1(line_to_kinds_pt1)(input)
// }
//
// fn lines_to_kinds_pt2(input: &str) -> IResult<&str, Vec<(RPSKind, RPSKind)>> {
//     many1(line_to_kinds_pt2)(input)
// }

fn kind_pair_to_points((p1, p2) : &(RPSKind, RPSKind)) -> u16 {
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

pub fn solve_and_print() {
    println!("\nsolving day 02:");
    let t0= Instant::now();
    let puzzle = include_str!("../../AOCDay02.txt");

    let t1= Instant::now();
    let points = solve_part_1(puzzle);
    println!("pt1 solution: {}", points);

    let t2= Instant::now();
    let points = solve_part_2(puzzle);
    println!("pt2 solution: {:?}", points);

    let t3= Instant::now();
    println!("\nday 02 timing info:\nload: {}\npt1: {}\npt2: {}",
             t1.duration_since(t0).as_micros(),
             t2.duration_since(t1).as_micros(),
             t3.duration_since(t2).as_micros());
}

pub fn solve_part_2(puzzle: &str) -> u16 {
    if let Ok((_, result)) = fold_many1(line_to_kinds_pt2, ||0, |a,b| a+kind_pair_to_points(&b) )(puzzle) {
        result
    } else {
        0
    }
}

pub fn solve_part_1(puzzle: &str) -> u16 {
    if let Ok((_, result)) = fold_many1(line_to_kinds_pt1, ||0, |a,b| a+kind_pair_to_points(&b) )(puzzle) {
        result
    } else {
        0
    }
}
