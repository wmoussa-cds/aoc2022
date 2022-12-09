mod day8;
mod utils;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let data = utils::read_lines("./test_data/day8.txt");
    println!("Part A: {}", day8::solve_a(&data));
    println!("Part B: {}", day8::solve_b(&data));
}
