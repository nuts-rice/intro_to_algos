use std::collections::{BTreeMap, HashSet, VecDeque};

pub fn depth_first_search(graph: &Graph, root: Node, target: Node) -> Option<Vec<u32>> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(current_node) = queue.pop_front() {
        history.push(current_node.value());

        if current_node == target {
            return Some(history);
        }

        for neighbor in current_node.neighbors(graph).into_iter().rev() {
            if visited.insert(neighbor) {
                queue.push_front(neighbor);
            }
        }
    }

    None
}

pub fn topological_sort(graph: &Graph<Node, Edge>) -> Vec<Node> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut degree: Vec<u32> = Vec::new();
    for u in graph.keys() {
        degree.insert(*u, 0);
        for (v, _) in graph.get(u).unwrap() {
            let entry = degree.entry(*v).or_insert(0);
            *entry += 1;
        }
    }

    let mut queue = VecDeque = VecDeque::new();
    for (u, d) in degree.iter() {
        if *d == 0 {
            queue.push_back(*u);
            visited.insert(*u, true);
        }
    }

    let mut ret = Vec::new();
    while let Some(u) = queue.pop_front() {
        ret.push(u);
        if let Some(from_u) = graph.get(&u) {
            for (v, _) in from_u {
                *degree.get_mut(v).unwrap() -= 1;
                if *degree.get(v).unwrap() == 0 {
                    queue.push_back(*v);
                    visited.insert(*v, true);
                }
            }
        }
    }
    ret
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord)]
pub struct Node(u32);
#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord)]
pub struct Edge(u32, u32);
#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }
}

impl From<u32> for Node {
    fn from(item: u32) -> Self {
        Node(item)
    }
}

impl Node {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Node> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}
