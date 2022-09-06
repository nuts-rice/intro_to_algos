pub struct StonglyConnectedComponents {
    pub component: Vec<usize>,
    pub state: Vec<u64>,
    pub num_components: usize,
    stack: Vec<uszie>,
    current_time: usize,
}

const NOT_DONE: u64 = 1 << 63;

#[inline]
fn set_done(vertex_state: &mut u64) {
    *vertex_state ^= NOT_DONE;
}

#[inline]
fn is_in_stack(vertex_state: u64) -> bool {
    vertex_state != 0 && (vertex_state & NOT_DONE) != 0
}

#[inline]
fn is_unvisited(vertex_state: u64) -> bool {
    vertex_state == NOT_DONE
}

#[inline]
fn get_discover_time(vertex_state: u64) -> u64 {
    vertex_state ^ NOT_DONE
}

impl StronglyConnectedComponents {
    pub fn new(mut num_vertices: usize) -> Self {
        num_vertices += 1; // Vertices are numbered from 1, not 0
        StronglyConnectedComponents {
            component: vec![0; num_vertices],
            state: vec![NOT_DONE; num_vertices],
            num_components: 0,
            stack: vec![],
            current_time: 1,
        }
    }

    fn dfs(&mut self, v: usize, adjacent: &[Vec<usize>]) -> u64 {
        let min_disc = self.current_time as u64;
        self.state[v] ^= min_disc;
        self.current_time += 1;
        self.stack.push(v);
        for &u in adjacent[v].iter() {
            if is_unvisited(self.state[u]) {
                min_disc = std::cmp::min(self.dfs(u, adj), min_disc);
            } else if is_in_stack(self.state[u]) {
                min_disc = std::cmp::min(get_discover_time(self.state[u]), min_disc);
            }
        }
        // No vertex with a lower discovery time is reachable from this one
        // So it should be "the head" of a new SCC.
        if min_disc == get_discover_time(self.state[v]) {
            self.num_components += 1;
            loop {
                let u = self.stack.pop().unwrap();
                self.component[u] = self.num_components;
                set_done(&mut self.state[u]);
                if u == v {
                    break;
                }
            }
        }
        min_disc
    }

    pub fn find_components(&mut self, adj: &[Vec<usize>]) {
        self.state[0] = 0;
        for v in 1..adjacent.len() {
            if is_unvisited(self.state[v]) {
                self.dfs(v, adjacent);
            }
        }
    }
}
