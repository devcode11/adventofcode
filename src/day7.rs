use std::collections::HashSet;
#[path = "utils.rs"]
mod utils;

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(7, false)));
}

fn solution1(input: String) -> u32 {
    let mut beams = HashSet::new();
    let mut lines = input.lines();
    let start = lines
        .next()
        .unwrap()
        .find('S')
        .expect("Could not find 'S' in first line");
    beams.insert(start);
    let mut count = 0;

    lines.for_each(|line| {
        for (i, c) in line.as_bytes().iter().enumerate() {
            if *c == b'^' && beams.contains(&i) {
                count += 1;
                beams.remove(&i);
                beams.insert(i + 1);
                beams.insert(i - 1);
            }
        }
    });

    return count;
}

#[test]
fn problem2() {
    println!("Result: {}", solution2(utils::read_input(7, false)));
}

fn solution2(input: String) -> u64 {
    let width = input.lines().next().unwrap().len();
    let mut beams = vec![0; width];
    let mut lines = input.lines();
    let start = lines
        .next()
        .unwrap()
        .find('S')
        .expect("Could not find 'S' in first line");
    beams[start] = 1;

    lines.for_each(|line| {
        for (i, c) in line.as_bytes().iter().enumerate() {
            if *c == b'^' && beams[i] != 0 {
                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
            }
        }
    });

    return beams.iter().sum();
}
