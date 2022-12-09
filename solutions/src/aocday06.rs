use nom::{InputTake, IResult};
use nom::character::complete::anychar;
use nom::multi::many_till;
use nom::sequence::tuple;

fn is_unique_lowercase4(bytes: &[u8]) -> bool {
    let num: usize = bytes.take(4).iter().fold(0,|a,b| a | (1usize<<b));
    num.count_ones()==4
}
fn is_unique_lowercase14(bytes: &[u8]) -> bool {
    let num: usize = bytes.take(14).iter().fold(0,|a,b| a | (1usize<<b));
    num.count_ones()==14
}

fn parse_unique4(input:&[u8]) -> IResult<&[u8],&[u8]> {
    // let mut set : HashSet<&u8> = input.iter().take(4).collect();
    // if set.len()==4 {
    if is_unique_lowercase4(input.take(4)) {
        Ok((&input[4..], input.take(4)))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(input,nom::error::ErrorKind::Satisfy)))
    }
}

fn parse_unique14(input:&[u8]) -> IResult<&[u8],&[u8]> {
    if is_unique_lowercase14(input.take(14)) {
        Ok((&input[4..], input.take(4)))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(input,nom::error::ErrorKind::Satisfy)))
    }
}

pub fn solve_pt1(input: &[u8]) -> Result<usize, &str> {
    if let Ok((_leftover, (char_vec, _res))) = many_till(anychar,parse_unique4)(input) {
        // println!(" {} {:?} {:?}", char_vec.len()+4, String::from_utf8_lossy(_res),String::from_utf8_lossy(_leftover));
        Ok(char_vec.len()+4)
    } else {
        Err("parse_pt1 error")
    }
}

pub fn solve_pt2(input: &[u8]) -> Result<usize, &str> {
    if let Ok((_leftover, (char_vec, _res))) = many_till(anychar,parse_unique14)(input) {
        // println!(" {} {:?} {:?}", char_vec.len()+4, String::from_utf8_lossy(_res),String::from_utf8_lossy(_leftover));
        Ok(char_vec.len()+14)
    } else {
        Err("parse_pt2 error")
    }
}

pub fn solve_both(input: &[u8]) -> Result<(usize,usize), &str> {
    if let Ok((_leftover, ((char_vec_4, _four), (char_vec_14, _fourteen)))) = tuple(
        (many_till(anychar,parse_unique4),many_till(anychar,parse_unique14))
    )(input) {
        Ok((char_vec_4.len()+4, char_vec_4.len()+4+char_vec_14.len()+14))
    } else {
        Err("parse_both error")
    }
}

pub fn solve_and_print() {
    let puzzle = include_str!("../../AOCDay06.txt");
    if let Ok(count) = solve_pt1(puzzle.as_bytes()) {
        println!("part 1 solution: {}", count);
    }
    if let Ok(count) = solve_pt2(puzzle.as_bytes()) {
        println!("part 2 solution: {}", count);
    }
    if let Ok((count1, count2)) = solve_both(puzzle.as_bytes()) {
        println!("part 1 solution: {} part 2 solution: {}", count1, count2);
    }
}