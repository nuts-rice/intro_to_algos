use std::collections::BTreeMap;
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn floyd_warshall<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
) -> BTreeMap<V, BTreeMap<V, E>> {
    let mut map: BTreeMap<V, BTreeMap<V, E>> = BTreeMap::new();

    for (neighbor, edges) in graph.iter() {
        if !map.contains_key(neighbor) {
            map.insert(*neighbor, BTreeMap::new());
        }
        for (v, weight) in edges.iter() {
            if !map.contains_key(v) {
                map.insert(*v, BTreeMap::new());
            }
            map.entry(*neighbor).and_modify(|mp| {
                mp.insert(*v, *weight);
            });
        }
    }
    let keys = map.iter().map(|(k, _)| *k).collect::<Vec<_>>();
    for &k in &keys {
        for &i in &keys {
            if map[&i].get(&k).is_none() {
                continue;
            }
            for &j in &keys {
                if i == j {
                    continue;
                }
                if !map[&k].contains_key(&j) {
                    continue;
                }
                let entry_i_j = map[&i].get(&j);
                let entry_i_k = map[&i][&k];
                let entry_k_j = map[&k][&j];
                match entry_i_j {
                    Some(&e) => {
                        if e > entry_i_k + entry_k_j {
                            map.entry(i)
                                .or_default()
                                .insert(j, entry_i_k + entry_k_j);
                        }
                    }
                    None => {
                        map.entry(i).or_default().insert(j, entry_i_k + entry_k_j);
                    }
                };
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
    }

    fn bi_add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        add_edge(graph, v1, v2, c);
        add_edge(graph, v2, v1, c);
    }
}
