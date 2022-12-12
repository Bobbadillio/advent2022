use std::collections::VecDeque;
use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{digit1, line_ending, not_line_ending};
use nom::combinator::{map_res, opt};
use nom::IResult;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, tuple};

struct Monkey {
    id: u16,
    items: VecDeque<u16>,
    operation: Box<dyn FnMut(u16)-> u16 >,
    test_div: u16,
    true_case: u16,
    false_case: u16
}

impl Monkey {
    fn show(&self) -> String {
        format!{"id: {} test_div: {} true_case: {} false_case: {} items {:?}",
            self.id, self.test_div, self.true_case, self.false_case, self.items}
    }

    fn inspect(&mut self) {
        while let Some(item) = self.items.pop_front() {
            println!("monkey {} found item with worry {}, soon to be {}", self.id, item, (self.operation)(item)/3);
        }
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (leftover, digits) = delimited(tag("Monkey "), map_res(digit1, u16::from_str), tag(":\n"))(input)?;
    // println!("parsed: {}", digits);
    let (leftover, starting_items) = delimited(
        tag("  Starting items:"),
        many1(delimited(tag(" "), map_res(digit1, u16::from_str), opt(tag(",")))),
        tag("\n"))(leftover)?;

    let (leftover, operation) = parse_operation(leftover)?;
    // println!("operation: {}", operation);
    let (leftover, test_div) = delimited(tag("  Test: divisible by "), map_res(digit1, u16::from_str), line_ending )(leftover)?;
    let (leftover, true_case) = delimited(tag("    If true: throw to monkey "), map_res(digit1, u16::from_str), line_ending )(leftover)?;
    let (leftover, false_case) = delimited(tag("    If false: throw to monkey "), map_res(digit1, u16::from_str), many0(line_ending) )(leftover)?;

    Ok((leftover,  Monkey{id: digits, items: VecDeque::from(starting_items), operation:operation, test_div, true_case, false_case}))
}

fn parse_operation(input: &str) -> IResult<&str, Box<dyn FnMut(u16) -> u16>> {
    let (leftover, (left, op, right)) = delimited(
        tag("  Operation: new = "),
        tuple((tag("old"), take(3usize), alt((digit1, tag("old"))))),
        line_ending)(input)?;
    let operation : Box<dyn FnMut(u16)->u16> = match (op, right) {
        (" * ", "old") => Box::new( |x | x*x),
        (" + ", "old") => Box::new( move |x | x+x),
        (" * ", string) => {
            let scalar = u16::from_str(string).unwrap_or(0);
            Box::new( move |x| x * scalar)
        },
        (_op, string) => {
            assert_eq!(_op, " + ", "_op must be + here");
            let scalar = u16::from_str(string).unwrap_or(0);
            Box::new( move |x| x + scalar)
        }
    };
    // let (leftover2,((left, op, right)) )= (operation)?;
    Ok((leftover, Box::new(operation)))
}

pub fn solve_and_print() {
    // let puzzle = include_str!("../../AOCDay10.txt");
    let (leftover, mut monkeys) = many1(parse_monkey)(EXAMPLE).unwrap();
    println!("{}", monkeys.iter().map(|each| each.show()).collect::<Vec<String>>().join("\n"));
    for each_monkey in monkeys.iter_mut() {
        each_monkey.inspect();
    }
}




const EXAMPLE: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";