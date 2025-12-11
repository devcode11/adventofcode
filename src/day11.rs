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

fn count_paths_2(
    graph: &Graph,
    from: usize,
    to: usize,
    cache: &mut Vec<u64>,
    state: &mut Vec<u8>,
) -> u64 {
    if from == to {
        return 1;
    }
    if state[from] == 1 {
        panic!("Loop found: revisiting {}", from);
    }
    state[from] = 1;
    // println!("Starting visit {}, cache: {}", from, cache[from]);
    cache[from] = graph.edges[from]
        .iter()
        .map(|&v| {
            // println!("Cache of {} is {}, state: {}", v, cache[v], state[v]);
            if state[v] > 0 {
                cache[v]
            } else {
                count_paths_2(graph, v, to, cache, state)
            }
        })
        .sum();
    state[from] = 2;
    // println!("Finishing visit {}, cache: {}", from, cache[from]);
    cache[from]
}

fn count_paths(graph: &Graph, from: usize, to: usize) -> u64 {
    let mut cache = vec![0; graph.index_of.len()];
    let mut state = vec![0; graph.index_of.len()];
    cache[to] = 1;
    count_paths_2(graph, from, to, &mut cache, &mut state)
}

fn solution1(input: String) -> u64 {
    let graph = parse_graph(input);
    count_paths(&graph, graph.index("you"), graph.index("out"))
}

#[test]
fn problem1() {
    println!("Result: {}", solution1(utils::read_input(11, false)));
}

fn solution2(input: String) -> u64 {
    let mut graph = parse_graph(input);
    // println!("{:?}", graph);

    let dac_index = graph.index("dac");
    let fft_index = graph.index("fft");
    let svr_index = graph.index("svr");
    let out_index = graph.index("out");

    let dac_edges = graph.edges[dac_index].clone();
    let fft_edges = graph.edges[fft_index].clone();
    let svr_edges = graph.edges[svr_index].clone();

    graph.edges[dac_index] = vec![];
    println!("Calculating paths from svr to fft");
    let svr_fft = count_paths(&graph, svr_index, fft_index);
    println!("\t{}", svr_fft);
    println!("Calculating paths from fft to out");
    let fft_out = count_paths(&graph, fft_index, out_index);
    println!("\t{}", fft_out);
    graph.edges[dac_index] = dac_edges;

    graph.edges[fft_index] = vec![];
    println!("Calculating paths from svr to dac");
    let svr_dac = count_paths(&graph, svr_index, dac_index);
    println!("\t{}", svr_dac);
    println!("Calculating paths from dac to out");
    let dac_out = count_paths(&graph, dac_index, out_index);
    println!("\t{}", dac_out);
    graph.edges[fft_index] = fft_edges;

    graph.edges[svr_index] = vec![];
    println!("Calculating paths from fft to dac");
    let fft_dac = count_paths(&graph, fft_index, dac_index);
    println!("\t{}", fft_dac);
    println!("Calculating paths from dac to fft");
    let dac_fft = count_paths(&graph, dac_index, fft_index);
    println!("\t{}", dac_fft);
    graph.edges[svr_index] = svr_edges;

    svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out
}

#[test]
fn problem2() {
    println!("Result: {}", solution2(utils::read_input(11, false)));
}
