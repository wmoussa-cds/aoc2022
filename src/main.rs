mod day13;
mod utils;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let data = utils::read_lines("./data/day13.txt");
    println!("Part A: {}", day13::solve_a(&data));
    println!("Part B: {}", day13::solve_b(&data));
}
