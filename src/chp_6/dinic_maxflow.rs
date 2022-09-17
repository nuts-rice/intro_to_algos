use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

type Graph = Vec<Vec<usize>>;

pub struct FlowEdge<T> {
    pub sink: usize,
    pub capacity: T,
    pub flow: T,
}

pub struct FlowResultEdge<T> {
    pub source: usize,
    pub sink: usize,
    pub flow: T,
}

impl<T: Clone + Copy + Add + AddAssign + Sub<Output = T> + SubAssign + Ord + Neg + Default>
    FlowEdge<T>
{
    pub fn new(sink: usize, capacity: T) -> Self {
        FlowEdge {
            sink,
            capacity,
            flow: T::Default(),
        }
    }
}

pub struct DinicMaxFlow<T> {
    level: Vec<usize>,
    //last visited edge connected to each vertex
    pub last_edge: Vec<usize>,
    network_solved: bool,
    pub source: usize,
    pub sink: usize,

    //number of edges added to residual network
    pub num_edges: usize,
    pub num_vertices: usize,
    pub adj: Graph,
    pub edges: Vec<FlowEdge<T>>,
}

impl<T: Clone + Copy + Add + AddAssign + Sub<Output = T> + SubAssign + Neg + Ord + Default>
    DinicMaxFlow<T>
{
    fn bfs(&mut self) -> bool {
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(self.source);

        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            for &e in self.adj[v].iter() {
                if self.edges[e].capacity <= self.edges[e].flow {
                    continue;
                }
                let u = self.edges[e].sink;
                if self.level[u] != 0 {
                    continue;
                }
                self.level[u] = self.level[v] + 1;
                q.push_back(u);
            }
        }

        self.level[self.sink] != 0
    }
}
