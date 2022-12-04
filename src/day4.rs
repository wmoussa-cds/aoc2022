pub fn solve_a(data: &[String]) -> i32 {
    data.iter().map(|pair| find_overlap(pair)).sum::<i32>()
}

fn find_overlap(pair: &String) -> i32 {
    let pair_1 = pair.split(",").collect::<Vec<&str>>()[0];
    let pair_2 = pair.split(",").collect::<Vec<&str>>()[1];

    let pair_1_first_id = pair_1.split("-").collect::<Vec<&str>>()[0]
        .parse::<u32>()
        .unwrap();
    let pair_1_second_id = pair_1.split("-").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let pair_2_first_id = pair_2.split("-").collect::<Vec<&str>>()[0]
        .parse::<u32>()
        .unwrap();
    let pair_2_second_id = pair_2.split("-").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let pair_1_total = pair_1_second_id - pair_1_first_id + 1;
    let pair_2_total = pair_2_second_id - pair_2_first_id + 1;

    if pair_1_total > pair_2_total {
        let mut cleaned = 0;
        for n in pair_2_first_id..pair_2_second_id + 1 {
            if n >= pair_1_first_id && n <= pair_1_second_id {
                cleaned += 1;
            }
        }
        if cleaned == pair_2_total {
            return 1;
        }
    } else if pair_1_total < pair_2_total {
        let mut cleaned = 0;
        for n in pair_1_first_id..pair_1_second_id + 1 {
            if n >= pair_2_first_id && n <= pair_2_second_id {
                cleaned += 1;
            }
        }
        if cleaned == pair_1_total {
            return 1;
        }
    } else {
        let mut cleaned = 0;
        for n in pair_1_first_id..pair_1_second_id + 1 {
            if n >= pair_2_first_id && n <= pair_2_second_id {
                cleaned += 1;
            }
        }
        if cleaned == pair_1_total {
            return 1;
        }
    }
    0
}

pub fn solve_b(data: &[String]) -> i32 {
    data.iter().map(|pair| find_overlap_b(pair)).sum::<i32>()
}

fn find_overlap_b(pair: &String) -> i32 {
    let pair_1 = pair.split(",").collect::<Vec<&str>>()[0];
    let pair_2 = pair.split(",").collect::<Vec<&str>>()[1];

    let pair_1_first_id = pair_1.split("-").collect::<Vec<&str>>()[0]
        .parse::<u32>()
        .unwrap();
    let pair_1_second_id = pair_1.split("-").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let pair_2_first_id = pair_2.split("-").collect::<Vec<&str>>()[0]
        .parse::<u32>()
        .unwrap();
    let pair_2_second_id = pair_2.split("-").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let mut cleaned = 0;
    for n in pair_1_first_id..pair_1_second_id + 1 {
        if n >= pair_2_first_id && n <= pair_2_second_id {
            cleaned += 1;
        }
    }
    if cleaned != 0 {
        return 1;
    }

    0
}
