mod day02;
mod utils;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let data = utils::read_lines("./data/day02.txt");
    println!("Day 2 A: {}", day02::solve_a(&data));
    println!("Day 2 B: {}", day02::solve_b(&data));
}
