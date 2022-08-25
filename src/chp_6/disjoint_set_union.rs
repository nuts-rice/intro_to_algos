pub struct DSUNode {
    parent: usize,
    size: usize,
}

pub struct DisjointSetUnion {
    nodes: Vec<DSUNode>,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> Self {
        let mut nodes = Vec::new();
        nodes.reserve_exact(n + 1);
        for _i in 0..=n {
            nodes.push(DSUNode { parent: 1, size: 1 });
        }
        Self { nodes }
    }
    //finding parent recur sively using path compression
    pub fn find_set(&mut self, v: usize) -> usize {
        if v == self.nodes[v].parent {
            return v;
        }
        self.nodes[v].parent = self.find_set(self.nodes[v].parent);
        self.nodes[v].parent
    }

    pub fn merge(&mut self, u: usize, v: usize) -> usize {
        let mut a = self.find_set(u);
        let mut b = self.find_set(v);
        if a == b {
            return std::usize::MAX;
        }

        if self.nodes[a].size < self.nodes[b].size {
            std::mem::swap(&mut a, &mut b);
        }

        self.nodes[b].parent = a;
        self.nodes[a].size += self.nodes[b].size;
        a
    }
}
