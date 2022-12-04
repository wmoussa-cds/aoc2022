pub fn solve_a(data: &[String]) -> i32 {
    data.iter().map(|s| find_dupe_a(s)).sum::<i32>()
}

fn find_dupe_a(round: &String) -> i32 {
    let left = round.split_at(round.chars().count() / 2).0;
    let right = round.split_at(round.chars().count() / 2).1;
    let mut dupe = '@';
    for c in left.chars() {
        if right.contains(c) {
            dupe = c;
            break;
        }
    }
    score_letter(dupe)
}

pub fn solve_b(data: &[String]) -> i32 {
    data.iter()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|f| find_dupe_b(f))
        .sum::<i32>()
}

fn find_dupe_b(round: &[&String]) -> i32 {
    let group_1 = round[0];
    let group_2 = round[1];
    let group_3 = round[2];
    let mut dupe = '@';
    for c in group_1.chars() {
        if group_2.contains(c) && group_3.contains(c) {
            dupe = c;
            break;
        }
    }
    score_letter(dupe)
}

fn score_letter(c: char) -> i32 {
    let alphabet: Vec<char> = ('a' as u8..'z' as u8 + 1)
        .chain('A' as u8..'Z' as u8 + 1)
        .map(|i| i as char)
        .collect();
    let index = alphabet.iter().position(|&r| r == c).unwrap();
    (index + 1) as i32
}
