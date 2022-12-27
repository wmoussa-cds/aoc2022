use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult, Parser,
};

pub fn solve_a(data: &[String]) -> usize {
    let binding = data.join("\n");
    let (_, trees) = parse_trees(&binding).unwrap();
    let max_length = trees.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(i, tree_line)| {
            let line_max_length = tree_line.len() - 1;
            tree_line
                .iter()
                .enumerate()
                .map(|(line_i, _)| {
                    if i == 0 || i == max_length || line_i == 0 || line_i == line_max_length {
                        true
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect();

    for y in 0..trees.len() {
        let mut current_tree_size = 0;
        for x in 0..trees[0].len() {
            if x == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }
    for y in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for x in (0..trees[0].len()).rev() {
            if x == trees.len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    // Iterations for Ys
    for x in 0..trees.len() {
        let mut current_tree_size = 0;
        for y in 0..trees[0].len() {
            if y == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }
    for x in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for y in (0..trees[0].len()).rev() {
            if y == trees.len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    visible_trees.iter().flatten().filter(|&&v| v).count()
}

fn parse_trees(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, vecs) = separated_list1(
        newline,
        digit1.map(|nums: &str| nums.chars().map(|num| num.to_digit(10).unwrap()).collect()),
    )(input)?;

    Ok((input, vecs))
}

pub fn solve_b(data: &[String]) -> u32 {
    let binding = data.join("\n");
    let (_, trees) = parse_trees(&binding).unwrap();

    let mut high_score = 0;

    let y_max = trees.len();
    let x_max = trees[0].len();

    for (y_index, tree_line) in trees.iter().enumerate() {
        for (x_index, treehouse_height) in tree_line.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];

            for x_position in (0..x_index).rev() {
                if trees[y_index][x_position] < *treehouse_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }
            // to right
            for x_position in (x_index + 1)..x_max {
                if trees[y_index][x_position] < *treehouse_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            // to up
            for y_position in (0..y_index).rev() {
                if trees[y_position][x_index] < *treehouse_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }
            // to down
            for y_position in (y_index + 1)..y_max {
                if trees[y_position][x_index] < *treehouse_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }
            let scenic_score: u32 = scores.iter().product();

            if scenic_score > high_score {
                high_score = scenic_score
            }
        }
    }
    high_score
}
