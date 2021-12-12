use std::collections::HashMap;
use petgraph::{Graph, Undirected};
use petgraph::graph::{NodeIndex, UnGraph};

#[aoc_generator(day12)]
fn generator_input(input: &str) -> Graph<String, (), Undirected> {
    let edges: Vec<(&str, &str)> = input.lines().map(|l| {
        let mut nodes = l.split('-');
        (nodes.next().unwrap(), nodes.next().unwrap())
    }).collect();
    let mut g = UnGraph::new_undirected();
    let mut nodes = HashMap::new();
    for (v1, v2) in edges {
        let n1 = *nodes.entry(v1).or_insert_with(|| g.add_node(v1.to_string()));
        let n2 = *nodes.entry(v2).or_insert_with(|| g.add_node(v2.to_string()));
        g.add_edge(n1, n2, ());
    }
    g
}

fn get_node_idx(g: &Graph<String, (), Undirected>, lbl: &str) -> Option<NodeIndex> {
    g.node_indices().find(|n| get_node_weight(g, n.index()).unwrap() == lbl)
}

#[aoc(day12, part1)]
fn part1(g: &Graph<String, (), Undirected>) -> u32 {
    let mut visited: Vec<usize> = vec![];
    let mut counter: u32 = 0;
    count_paths(g, get_node_idx(g, "start").unwrap(), get_node_idx(g, "end").unwrap(), &mut counter, &mut visited, false);
    counter
}

#[aoc(day12, part2)]
fn part2(g: &Graph<String, (), Undirected>) -> u32 {
    let mut visited: Vec<usize> = vec![];
    let mut counter: u32 = 0;
    count_paths(g, get_node_idx(g, "start").unwrap(), get_node_idx(g, "end").unwrap(), &mut counter, &mut visited, true);
    counter
}

fn count_paths(g: &Graph<String, (), Undirected>, start: NodeIndex, end: NodeIndex, counter: &mut u32, visited: &mut Vec<usize>, special_pass: bool) {
    visited.push(start.index());
    if start == end {
        *counter += 1;
        // for v in visited.clone() {
        //     print!("{}-", get_node_weight(&g, v).unwrap());
        // }
        // println!();
    } else {
        for i in g.neighbors(start) {
            // we never wanna go back to `start`
            if get_node_weight(g, i.index()).unwrap() == "start" {
                continue;
            }
            if !visited.contains(&i.index()) || !small_cave(g, i.index()) {
                count_paths(g, i, end, counter, visited, special_pass);
            } else if special_pass {
                count_paths(g, i, end, counter, visited, false);
            }
        }
    }
    visited.pop();
}

fn small_cave(g: &Graph<String, (), Undirected>, n: usize) -> bool {
    if let Some(l) = get_node_weight(g, n) {
        return l == l.to_lowercase();
    }
    panic!("could not find node");
}

fn get_node_weight(graph: &Graph<String, (), Undirected>, start: usize) -> Option<String> {
    graph.node_weights().enumerate().find_map(|(i, x)| {
        if i == start { Some(x.clone()) } else { None }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SML: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    const INPUT_MED: &str = "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
    const INPUT_LRG: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_parse() {
        let g = generator_input(INPUT_MED);
        println!("{:?}", &g);
        assert_eq!(Some("start".to_string()), get_node_weight(&g, 3));
        //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
        assert!(small_cave(&g, 0));
        assert!(!small_cave(&g, 2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(10, part1(&generator_input(INPUT_SML)));
        assert_eq!(19, part1(&generator_input(INPUT_MED)));
        assert_eq!(226, part1(&generator_input(INPUT_LRG)));
    }

    #[test]
    fn test_counting_paths() {
        assert_eq!(36, part2(&generator_input(INPUT_SML)));
        assert_eq!(103, part2(&generator_input(INPUT_MED)));
        assert_eq!(3509, part2(&generator_input(INPUT_LRG)));
    }
}