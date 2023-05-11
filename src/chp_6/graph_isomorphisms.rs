use super::*;
use std::{
    collections::{BTreeMap, BinaryHeap},
    ops::{Add, Neg}, cmp::Ordering,
};
type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;
pub fn is_isomorphic<
    V: Ord + Copy,
    E: Ord + Copy + Add<Output = E> + Neg<Output = E> + std::ops::Sub<Output = E>,
>(
    graph: &Graph<V, E>,
    other_graph: &Graph<V, E>,
) -> bool {
    if graph.len() > other_graph.len() || graph.values().count() > other_graph.values().count() {
        return false;
    }
    for element in (graph.values() {
        match other_graph.cmp(element) {
            Ordering::Equal => {
                //vertex matches here, 
                true;
            }
            Ordering::Greater => {
                todo!()
            }
            Ordering::Less => {
                todo!()
            }
            


        }
    }

    fn traverse_neighbors<V: Ord + Copy, E: Ord + Copy> (node: &V) -> Vec<&V> {
     todo!()
    } 

    // for _ in 1..(other_graph.len()) {
    //     for edge in other_graph {
    //         let has_edge = match graph.get(&edge) {
    //             Some(&E) => continue,
    //             None(&E) => false;
    //         };

    todo!()
}
