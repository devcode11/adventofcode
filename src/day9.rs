#[path = "utils.rs"]
mod utils;

fn parse_input(input: String) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let (y, x) = line.split_once(',').expect("Could not split line");
            (
                x.parse::<u32>().expect("Could not parse x as integer"),
                y.parse::<u32>().expect("Could not parse y as integer"),
            )
        })
        .collect()
}

fn solution1(input: String) -> u64 {
    let points = parse_input(input);
    let mut max_area = 0;

    for (i, (x1, y1)) in points.iter().enumerate() {
        for (x2, y2) in points.iter().take(i) {
            let area = (x1.abs_diff(*x2) as u64 + 1) * (y1.abs_diff(*y2) as u64 + 1);
            max_area = std::cmp::max(max_area, area);
        }
    }
    return max_area;
}

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(9, true)));
}

fn is_valid_rect(_p1: &(u32, u32), _p2: &(u32, u32), _points: &Vec<(u32, u32)>) -> bool {
    true
}

fn solution2(input: String) -> u64 {
    let points = parse_input(input);
    let mut max_area = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().take(i) {
            let area = (p1.0.abs_diff(p2.0) as u64 + 1) * (p1.1.abs_diff(p2.1) as u64 + 1);
            if area > max_area && is_valid_rect(p1, p2, &points) {
                println!(
                    "Area {} for points {:?} {:?} greater then max",
                    area, p1, p2
                );
                max_area = area;
            }
        }
    }
    return max_area;
}

#[test]
fn problem2() {
    println!("Result: {}", solution2(utils::read_input(9, true)));
}
