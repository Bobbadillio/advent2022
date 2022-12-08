use nom::{
    bytes::complete::take_while,
    character::complete::newline,
    combinator::{map_res, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::time::Instant;
use nom::multi::fold_many1;

fn str_to_int(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn from_decimal_line(input: &str) -> IResult<&str, u32> {
    let (leftover, (calories, _)) =
        tuple((map_res(take_while(is_digit), str_to_int), opt(newline)))(input)?;
    Ok((leftover, calories))
}

fn calories(input: &str) -> IResult<&str, u32> {
    let (leftover, (contents, _) ) = tuple((fold_many1(from_decimal_line, || 0, |a,b| a+b ), opt(newline)))(input)?;
    Ok((leftover, contents))
}

pub fn solve_part_1(input: &str) -> u32 {
    if let Ok((_, result)) = fold_many1(calories, ||0, |a,b| a.max(b))(input){
        result
    } else {
        0
    }
}

fn bubble(mut a : [u32;4], new:u32) -> [u32;4]{
    // let first_mut = a.get_mut(0)?
    if let Some(first_el) = a.get_mut(0) {
        *first_el = new;
        a.sort();
        a
    } else {
        a
    }
}

pub fn solve_part_2(input: &str) -> u32 {
    if let Ok((_, result_vec)) = fold_many1(
        calories, || [0;4],bubble
    )(input) {
        result_vec.get(1).unwrap_or(&0)
            +result_vec.get(2).unwrap_or(&0)
            +result_vec.get(3).unwrap_or(&0)
    } else {
        0
    }
}

pub fn solve_both(input: &str) -> (u32,u32) {
    if let Ok((_, result_vec)) = fold_many1(
        calories, || [0;4],   bubble
    )(input) {
        (*result_vec.get(3).unwrap_or(&0), result_vec.get(1).unwrap_or(&0)
            +result_vec.get(2).unwrap_or(&0)
            +result_vec.get(3).unwrap_or(&0))
    } else {
        (0,0)
    }
}

fn all_calories(input: &str) -> IResult<&str, Vec<u32>> {
    let (leftover, (vec, _)) = tuple((many1(calories), opt(newline)))(input)?;
    Ok((leftover, vec))
}

pub fn solve_and_print() {
    println!("\nsolving day 01:");
    let t0= Instant::now();
    let puzzle = include_str!("../../AOCDay01.txt");

    let t1= Instant::now();

    // let mut all_calorie_result = parse_calories(puzzle);
    println!("part 1 {:?}", solve_part_1(puzzle));

    let t2= Instant::now();

    println!("part 2 {:?} (new!)",solve_part_2(puzzle));

    let t3= Instant::now();
    println!("part 1&2 {:?} (new!)",solve_both(puzzle));


    println!("\nday 01 timing info:\nload: {}\npt1: {}\npt2: {}",
             t1.duration_since(t0).as_micros(),
             t2.duration_since(t1).as_micros(),
             t3.duration_since(t2).as_micros());
}

pub fn parse_calories(puzzle: &str) -> Vec<u32> {
    let (_, all_calorie_result) = all_calories(puzzle).unwrap();
    all_calorie_result
}
