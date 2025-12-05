use std::io::BufRead;

pub fn solve() {
    let file = std::fs::File::open("inputs/day5.txt").expect("Could not open input file");
    let lines = std::io::BufReader::new(file).lines();

    let mut available_start = false;
    let mut fresh_range = Vec::new();
    let mut available = Vec::new();

    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            available_start = true;
            continue;
        }
        if available_start {
            available.push(line.parse::<u64>().expect("Could not parse available"));
        } else {
            let tuple = line.split_once('-').expect("Could not split range line");
            fresh_range.push((
                tuple
                    .0
                    .parse::<u64>()
                    .expect("Could not parse first from fresh range"),
                tuple
                    .1
                    .parse::<u64>()
                    .expect("Could not parse second from fresh range"),
            ));
        }
    }

    // let result = problem1(fresh_range, available);
    let result = problem2(fresh_range);
    println!("Result: {result}");
}

fn problem1(fresh_range: Vec<(u64, u64)>, available: Vec<u64>) -> u64 {
    let mut fresh = 0;

    for a in available {
        for (start, end) in fresh_range.iter() {
            if a >= *start && a <= *end {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

fn problem2(mut fresh_range: Vec<(u64, u64)>) -> u64 {
    let mut fresh = 0;
    let mut curr = 0;
    fresh_range.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    // println!("Fresh range: {fresh_range:?}");
    fresh_range.iter().for_each(|pair| {
        let start = std::cmp::max(pair.0, curr);
        curr = std::cmp::max(curr, pair.1 + 1);
        fresh += std::cmp::max(0, curr - start);
        // println!("Pair: {pair:?}, fresh: {fresh}");
    });
    fresh
}
