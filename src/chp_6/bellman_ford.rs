use std::collections::BTreeMap;
use std::ops::{Add, Neg};

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn bellman_ford<
    V: Ord + Copy,
    E: Ord + Copy + Add<Output = E> + Neg<Output = E> + std::ops::Sub<Output = E>,
>(
    graph: &Graph<V, E>,
    source: &V,
) -> Option<BTreeMap<V, Option<(V, E)>>> {
    let mut res: BTreeMap<V, Option<(V, E)>> = BTreeMap::new();
    res.insert(*source, None);

    for _ in 1..(graph.len()) {
        for (u, edges) in graph {
            let dist_u = match res.get(u) {
                Some(Some((_, d))) => Some(*d),
                Some(None) => None,
                None => continue,
            };

            for (v, d) in edges {
                match res.get(v) {
                    Some(Some((_, dist)))
                        if match dist_u {
                            Some(dist_u) => dist_u + *d >= *dist,
                            None => d >= dist,
                        } => {}
                    Some(None) => {
                        match dist_u {
                            Some(dist_u) if dist_u >= -*d => {}
                            _ => {
                                if *d > *d + *d {
                                    return None;
                                }
                            }
                        };
                    }
                    //shorter path: either dist_v was infinite or it was longer than dist_u + d
                    _ => {
                        res.insert(
                            *v,
                            Some((
                                *u,
                                match dist_u {
                                    Some(dist) => dist + *d,
                                    None => *d,
                                },
                            )),
                        );
                    }
                }
            }
        }
    }
    for (u, edges) in graph {
        for (v, d) in edges {
            match (res.get(u), res.get(v)) {
                (Some(None), Some(None)) if *d > *d + *d => return None,
                (Some(None), Some(Some((_, dv)))) if d < dv => return None,
                (Some(Some((_, du))), Some(None)) if *du < -*d => return None,
                (Some(Some((_, du))), Some(Some((_, dv)))) if *du + *d < *dv => return None,
                (_, _) => {}
            }
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::{bellman_ford, Graph};
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
        assert_eq!(bellman_ford(&graph, &'a'), Some(dists_a));

        let mut dists_c = BTreeMap::new();
        dists_c.insert('c', None);
        dists_c.insert('b', Some(('c', 20)));
        dists_c.insert('d', Some(('c', 32)));
        dists_c.insert('a', Some(('b', 30)));
        assert_eq!(bellman_ford(&graph, &'c'), Some(dists_c));
    }
}
