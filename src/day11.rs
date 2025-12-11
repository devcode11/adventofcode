#[path = "utils.rs"]
mod utils;
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    index_of: HashMap<String, usize>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn index(&self, key: &str) -> usize {
        self.index_of
            .get(key)
            .unwrap_or_else(|| panic!("Could not find index of key {} in graph", key))
            .to_owned()
    }
}

fn parse_graph(input: String) -> Graph {
    let mut index_of = HashMap::new();
    let mut idx = 0;
    for line in input.lines() {
        let (u, _) = line.split_once(':').unwrap();
        index_of.insert(u.to_owned(), idx);
        idx += 1;
    }
    index_of.insert("out".to_owned(), idx);
    let len = index_of.len();
    let mut graph = Graph {
        index_of: index_of,
        edges: vec![vec![]; len],
    };

    for line in input.lines() {
        let (u, vs) = line.split_once(':').unwrap();
        let uidx = graph.index(u);
        for v in vs.trim().split(' ') {
            let vidx = graph.index(v);
            if vidx == uidx {
                panic!("Self loop edge found in graph");
            }
            graph.edges[uidx].push(vidx);
        }
    }

    graph
}

fn count_paths(graph: &Graph, cache: &mut Vec<u32>, from: usize, to: usize) -> u32 {
    graph.edges[from]
        .iter()
        .map(|&v| {
            if v == to {
                1
            } else if cache[v] != 0 {
                cache[v] as u32
            } else {
                count_paths(graph, cache, v, to)
            }
        })
        .sum()
}

fn solution1(input: String) -> u32 {
    let graph = parse_graph(input);
    let mut cache = vec![0; graph.index_of.len()];
    count_paths(&graph, &mut cache, graph.index("you"), graph.index("out"))
}

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(11, true)));
}

fn solution2(input: String) -> u32 {
    println!("{:?}", parse_graph(input));
    0
}

#[test]
fn problem2() {
    println!("Result: {}", solution2(utils::read_input(11, true)));
}
