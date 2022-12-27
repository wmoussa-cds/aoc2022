use std::collections::BTreeMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    *,
};

use std::{fmt::Display, ops::RangeInclusive};

pub fn solve_a(data: &[String]) -> i32 {
    let input = data.join("\n");

    let notable_cycles = [20, 60, 100, 140, 180, 220];
    let mut scores: BTreeMap<u32, i32> = BTreeMap::new();

    let (_, instructions) = instruction_set(&input).unwrap();

    let mut x: i32 = 1;
    let mut cycles: u32 = 0;

    for instruction in instructions.iter() {
        if notable_cycles.contains(&(cycles + 1)) {
            scores.insert(cycles + 1, (cycles as i32 + 1) * x);
        }

        if notable_cycles.contains(&(cycles + 2)) {
            scores.insert(cycles + 2, (cycles as i32 + 2) * x);
        }

        cycles += instruction.cycles();
        match instruction {
            Noop => {}
            Add(num) => {
                x += num;
            }
        };
    }
    scores.iter().map(|(_key, value)| value).sum::<i32>()
}

/*
noop
addx -1
*/
fn instruction_set(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, vecs) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Noop),
            preceded(tag("addx "), complete::i32).map(|num| Add(num)),
        )),
    )(input)?;

    Ok((input, vecs))
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}
use Instruction::*;

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Noop => 1,
            Add(_) => 2,
        }
    }
}

pub fn solve_b(data: &[String]) -> usize {
    let input = data.join("\n");

    let (_, instructions) = instruction_set(&input).unwrap();

    let computer = instructions
        .iter()
        .fold(Computer::new(), |mut computer, instruction| {
            computer.interpret(instruction);
            computer
        });

    print!("{}", computer.to_string());
    0
}

struct Computer {
    x: i32,
    cycles: u32,
    pixels: String,
}
impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pixels
                .chars()
                .chunks(40)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .join("\n")
        )
    }
}
impl Computer {
    fn new() -> Self {
        Computer {
            x: 1,
            cycles: 0,
            pixels: "".to_string(),
        }
    }
    fn sprite_range(&self) -> RangeInclusive<i32> {
        (self.x - 1)..=(self.x + 1)
    }
    fn interpret(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.cycles() {
            let cycle_guard = self.start_cycle();

            if cycle_guard
                .computer
                .sprite_range()
                .contains(&(cycle_guard.pixel as i32))
            {
                cycle_guard.computer.pixels.push_str("#");
            } else {
                cycle_guard.computer.pixels.push_str(".");
            }
        }

        match instruction {
            Noop => {}
            Add(num) => {
                self.x += num;
            }
        };
    }
    fn start_cycle(&mut self) -> Cycle {
        Cycle {
            cycle: self.cycles,
            pixel: self.cycles % 40,
            computer: self,
        }
    }
}

struct Cycle<'a> {
    cycle: u32,
    pixel: u32,
    computer: &'a mut Computer,
}
impl<'a> Drop for Cycle<'a> {
    fn drop(&mut self) {
        self.computer.cycles += 1;
    }
}
