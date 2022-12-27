use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn solve_a(data: &[String]) -> usize {
    let binding = data.join("\n");
    let (_, move_set) = moves(&binding).unwrap();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = HashSet::from([tail]);

    for head_move in move_set.iter() {
        match head_move {
            Direction::Left => {
                head.0 -= 1;
            }
            Direction::Right => {
                head.0 += 1;
            }
            Direction::Up => {
                head.1 += 1;
            }
            Direction::Down => {
                head.1 -= 1;
            }
        }
        let x_range_movement = (head.0 - 1)..=(head.0 + 1);
        let y_range_movement = (head.1 - 1)..=(head.1 + 1);

        // scan all touchpoints and check if tail is within the boundary
        let tail_is_connected = x_range_movement
            .cartesian_product(y_range_movement)
            .any(|tuple| tuple == tail);

        if !tail_is_connected {
            // move_tail, return to previous head position
            let mut new_tail = head.clone();
            match head_move {
                Direction::Left => {
                    new_tail.0 += 1;
                }
                Direction::Right => {
                    new_tail.0 -= 1;
                }
                Direction::Up => {
                    new_tail.1 -= 1;
                }
                Direction::Down => {
                    new_tail.1 += 1;
                }
            }
            tail = new_tail;
            tail_positions.insert(new_tail);
        }
    }

    tail_positions.len()
}

pub fn solve_b(data: &[String]) -> usize {
    0
}

fn moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, vecs) =
        separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;

    // use the vec macro syntax vec![element;repeater]
    let vecs = vecs
        .iter()
        .flat_map(|(dir, repeat)| vec![*dir; *repeat as usize])
        .collect();
    Ok((input, vecs))
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
        complete::char('U').map(|_| Direction::Up),
        complete::char('D').map(|_| Direction::Down),
    ))(input)?;
    Ok((input, dir))
}
