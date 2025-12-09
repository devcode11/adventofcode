#[path = "utils.rs"]
mod utils;

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(9, false)));
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

fn parse_input(input: String) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (y, x) = line.split_once(',').expect("Could not split line");
            (
                x.parse::<usize>().expect("Could not parse x as integer"),
                y.parse::<usize>().expect("Could not parse y as integer"),
            )
        })
        .collect()
}
