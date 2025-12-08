#[path = "utils.rs"]
mod utils;

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(8, false), 1000));
}

#[test]
fn problem2() {
    println!("Result: {}", solution2(utils::read_input(8, false)));
}

#[derive(Debug, PartialEq)]
struct Point(u32, u32, u32);
fn distance_between(p1: &Point, p2: &Point) -> f64 {
    let p1 = (p1.0 as i64, p1.1 as i64, p1.2 as i64);
    let p2 = (p2.0 as i64, p2.1 as i64, p2.2 as i64);
    let s = (p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2) + (p2.2 - p1.2).pow(2);
    (s as f64).sqrt()
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Dist(f64, usize, usize);

struct UnionFind {
    set: Vec<usize>,
    size: Vec<u32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            set: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        if a >= self.set.len() {
            panic!(
                "{} is greater than size of union find {}",
                a,
                self.set.len()
            );
        }
        if self.set[a] == a {
            return a;
        }
        let res = self.find(self.set[a]);
        self.set[a] = res;
        self.size[res] += self.size[a];
        self.size[a] = 0;
        res
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        if a >= self.set.len() {
            panic!(
                "arg1 {} is greater than size of union find {}",
                a,
                self.set.len()
            );
        }
        if b >= self.set.len() {
            panic!(
                "arg2 {} is greater than size of union find {}",
                b,
                self.set.len()
            );
        }
        let (a, b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if a < b {
            self.set[b] = a;
            self.size[a] += self.size[b];
            self.size[b] = 0;
        } else {
            self.set[a] = b;
            self.size[b] += self.size[a];
            self.size[a] = 0;
        }
        true
    }

    pub fn get_sizes(&mut self) -> Vec<u32> {
        let mut sizes = vec![0; self.set.len()];
        for i in 0..self.set.len() {
            let root = self.find(i);
            sizes[root] += 1;
        }
        // println!("self.size {:?}\nsizes {:?}", self.size, sizes);
        sizes
    }

    pub fn all_connected(&self) -> bool {
        // println!("{:?}", self.size);
        self.size[0] == self.set.len() as u32
    }
}

fn parse_points(input: String) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, remaining) = line
                .split_once(',')
                .expect("Could not split line into two points");
            let (y, z) = remaining
                .split_once(',')
                .expect("Could not split remaining into two points");
            Point(
                x.parse::<u32>().expect("Could not parse x into number"),
                y.parse::<u32>().expect("Could not parse y into number"),
                z.parse::<u32>().expect("Could not parse z into number"),
            )
        })
        .collect()
}

fn get_distances(points: &Vec<Point>) -> Vec<Dist> {
    let mut dists = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for j in i + 1..points.len() {
            dists.push(Dist(distance_between(p1, &points[j]), i, j))
        }
    }

    dists.sort_by(|a, b| {
        if a > b {
            return std::cmp::Ordering::Greater;
        } else if b > a {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Equal;
        }
    });
    // println!("Dists: {dists:?}");
    dists
}

fn solution1(input: String, connections: usize) -> u32 {
    let points = parse_points(input);
    let dists = get_distances(&points);

    let mut uf = UnionFind::new(points.len());
    // println!("Before start Union find: {:?}", uf.0);
    for dist in dists.iter().take(connections) {
        uf.union(dist.1, dist.2);
    }
    // println!("Final union find: {:?} with {connections} remaining", uf.0);

    let mut sizes = uf.get_sizes();
    // println!("Union find sizes: {sizes:?}");
    sizes.sort_unstable_by_key(|e| i64::from(*e) * -1);
    sizes.iter().take(3).product()
}

fn solution2(input: String) -> u64 {
    let points = parse_points(input);
    let dists = get_distances(&points);

    let mut uf = UnionFind::new(points.len());
    let mut result = 0;
    // println!("Before start Union find: {:?}", uf.0);
    for dist in dists.iter() {
        if uf.union(dist.1, dist.2) {
            if uf.all_connected() {
                let p1 = &points[dist.1];
                let p2 = &points[dist.2];
                result = p1.0 as u64 * p2.0 as u64;
                break;
            }
        }
    }
    // println!("Final union find: {:?} with {connections} remaining", uf.0);

    result
}
