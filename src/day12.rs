#[path = "utils.rs"]
mod utils;

struct Present([[u8; 3]; 3]);

impl std::fmt::Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|row| row
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(""))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

struct Region {
    dimensions: (u8, u8),
    requirements: Vec<u8>,
}

fn parse_input(input: String) -> (Vec<Present>, Vec<Region>) {
    let mut presents = Vec::new();
    let mut regions = Vec::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let bytes = line.bytes().collect::<Vec<_>>();
        if bytes[1] == b':' {
            let mut present = [[0; 3]; 3];
            for i in 0..3 {
                for (j, c) in lines
                    .next()
                    .expect("Could not get next line for present")
                    .bytes()
                    .take(3)
                    .enumerate()
                {
                    present[i][j] = if c == b'#' { 1 } else { 0 }
                }
            }
            presents.push(Present(present));
            lines.next();
        } else {
            let (dims, reqs) = line
                .split_once(": ")
                .expect("Could not split requirements into dimensions");
            let dims = dims.split_once('x').expect("Could not split dimensions");
            let dims = (
                dims.0
                    .parse::<u8>()
                    .expect("Could not parse first dimension into integer"),
                dims.1
                    .parse::<u8>()
                    .expect("Could not parse second dimension into integer"),
            );

            let reqs = reqs
                .split(' ')
                .map(|r| {
                    r.parse::<u8>()
                        .expect("Could not parse requirement into integer")
                })
                .collect();

            regions.push(Region {
                dimensions: dims,
                requirements: reqs,
            });
        }
    }

    (presents, regions)
}

fn solution1(input: String) -> usize {
    let (presents, regions) = parse_input(input);

    println!("Presents:");
    for p in presents.iter() {
        println!("{}\n", p);
    }
    for region in regions.iter() {
        println!("{:?}: {:?}", region.dimensions, region.requirements);
    }

    let present_sizes = presents
        .iter()
        .map(|present| {
            present
                .0
                .iter()
                .map(|&row| row.iter().sum::<u8>())
                .sum::<u8>()
        })
        .collect::<Vec<_>>();

    println!("Present sizes: {:?}", present_sizes);

    regions
        .iter()
        .filter(|reg| {
            let available = reg.dimensions.0 as u64 * reg.dimensions.1 as u64;
            let required = reg
                .requirements
                .iter()
                .zip(present_sizes.iter())
                .map(|(&count, &present_size)| count as u64 * present_size as u64)
                .sum::<u64>();
            required <= available
        })
        .count()
}

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(12, false)));
}
