use std::collections::HashSet;

pub fn solve_a(data: &[String]) -> usize {
    find_marker(&data[0], 4)
}

pub fn solve_b(data: &[String]) -> usize {
    find_marker(&data[0], 14)
}

fn find_marker(input: &str, window_size: usize) -> usize {
    let result = input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .map(|window| -> HashSet<char> { HashSet::from_iter(window.to_vec()) })
        .position(|window| window.len() == window_size);
    result.unwrap() + window_size
}
