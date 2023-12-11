mod graph;
mod graph_dfs;
mod constants;
use graph::UndirectedGraph;
use graph_dfs::{dfs, threaded_dfs};

fn main() {
    println!("In the following test, you'll provide some undirected graphs with vertices of different types");
    println!("You must enter each edge on a different line as a space-separated pair of values representing the two ends of the edge");
    println!("Make sure to provide an EOF at the end, and specify the starting vertex");

    // Test 1: Graph of numbers (starts from 0, so make sure to add an edge with 0)
    println!("Enter a graph of numbers which contains the vertex \"0\"");
    let graph = UndirectedGraph::new(std::io::stdin().lines().map(|line| {
        let unwrapped_line = line.expect("Vertex was not a number");
        let mut it = unwrapped_line.split(' ');
        (it.next().unwrap().parse::<i32>().unwrap(), it.next().unwrap().parse::<i32>().unwrap())
    }));

    dfs(&graph, 0, print_number);
    
    // Test 2: Graph of words (ensure one of the words is "start")
    println!("Enter a graph of words which contains the vertex \"start\"");
    let graph = UndirectedGraph::new(std::io::stdin().lines().map(|line| {
        let unwrapped_line = line.unwrap();
        let mut it = unwrapped_line.split(' ');
        (String::from(it.next().unwrap()), String::from(it.next().unwrap()))
    }));

    dfs(&graph, "start".to_string(), print_string);

    // Test 3: Threaded DFS through graph
    println!("Enter a graph of words which contains the vertex \"start\"");
    let graph = UndirectedGraph::new(std::io::stdin().lines().map(|line| {
        let unwrapped_line = line.unwrap();
        let mut it = unwrapped_line.split(' ');
        (String::from(it.next().unwrap()), String::from(it.next().unwrap()))
    }));
    
    threaded_dfs(&graph, "start".to_string(), print_string);
}

fn print_number(n: &i32) {
    println!("Processed number {}", *n);
}

fn print_string(line: &String) {
    println!("Processed word {line}");
}