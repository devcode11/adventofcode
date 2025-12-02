pub fn solve() {
    let input = std::fs::read_to_string("./inputs/day2.txt").expect("Failed to read input file");

    let result = problem2(parse_input(input));
    println!("Result is: {result}");
}

fn parse_input(input: String) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|prd_id_rng| prd_id_rng.split_once('-').unwrap())
        .map(|tup| (tup.0.parse::<u64>().unwrap(), tup.1.parse::<u64>().unwrap()))
        .collect::<Vec<_>>()
}

fn problem1(ranges: Vec<(u64, u64)>) -> u64 {
    // let limit = ranges.iter().map(|tup| tup.1).max().unwrap().to_owned();
    let mut total = 0;
    for (start, end) in ranges {
        println!("Processing range {start} - {end}");
        let (s_dc, e_dc) = (digit_cnt(start), digit_cnt(end));
        if s_dc % 2 != 0 && s_dc == e_dc {
            continue;
        }
        let mut h = first_half(start);
        if h == 0 {
            h += 1;
        }
        loop {
            let num = h * (10u32.pow(digit_cnt(h)) as u64) + h;
            if num > end {
                break;
            }
            if num >= start {
                // println!("Adding {num}");
                total += num;
            }
            h += 1
        }
    }

    total
}

fn problem2(ranges: Vec<(u64, u64)>) -> u64 {
    // let limit = ranges.iter().map(|tup| tup.1).max().unwrap().to_owned();
    let mut total = 0;
    for (start, end) in ranges {
        println!("Processing range {start} - {end}");
        for i in start..=end {
            let s = i.to_string();
            for j in 1..s.len() {
                let rep = s[0..j].to_string();
                if s.replace(&rep, "").is_empty() {
                    // println!("Adding {i}");
                    total += i;
                    break;
                }
            }
        }
        // println!("\tTotal = {total}");
    }

    total
}

fn first_half(num: u64) -> u64 {
    let digits = digit_cnt(num);
    let pow = if digits.is_multiple_of(2) {
        digits / 2
    } else {
        digits / 2 + 1
    };
    num / (10u64.pow(pow))
}

fn digit_cnt(num: u64) -> u32 {
    num.ilog10() + 1_u32
}
