mod day5;
mod utils;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let data = utils::read_lines("./data/day5.txt");
    println!("Part A: {}", day5::solve_a(&data));
    println!("Part B: {}", day5::solve_b(&data));
}
