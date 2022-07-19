use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct HuffmanValue {
    pub value: u64,
    pub bits: u32,
}

pub struct HuffmanNode<T> {
    pub left: Option<Box<HuffmanNode<T>>>,
    pub right: Option<Box<HuffmanNode<T>>>,
    pub symbol: Option<T>,
    pub frequency: u64,
}

impl<T> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl<T> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.frequency.cmp(&other.frequency).reverse())
    }
}

impl<T> Eq for HuffmanNode<T> {}

impl<T> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

impl<T: Clone + Copy + Ord> HuffmanNode<T> {
    pub fn get_alphabet(
        &self,
        height: u32,
        path: u64,
        node: &HuffmanNode<T>,
        map: &mut BTreeMap<T, HuffmanValue>,
    ) {
        match node.symbol {
            Some(s) => {
                map.insert(
                    s,
                    HuffmanValue {
                        value: path,
                        bits: height,
                    },
                );
            }
            None => {
                self.get_alphabet(
                    height + 1,
                    path | (1 << height),
                    node.right.as_ref().unwrap(),
                    map,
                );
            }
        }
    }
}
