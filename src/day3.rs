pub fn solve() {
    let input = std::fs::read_to_string("inputs/day3.txt").expect("Cannot read input file");
    let result = problem2(parse_input(input));

    println!("{result}");
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Could not parse digit"))
                .collect()
        })
        .collect()
}

fn problem1(input: Vec<Vec<u32>>) -> u32 {
    input
        .into_iter()
        .map(|arr| {
            let arr_len = arr.len();
            let (mut d1, mut d2) = (arr[arr_len - 2], arr[arr_len - 1]);
            arr.into_iter().rev().skip(2).for_each(|d| {
                if d >= d1 {
                    if d1 > d2 {
                        d2 = d1;
                    }
                    d1 = d;
                }
            });

            // println!(" sum {d1 * 10 + d2}");
            d1 * 10 + d2
        })
        .sum()
}

fn problem2(input: Vec<Vec<u32>>) -> u64 {
    const REQUIRED: usize = 12;
    input
        .into_iter()
        .map(|arr| {
            let mut digits = Vec::new();
            arr.into_iter().rev().for_each(|mut d| {
                if digits.len() < REQUIRED {
                    digits.push(d);
                } else {
                    for c in digits.iter_mut().rev() {
                        if d >= *c {
                            (d, *c) = (*c, d)
                        } else {
                            break;
                        }
                    }
                }
            });

            let mut result: u64 = 0;
            digits
                .into_iter()
                .rev()
                .for_each(|d| result = result * 10 + (d as u64));
            // println!("Result: {result}");
            result
        })
        .sum()
}
