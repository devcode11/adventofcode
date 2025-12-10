#[path = "utils.rs"]
mod utils;
use itertools::Itertools;

#[derive(Debug)]
struct Machine {
    state: u16,
    buttons: Vec<Vec<u16>>,
    joltages: Vec<u16>,
}

fn parse_machines(input: String) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut state = 0;
            let state_end_idx = line
                .chars()
                .position(|c| c == ']')
                .expect("Could not find ] in line");
            for c in line[1..state_end_idx].chars().rev() {
                state <<= 1;
                if c == '#' {
                    state |= 1;
                }
            }
            let jolts_start_idx = line
                .chars()
                .position(|c| c == '{')
                .expect("Could not find { in line");
            let buttons = line[state_end_idx + 2..jolts_start_idx - 1]
                .split(' ')
                .map(|btn| btn[1..btn.len() - 1].to_owned())
                .map(|btn| {
                    btn.split(',')
                        .map(|s| s.parse::<u16>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();

            let joltages = line[jolts_start_idx + 1..line.len() - 1]
                .split(',')
                .map(|num| num.parse::<u16>().unwrap())
                .collect::<Vec<_>>();

            Machine {
                state: state,
                buttons: buttons,
                joltages: joltages,
            }
        })
        .collect()
}

fn solution1(input: String) -> u64 {
    let machines = parse_machines(input);
    let mut total = 0;

    for machine in machines {
        // println!("{:?}", machine);
        let buttons = machine
            .buttons
            .iter()
            .map(|btn| btn.iter().map(|b| 1 << b).sum())
            .collect::<Vec<u16>>();
        for req_press in 1..machine.buttons.len() {
            if buttons
                .iter()
                .combinations(req_press)
                .any(|btns| btns.iter().fold(0, |a, b| a ^ **b) == machine.state)
            {
                total += req_press as u64;
                break;
            }
        }
    }

    total
}

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(10, false)));
}

fn solution2(input: String) -> u64 {
    let machines = parse_machines(input);
    let total = 0;

    for machine in machines {
        println!("{:?}", machine);
    }

    total
}

#[test]
fn problem2() {
    println!("Result: {}", solution2(utils::read_input(10, true)));
}
