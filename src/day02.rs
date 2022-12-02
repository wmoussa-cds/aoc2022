use std::ops::Deref;

pub fn solve_a(data: &[String]) -> i32 {
    data.iter().map(|s| compute(s)).sum::<i32>()
}

pub fn solve_b(data: &[String]) -> i32 {
    data.iter().map(|s| compute_b(s)).sum::<i32>()
}

fn compute(round: &String) -> i32 {
    match round.deref() {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

fn compute_b(round: &String) -> i32 {
    match round.deref() {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}
