use nom::{
    character::complete::{newline, anychar},
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};

use std::fs;
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

fn main() {
    let puzzle = fs::read_to_string("./AOCDay02.txt").unwrap();

    let (leftover, kinds) = lines_to_kinds_pt1(&puzzle).unwrap();
    println!("part 1 points: {} with leftover [{}]", kinds_to_points(&kinds), leftover);

    let (leftover_pt2, kinds2) = lines_to_kinds_pt2(&puzzle).unwrap();
    println!("{:?} with leftover [{}]", kinds_to_points(&kinds2), leftover_pt2);
}
