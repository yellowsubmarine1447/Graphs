mod graph;
mod graph_dfs;
use graph::UndirectedGraph;
use graph_dfs::{dfs, threaded_dfs};

fn main() {
    // Test 1: Graph of numbers (starts from 0, so make sure to add an edge with 0)
    let graph = UndirectedGraph::new(std::io::stdin().lines().map(|line| {
        let unwrapped_line = line.unwrap();
        let mut it = unwrapped_line.split(' ');
        (it.next().unwrap().parse::<i32>().unwrap(), it.next().unwrap().parse::<i32>().unwrap())
    }));

    dfs(&graph, 0, print_number);
    
    // Test 2: Graph of words (ensure one of the words is "start")
    let graph = UndirectedGraph::new(std::io::stdin().lines().map(|line| {
        let unwrapped_line = line.unwrap();
        let mut it = unwrapped_line.split(' ');
        (String::from(it.next().unwrap()), String::from(it.next().unwrap()))
    }));

    dfs(&graph, "start".to_string(), print_string);

    // Test 3: Threaded DFS through graph
    let graph = UndirectedGraph::new(std::io::stdin().lines().map(|line| {
        let unwrapped_line = line.unwrap();
        let mut it = unwrapped_line.split(' ');
        (String::from(it.next().unwrap()), String::from(it.next().unwrap()))
    }));
    
    threaded_dfs(&graph, "start".to_string(), print_string);
}

fn print_number(n: &i32) {
    println!("{}", *n);
}

fn print_string(line: &String) {
    println!("{line}");
}