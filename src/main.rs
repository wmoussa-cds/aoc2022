mod day6;
mod utils;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let data = utils::read_lines("./data/day6.txt");
    println!("Part A: {}", day6::solve_a(&data));
    println!("Part B: {}", day6::solve_b(&data));
}
