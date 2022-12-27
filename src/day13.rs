use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};
use std::{
    cmp::Ordering::{self, *},
    fmt::Display,
};

pub fn solve_a(data: &[String]) -> usize {
    let input = data.join("\n");
    let (_, pairs) = pairs(&input).unwrap();

    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, Pair { left, right })| match left.cmp(right) {
            Less => Some(i),
            Equal => panic!("equal??"),
            Greater => None,
        })
        .map(|v| v + 1)
        .sum::<usize>()
}

/*
[[1],[2,3,4]]
[[1],4]
 */
pub fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)
}

/*
recursive implementation
[[1],[2,3,4]]
 */
pub fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom::character::complete::u32.map(|num| Packet::Number(num)),
    ))(input)
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Packet::List(list) => format!(
                    "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Packet::Number(num) => num.to_string(),
            }
        )
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::Number(r0)) => l0 == &vec![Packet::Number(*r0)],
            (Self::Number(l0), Self::List(r0)) => &vec![Packet::Number(*l0)] == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(&b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}

pub fn solve_b(data: &[String]) -> usize {
    0
}
