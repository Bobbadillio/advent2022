use std::ops::Rem;
use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map_res, opt};
use nom::IResult;
use nom::multi::fold_many1;
use nom::sequence::{terminated, tuple};

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(i16)
}

#[derive(Debug)]
pub struct Computer {
    eax: isize,
    clock: isize,
    strength:isize,
    display: String
}

impl Computer {
    fn display(&self) {
        let to_show :String = self.display.as_bytes().chunks(40).map(|chars|
            String::from_utf8_lossy(chars)+"\n"
        ).collect();
        println!("{to_show}")
    }
    fn new() -> Computer {
        Computer{
            eax:1,
            clock:0,
            strength:0,
            display:String::from("")
        }
    }
    fn advance_clock (&mut self){
        if ((self.clock.rem(40))-self.eax).abs() <=1 {
            self.display.push('#')
        } else {
            self.display.push('.')
        }

        self.clock += 1;
        if self.clock.rem(40) == 20 {
            self.strength += self.clock*self.eax;
        }
    }
    fn process(&mut self, instruction: Instruction) -> &mut Computer {
        match instruction {
            Instruction::NOOP => self.advance_clock(),
            Instruction::ADDX(inc) => {
                self.advance_clock();
                self.advance_clock();
                self.eax += inc as isize;
            }
        }
        self
    }
}

fn parse_noop(input : &str) -> IResult<&str, Instruction> {
    let (leftover, _) = tag("noop")(input)?;
    Ok((leftover, Instruction::NOOP))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    // println!("found leftover [{}]", input);
    let (leftover, (_tag, sign, digit)) = tuple(
        (tag("addx "), opt(tag("-")), map_res(digit1, i16::from_str)))(input)?;
    match sign {
        Some("-") => Ok((leftover, Instruction::ADDX(-digit))),
        _ => Ok((leftover, Instruction::ADDX(digit))),
    }

}


pub fn solve_and_print() {
    let puzzle = include_str!("../../AOCDay10.txt");

    let computer = solve_both(puzzle);
    computer.display();
    println!("{:?}", computer);
}

pub fn solve_both(puzzle: &str) -> Computer {
    if let Ok((_, final_computer)) = fold_many1(
        terminated(alt((parse_instruction, parse_noop)), opt(newline)),
        Computer::new,
        |mut a, b| {
            a.process(b);
            a
        }
    )(puzzle) {
        // println!("{:?}", final_computer);
        final_computer
    } else {
        panic!("parse failure")
    }
}