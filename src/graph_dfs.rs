use std::{
    hash::Hash, 
    sync::{Arc, RwLock}
};
use bitvec::{bitvec, vec::BitVec};
use threadpool::ThreadPool;
use threadpool_scope::scope_with;


use crate::graph::UndirectedGraph;
use crate::constants::MAX_THREADS;

pub fn dfs<T, F: Fn(&T)>(graph: &UndirectedGraph<T>, start: T, process: F)
where
    T: Hash,
    T: Clone,
    T: Eq,
{
    let mut seen = bitvec![0; graph.size];
    let node = *graph.get_node_number(start).expect("Start node does not exist in graph");
    dfs_helper(graph, node, &mut seen, &process);
}

pub fn dfs_helper<T, F: Fn(&T)>(graph: &UndirectedGraph<T>, node: usize, seen: &mut BitVec, process: &F)
where
    T: Hash,
    T: Clone,
    T: Eq,
{
    seen.set(node, true);
    process(graph.get_node_value(node));
    for child in &graph.adjacency[node] {
        if !seen.get(*child).unwrap() {
            dfs_helper(graph, *child, seen, process);
        }
    }
}

pub fn threaded_dfs<T, F : Fn(&T) -> () + Send + Sync>(graph: &UndirectedGraph<T>, start: T, process: F)
where
    T: Hash,
    T: Clone,
    T: Eq,
    T: Sync,
{
    let seen = bitvec![0; graph.size];
    let node = *graph.get_node_number(start).expect("Start node does not exist in graph");
    let pool = ThreadPool::new(MAX_THREADS);
    threaded_dfs_helper(graph, node, Arc::new(RwLock::new(seen)), &process, &pool);
}

pub fn threaded_dfs_helper<'a, T, F: Fn(&T) -> () + Send + Sync>(graph: &UndirectedGraph<T>, node: usize, seen: Arc<RwLock<BitVec>>, process: & F, pool: & ThreadPool)
where
    T: Hash,
    T: Clone,
    T: Eq,
    T: Sync,
{
    {
        let mut seen_writer = seen.write().unwrap();
        seen_writer.set(node, true);
    }
    process(graph.get_node_value(node));
    scope_with(&pool, |scope| {
        for child in &graph.adjacency[node] {
            if !seen.read().unwrap().get(*child).unwrap() {
                let seen = seen.clone();
                scope.execute(move || {
                    threaded_dfs_helper(graph, *child, seen, process, pool);
                })
            }
        }
    })
}