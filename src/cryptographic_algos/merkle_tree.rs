use crypto::digest::Digest;
use crypto::sha2::Sha256;
use itertools::Itertools;
use std::boxed::Box;

enum MerkleNodePayload<T: ToString> {
    Leaf(T),
    Node(Box<MerkleNode<T>>, Box<MerkleNode<T>>),
}

use self::MerkleNodePayload::{Leaf, Node};

struct MerkleNode<T: ToString> {
    hash: String,
    payload: MerkleNodePayload<T>,
}

pub struct MerkleTree<T: ToString> {
    root: MerkleNode<T>,
    leaves: Vec<T>,
}

//So given h = Hash(val), then result = Hash(h)
fn hash(val: String) -> String {
    let mut hasher_1 = Sha256::new();
    let mut hasher_2 = Sha256::new();
    hasher_1.input_str(&val);
    let hash1 = hasher_1.result_str();

    hasher_2.input_str(&hash1);

    hasher_2.result_str()
}

//Leaf (lowest) that holds ToString value
fn new_leaf<T>(val: T) -> MerkleNode<T>
where
    T: ToString,
{
    MerkleNode {
        hash: hash(val.to_string()),
        payload: Leaf(val),
    }
}

//intermediate node that links to left and right child
fn build_node<T>(left: MerkleNode<T>, right: MerkleNode<T>) -> MerkleNode<T>
where
    T: ToString,
{
    let concatd = combine(&left.hash, &right.hash);
    MerkleNode {
        hash: hash(concatd),
        payload: Node(Box::new(left), Box::new(right)),
    }
}

//iterates and batches items by matching if they have left and right children
//if has odd number of nodes, last node becomes root
fn build_layer<T>(items: Vec<MerkleNode<T>>) -> Vec<MerkleNode<T>>
where
    T: ToString,
{
    //adaptor that gathers elements by matching conditions to produce next iterated
    let new_layer = items.into_iter().batching(|it| match it.next() {
        Some(left) => match it.next() {
            Some(right) => Some(build_node(left, right)),
            None => Some(left),
        },
        None => None,
    });
    new_layer.collect::<Vec<_>>()
}

fn combine(s1: &String, s2: &String) -> String {
    format!("{}{}", s1, s2)
}

impl<T: ToString> MerkleTree<T> {
    pub fn from_leaves<II>(_items: II) -> MerkleTree<II::Item>
    where
        II: IntoIterator,
        II::Item: ToString + Clone,
    {
        let leaves = _items.into_iter().collect::<Vec<_>>();
        let mut layer: Vec<_> = leaves.iter().cloned().map(new_leaf).collect();
        while layer.len() != 1 {
            layer = build_layer(layer);
        }

        match layer.pop() {
            Some(root) => MerkleTree { root, leaves },
            None => panic!("empty tree"),
        }
    }

    pub fn get_root(&self) -> String {
        self.root.hash.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn merkle_tree_equal_invariant_test() {
        let hashed = vec![
            "FJISJIOFJWIOSJFIW4JERIOFJWIJFJ4I59834985928RFSEKSAJIFAJ324148CVJKSJ",
            "RWIUR83485U834U85U84IJ563U4UT8UGDIUITGUWW958T3495598938593859038958",
            "R23UI249358VGMXVSIF43U953UTUUBD0VUE9UT3959U3905UGJSNNJFGUUI3WUU35T5",
            "DK3QOQ23I4OI92I499289458FJISJIGJTIWEJITU8I3U85U803U90593U95U9UJGEME",
            "DFIIRJ2IJFSTJ3I60-98G9X9GISGOKOWKO5I3985989VBKDF,GOE9TI93I9T93I9TK9",
        ];
        let hashed_clone = hashed.clone();
        let tree_1 = MerkleTree::<&str>::from_leaves(hashed);
        let tree_2 = MerkleTree::<&str>::from_leaves(hashed_clone);
        assert_eq!(tree_1.get_root(), tree_2.get_root());
    }
}
