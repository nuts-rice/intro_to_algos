
pub struct UnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Creates a new UnionFind data structure with n elements
    pub fn new(n: usize) -> Self {
        let mut id = vec![0; n];
        let mut size = vec![0; n];
        for i in 0..n {
            id[i] = i;
            size[i] = 1;
        }
        Self { id, size, count: n }
    }

    /// Returns the parent of the element
    pub fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while x != self.id[x] {
            x = self.id[x];
            // self.id[x] = self.id[self.id[x]]; // path compression
        }
        x
    }

    /// Unions the sets containing x and y
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            self.id[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.id[y] = x;
            self.size[x] += self.size[y];
        }
        self.count -= 1;
        true
    }

