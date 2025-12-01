use std::io::Read;

const LIMIT: i32 = 100;

pub fn solve() {
    let mut input = String::new();
    _ = std::fs::File::open("./inputs/day1.txt")
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Could not read input file");

    let password = problem2(input);

    println!("Password is {password}")
}

fn problem1(input: String) -> i32 {
    let mut curr_pos = 50;
    let mut password = 0;
    let mut rotate_cnt: i32;

    for line in input.lines() {
        rotate_cnt = line
            .split_at(1)
            .1
            .parse::<i32>()
            .expect("Could not parse integer");
        if line.starts_with("L") {
            rotate_cnt = -rotate_cnt
        }
        curr_pos = (curr_pos + rotate_cnt + LIMIT) % LIMIT;
        if curr_pos == 0 {
            password += 1
        }
    }
    password
}

fn problem2(input: String) -> i32 {
    let mut curr_pos = 50;
    let mut password = 0;
    let mut rotate_cnt: i32;

    for line in input.lines() {
        rotate_cnt = line
            .split_at(1)
            .1
            .parse::<i32>()
            .expect("Could not parse integer");
        password += rotate_cnt / LIMIT;
        rotate_cnt = rotate_cnt % LIMIT;
        if line.starts_with("L") {
            rotate_cnt = -rotate_cnt
        }
        curr_pos += rotate_cnt;
        if curr_pos > LIMIT || (curr_pos - rotate_cnt != 0 && curr_pos < 0) {
            password += 1
        }
        curr_pos = (curr_pos + LIMIT) % LIMIT;
        if curr_pos == 0 {
            password += 1
        }
        // println!("Line: {line}, password: {password}, curr_pos: {curr_pos}")
    }
    password
}
