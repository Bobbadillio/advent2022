use std::collections::{HashMap, VecDeque};
use std::num::ParseIntError;
use std::str:: FromStr;
use nom::{IResult, character::{
    complete::anychar}
};
use nom::bytes::complete::tag;
use nom::character::complete::{ digit1, newline, not_line_ending};
use nom::combinator::{map, map_res, opt};
use nom::multi::{fold_many1, many1};
use nom::sequence::{ terminated, tuple};

fn match_line(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, opt(newline))(input)
    // if res.contains("[") {
    //     Ok((leftover, res))
    // } else {
    //     Err(nom::Err::Error(nom::error::Error::new(input,nom::error::ErrorKind::Satisfy)))
    // }
}

fn parse_stack_line(input: &str) -> Result<Vec<(usize,char)>,&str> {
    if input.contains('[') {
        Ok(input.chars().enumerate().filter(
            |(_, a_char)| a_char.is_alphabetic()).collect())
    } else {
        Err(input)
    }
}

fn parse_stack_lines(input: &str) -> IResult<&str, HashMap<usize, VecDeque<char>>> {
    fold_many1(map_res(match_line, parse_stack_line), HashMap::new,
    |mut stacks, b| {
        for (location, each_char) in b {
            let stack = stacks.entry(location).or_insert_with(|| VecDeque::with_capacity(64));
            stack.push_front(each_char);
        }
        stacks
    }
    )(input)


}

fn parse_stack_locations(input: &str) -> HashMap<usize, char> {
    input.chars().enumerate().filter(
        |(_, a_char)| a_char.is_alphanumeric()).collect()
}

#[derive(Debug)]
pub struct Order {
    count  : u8,
    source : char,
    dest   : char
}

#[derive(Debug, Clone)]
pub struct CrateStacks {
    stacks: HashMap<char, VecDeque<char>>,
    // stacks: HashMap<usize, VecDeque<char>>,
    // stack_mapping: HashMap<char, usize>
}
impl CrateStacks {
    fn new<'a>( stacks: &mut HashMap<usize, VecDeque<char>>, stack_mapping: IResult<&'a str, HashMap<usize, char>>) -> IResult<&'a str, CrateStacks> {
        let (leftover, stack_mapping) = stack_mapping?;
        Ok((
            leftover,
            CrateStacks{
                stacks:stacks.drain().map(
                    |(each_key, stack)| (
                        *stack_mapping.get(&each_key).unwrap_or(&'1'),
                        stack
                    )).collect()
            }
        ))
    }

    fn process_order(&mut self, an_order: &Order) -> Result<(), &str> {
        let Order{count, source, dest} = an_order;
        let source_stack= self.stacks.get_mut(source).ok_or("no source stack")?;
        let start_index = source_stack.len().saturating_sub(*count as usize);
        let transfer  = source_stack.drain(start_index..).collect::<VecDeque<_>>();
        let dest_stack = self.stacks.get_mut(dest).ok_or("no destination stack")?;
        dest_stack.extend(transfer.iter().rev());
        Ok(())
    }

    fn process_order_9001(&mut self, an_order: &Order) -> Result<(), &str> {
        let Order{count, source, dest} = an_order;
        let source_stack= self.stacks.get_mut(source).ok_or("no source stack")?;
        let start_index = source_stack.len().saturating_sub(*count as usize);
        let mut transfer  = source_stack.drain(start_index..).collect::<VecDeque<_>>();
        let dest_stack = self.stacks.get_mut(dest).ok_or("no destination stack")?;
        dest_stack.append(&mut transfer);
        Ok(())
    }

    fn get_tops(&self) -> Vec<char> {
        let mut stack_keys: Vec<&char> = self.stacks.keys().collect();
        stack_keys.sort();
        stack_keys.iter().map(|&each_key| {
            let stack = self.stacks.get(each_key);
            let top = stack.map(|a_vec| a_vec.back().unwrap_or(&'X'));
            *(top.unwrap_or(&'X'))
        }).collect()
    }
}

impl Order {
}

fn get_u8(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str(input)
}

fn parse_order(input: &str) -> IResult<&str,Order> {
    let (leftover, (_, count, _, source, _, dest, _)) = tuple((
        tag("move "), map_res(digit1, get_u8),
        tag(" from "), anychar,
        tag(" to "), anychar, opt(newline))
    )(input)?;
    Ok((
        leftover,
        Order{count, source, dest }
    ))
}

fn parse_stack_mapping(input: &str) -> IResult<&str, HashMap<usize,char>> {
    terminated(map(match_line, parse_stack_locations),tag("\n"))(input)
}

pub fn solve_pt1(the_stacks: &mut CrateStacks,  commands: &[Order]) -> String {
    for each_order in commands.iter() {
        the_stacks.process_order(each_order).ok();
    }
    the_stacks.get_tops().iter().collect()
}

pub fn solve_pt2(the_stacks: &mut CrateStacks,  commands: &[Order]) -> String {
    for each_order in commands.iter() {
        the_stacks.process_order_9001(each_order).ok();
    }
    the_stacks.get_tops().iter().collect()
}

pub fn parse_fully(input: &str) -> IResult<&str,(CrateStacks, Vec<Order>)> {
    let (leftover, mut stacks) = parse_stack_lines(input)?;
    let stack_map_res = parse_stack_mapping(leftover);
    let (leftover, the_stacks) = CrateStacks::new(&mut stacks, stack_map_res)?;
    let (leftover, commands) = many1(parse_order)(leftover)?;
    Ok((leftover, (the_stacks, commands)))
}

pub fn solve_and_print() {
    let puzzle = include_str!("../../AOCDay05.txt");
    if let Ok((_, (mut the_stacks, commands))) = parse_fully(puzzle){
        println!("{:?}", solve_pt1(&mut the_stacks.clone(), &commands));
        println!("{:?}", solve_pt2( &mut the_stacks, &commands));
    }
}