mod day7;
mod utils;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let data = utils::read_lines("./data/day7.txt");
    println!("Part A: {}", day7::solve_a(&data));
    println!("Part B: {}", day7::solve_b(&data));
}
