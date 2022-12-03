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
    println!("input len is {}", input.len());
    let (leftover, (sack, _)) = tuple((map_res(take_while(is_not_newline), RuckSack::from_str), opt(newline)))(input)?;
    println!("returning sack {:?}", sack);
    // let (leftover, (sack,_)) = tuple((RuckSack::from_str, opt(newline)))(input)?;
    Ok((leftover,sack))
}

fn line_to_overlap(input: &str) -> IResult<&str, Option<char>> {
    println!("input len is {}", input.len());
    // println!("[{}]",input);
    let (leftover, (sack, _)) = tuple((map_res(take_while(is_not_newline), RuckSack::from_str), opt(newline)))(input)?;
    // let (leftover, (sack,_)) = tuple((RuckSack::from_str, opt(newline)))(input)?;
    println!("returning overlap {:?}", sack.find_overlap());
    Ok((leftover,sack.find_overlap()))
}

fn lines_to_sacks(input: &str) -> IResult<&str, Vec<RuckSack>> {
    let (leftover, sacks) = many0(line_to_sack)(input)?;
    Ok((leftover, sacks))

}

fn lines_to_overlaps(input: &str) -> IResult<&str, Vec<Option<char>>> {
    many0(line_to_overlap)(input)
}

// fn line_to_overlap(input: &str) -> IResult<&str, &u8> {
//     let (leftover, chars) = take_while(is_not_newline)(input)?;
//     let (leftover, _ ) = opt(newline);
//     let sack = RuckSack::from_str(chars)?;
//     Ok((leftover, sack.find_overlap()))
// }

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
    // println!("{:?}",line_to_sack(EXAMPLE));
    // println!("{:?}",lines_to_overlaps(EXAMPLE));
    println!("{:?}",lines_to_sacks(EXAMPLE));
    let puzzle = fs::read_to_string("./AOCDay03.txt").unwrap();

    use nom::{IResult, multi::many0, bytes::complete::tag};
    use std::str;

    fn mytag(i: &str) -> IResult<&str, &str>{
        println!("multitag {}", i.len());
        tag("abcd")(i)
    }

    fn multi(i: &str) -> IResult<&str, Vec<&str>> {
        many0(mytag)(i)
    }

    let a = "abcdef";
    let b = "abcdabcdef";
    let c = "azerty";
    let d = "abcdabcd";
    // println!("{:?}, {:?}, {:?}, {:?}", multi(a),multi(b),multi(c),multi(d));

}
