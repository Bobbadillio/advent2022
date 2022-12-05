use nom::{
    bytes::complete::take_while,
    character::complete::newline,
    combinator::{map_res, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::fs;
use std::time::Instant;

fn str_to_int(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn opt_newline(input: &str) -> IResult<&str, Option<char>> {
    opt(newline)(input)
}
fn from_decimal_line(input: &str) -> IResult<&str, u32> {
    let (leftover, (digits, _)) =
        tuple((map_res(take_while(is_digit), str_to_int), opt_newline))(input)?;
    Ok((leftover, digits))
}

fn calories(input: &str) -> IResult<&str, u32> {
    let (leftover, (vec, _)) = tuple((many1(from_decimal_line), opt_newline))(input)?;
    let calories = vec.iter().map(|each_int| *each_int as u32).sum();
    Ok((leftover, calories))
}

fn all_calories(input: &str) -> IResult<&str, Vec<u32>> {
    let (leftover, (vec, _)) = tuple((many1(calories), opt_newline))(input)?;
    Ok((leftover, vec))
}

pub fn solve_and_print() {
    println!("\nsolving day 01:");
    let t0= Instant::now();
    let puzzle = include_str!("../../AOCDay01.txt");

    let t1= Instant::now();
    let (_, mut all_calorie_result) = all_calories(puzzle).unwrap();

    println!("part 1 {:?}", all_calorie_result.iter().max().unwrap());

    let t2= Instant::now();
    if let ([a, b], c, ..) = all_calorie_result.select_nth_unstable_by(2, |a, b| b.cmp(a)) {
        println!("part 2 {} (from {} {} {})", *a + *b + *c, a, b, c)
    }

    let t3= Instant::now();
    println!("\nday 01 timing info:\nload: {}\npt1: {}\npt2: {}",
             t1.duration_since(t0).as_micros(),
             t2.duration_since(t1).as_micros(),
             t3.duration_since(t2).as_micros());
}
