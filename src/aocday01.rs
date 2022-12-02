use nom::{
    bytes::complete::take_while,
    character::complete::newline,
    combinator::{map_res, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::fs;

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

fn main() {
    let puzzle = fs::read_to_string("./AOCDay01.txt").unwrap();
    let (_, mut all_calorie_result) = all_calories(&puzzle).unwrap();

    println!("part 1 {:?}", all_calorie_result.iter().max().unwrap());

    if let ([a, b], c, ..) = all_calorie_result.select_nth_unstable_by(2, |a, b| b.cmp(a)) {
        eprint!("part 2 {} (from {} {} {})", *a + *b + *c, a, b, c)
    } else {
        panic!("that should've worked...")
    }
}
