pub fn solve() {
    let input =
        std::fs::read_to_string("inputs/day6.txt").expect("Could not open input file");

    // problem1(input);
    problem2(input);
}

fn problem2(input: String) {
    let grid = into_grid(input);
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut results: Vec<u64> = Vec::new();
    let mut group_res = 0;
    let add = |a: u64, b: u64| a + b;
    let mul = |a: u64, b: u64| a * b;
    let mut current_op: Box<dyn Fn(u64, u64) -> u64> = Box::new(add);

    for j in 0..cols {
        let mut num = 0;
        for i in 0..rows {
            if i == rows - 1 {
                if grid[i][j] != ' ' {
                    results.push(group_res);
                    println!("results: {results:?}");
                    group_res = num;
                    current_op = if grid[i][j] == '+' {
                        Box::new(add)
                    } else if grid[i][j] == '*' {
                        Box::new(mul)
                    } else {
                        current_op
                    }
                } else if num != 0 {
                    print!("Applying op on {group_res} and {num} = ");
                    group_res = current_op(group_res, num);
                    println!("{group_res}");
                }
            } else {
                if grid[i][j] == ' ' {
                    continue;
                }
                num = num * 10
                    + grid[i][j]
                        .to_digit(10)
                        .expect("Could not convert char to digit") as u64;
            }
        }
    }
    results.push(group_res);

    let result: u64 = results.iter().sum();

    println!("Result: {result}");
}

fn into_grid(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn problem1(input: String) {
    let mut lines = input.lines().rev();
    let operations = parse_operations(lines.next().expect("Could not get operations line"));
    let result = calc1(operations, lines.map(parse_nums));
    println!("Result: {result}");
}

fn parse_nums(line: &str) -> Vec<u32> {
    line.split_ascii_whitespace()
        .map(|num| num.parse::<u32>().expect("Could not parse integer"))
        .collect::<Vec<u32>>()
}

fn parse_operations(line: &str) -> Vec<char> {
    line.split_ascii_whitespace()
        .map(|w| {
            w.chars()
                .next()
                .expect("Could not get operations character")
        })
        .collect::<Vec<_>>()
}

fn calc1(operations: Vec<char>, nums_list: impl Iterator<Item = Vec<u32>>) -> u64 {
    let mut results = operations
        .iter()
        .map(|c| if *c == '*' { 1 } else { 0 })
        .collect::<Vec<u64>>();
    for nums in nums_list {
        for (i, op) in operations.iter().enumerate() {
            results[i] = if *op == '+' {
                results[i] + nums[i] as u64
            } else {
                results[i] * nums[i] as u64
            }
        }
    }
    results.iter().sum()
}
