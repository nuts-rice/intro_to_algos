use std::collections::HashSet;
use std::collections::VecDeque;

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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(u32);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
