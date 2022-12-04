mod day4;
mod utils;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let data = utils::read_lines("./test_data/day4.txt");
    println!("Day 4 A: {}", day4::solve_a(&data));
    println!("Day 4 B: {}", day4::solve_b(&data));
}
