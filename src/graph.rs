use std::{collections::HashMap, hash::Hash};

pub struct UndirectedGraph<T>
where
    T: Hash,
    T: Clone,
    T: Eq
{
    pub size: usize,
    pub to_num: HashMap<T, usize>,
    pub from_num: Vec<T>,
    pub adjacency: Vec<Vec<usize>>,
}

impl<T> UndirectedGraph<T> 
where
    T: Hash,
    T: Clone,
    T: Eq,
{
    pub fn new<J: Iterator<Item = (T, T)>>(adjacencies: J) -> Self {
        let mut graph = UndirectedGraph {
            size: 0,
            adjacency: Vec::new(),
            to_num: HashMap::new(),
            from_num: Vec::new()
        };
        adjacencies.for_each(|(u, v)| graph.add_edge(u, v));
        graph
    }

    pub fn add_edge(&mut self, u: T, v: T) {
        let u_index = if let Some(index) = self.to_num.get(&u) {
            *index
        } else {
            self.add_node(u)
        };
        let v_index = if let Some(index) = self.to_num.get(&v) {
            *index
        } else {
            self.add_node(v)
        };
        self.adjacency[u_index].push(v_index);
        self.adjacency[v_index].push(u_index);
    }

    pub fn add_node(&mut self, u: T) -> usize {
        self.adjacency.push(Vec::new());
        self.to_num.insert(u.clone(), self.size);
        self.from_num.push(u);
        self.size += 1;
        self.size - 1
    }

    pub fn get_node_number(&self, u: T) -> Option<&usize> {
        self.to_num.get(&u)
    }

    pub fn get_node_value(&self, u: usize) -> &T {
        &self.from_num[u]
    }
}