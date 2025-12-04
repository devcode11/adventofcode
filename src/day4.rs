pub fn solve() {
    let input = std::fs::read_to_string("inputs/day4.txt").expect("Could not read input file");

    let result = problem2(parse_input(input));

    println!("Result: {result}");
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn problem1(grid: Vec<Vec<char>>) -> u32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut total = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '@' {
                continue;
            }

            let mut count = 0;
            for ii in i.saturating_sub(1)..=i.saturating_add(1) {
                for jj in j.saturating_sub(1)..=j.saturating_add(1) {
                    if ii < m && jj < n && grid[ii][jj] == '@' {
                        count += 1
                    }
                }
            }
            if count < 5 {
                total += 1;
            }
        }
    }
    total
}

fn problem2(mut grid: Vec<Vec<char>>) -> u32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut total = 0;
    let mut remove = Vec::new();
    loop {
        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c != '@' {
                    continue;
                }

                let mut count = 0;
                for ii in i.saturating_sub(1)..=i.saturating_add(1) {
                    for jj in j.saturating_sub(1)..=j.saturating_add(1) {
                        if ii < m && jj < n && grid[ii][jj] == '@' {
                            count += 1
                        }
                    }
                }
                if count < 5 {
                    total += 1;
                    remove.push((i, j));
                }
            }
        }

        if remove.len() > 0 {
            for (i, j) in remove.iter() {
                grid[*i][*j] = ' ';
            }
            remove.clear();
            continue;
        } else {
            break;
        }
    }
    total
}
