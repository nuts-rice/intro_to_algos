use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    source: &V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut res = BTreeMap::new();
    let mut priority_queue = BinaryHeap::new();

    res.insert(*source, None);

    for (neighbor, weight) in &graph[source] {
        res.insert(*neighbor, Some((*source, *weight)));
        priority_queue.push(Reverse((*weight, neighbor, source)));
    }

    while let Some(Reverse((dist_neighbor, neighbor, previous))) = priority_queue.pop() {
        match res[neighbor] {
            Some((p, d)) if p == *previous && d == dist_neighbor => {}
            _ => continue,
        }

        for (next, weight) in &graph[neighbor] {
            //If result[neighbor] is lower dist than alternative, move on
            match res.get(next) {
                Some(Some((_, dist_next))) if dist_neighbor + *weight >= *dist_next => {}
                Some(None) => {}
                //otherwise path is shorter and added to res
                _ => {
                    res.insert(*next, Some((*neighbor, *weight + dist_neighbor)));
                    priority_queue.push(Reverse((*weight + dist_neighbor, next, neighbor)));
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{dijkstra, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
        graph.entry(v2).or_insert_with(BTreeMap::new);
    }

    #[test]
    fn graph_test_1() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 'a', 'c', 12);
        add_edge(&mut graph, 'a', 'd', 60);
        add_edge(&mut graph, 'b', 'a', 10);
        add_edge(&mut graph, 'c', 'b', 20);
        add_edge(&mut graph, 'c', 'd', 32);
        add_edge(&mut graph, 'e', 'a', 7);

        let mut dists_a = BTreeMap::new();
        dists_a.insert('a', None);
        dists_a.insert('c', Some(('a', 12)));
        dists_a.insert('d', Some(('c', 44)));
        dists_a.insert('b', Some(('c', 32)));
        assert_eq!(dijkstra(&graph, &'a'), dists_a);

        let mut dists_b = BTreeMap::new();
        dists_b.insert('b', None);
        dists_b.insert('a', Some(('b', 10)));
        dists_b.insert('c', Some(('a', 22)));
        dists_b.insert('d', Some(('c', 54)));
        assert_eq!(dijkstra(&graph, &'b'), dists_b);
    }
}
