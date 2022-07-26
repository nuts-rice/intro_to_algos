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
                self.get_alphabet(height + 1, path, node.left.as_ref().unwrap(), map);
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

pub struct HuffmanDictionary<T> {
    pub alphabet: BTreeMap<T, HuffmanValue>,
    pub root: HuffmanNode<T>,
}

//The algorithm builds the tree T corresponding to an
//optimal code in a bottom-up manner. It begins with a set of | C | leaves
//and performs a sequence of | C | − 1 “merging” operations to create the
//ﬁnal tree. The algorithm uses a min-priority queue Q , keyed on the freq
//attribute, to identify the two least-frequent objects to merge together.
//The result of merging two objects is a new object whose frequency is the
//sum of the frequencies of the two objects that were merged.
impl<T: Clone + Copy + Ord> HuffmanDictionary<T> {
    pub fn new(alphabet: &[(T, u64)]) -> Self {
        let mut alph: BTreeMap<T, HuffmanValue> = BTreeMap::new();
        let mut queue: BinaryHeap<HuffmanNode<T>> = BinaryHeap::new();
        for (symbol, freq) in alphabet.iter() {
            queue.push(HuffmanNode {
                left: None,
                right: None,
                symbol: Some(*symbol),
                frequency: *freq,
            });
        }
        for _ in 1..alphabet.len() {
            let left = queue.pop().unwrap();
            let right = queue.pop().unwrap();
            let sm_freq = left.frequency + right.frequency;
            queue.push(HuffmanNode {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
                symbol: None,
                frequency: sm_freq,
            });
        }
        let root = queue.pop().unwrap();
        root.get_alphabet(0, 0, &root, &mut alph);
        HuffmanDictionary {
            alphabet: alph,
            root,
        }
    }

    pub fn encode(&self, data: &[T]) -> HuffmanEncoding {
        let mut result = HuffmanEncoding::new();
        data.iter()
            .for_each(|value| result.add_data(*self.alphabet.get(value).unwrap()));
        result
    }
}

pub struct HuffmanEncoding {
    pub num_bits: u64,
    pub data: Vec<u64>,
}

impl Default for HuffmanEncoding {
    fn default() -> Self {
        Self::new()
    }
}

impl HuffmanEncoding {
    pub fn new() -> Self {
        HuffmanEncoding {
            num_bits: 0,
            data: vec![0],
        }
    }

    #[inline]
    pub fn add_data(&mut self, data: HuffmanValue) {
        let shift = (self.num_bits & 63) as u32;
        let val = data.value;
        *self.data.last_mut().unwrap() |= val.wrapping_shl(shift);
        if (shift + data.bits) >= 64 {
            self.data.push(val.wrapping_shr(64 - shift));
        }
        self.num_bits += data.bits as u64;
    }

    fn get_bit(&self, pos: u64) -> bool {
        (self.data[(pos >> 6) as usize] & (1 << (pos & 63))) != 0
    }

    pub fn decode<T: Clone + Copy + Ord>(&self, dict: &HuffmanDictionary<T>) -> Option<Vec<T>> {
        let mut state = &dict.root;
        let mut result: Vec<T> = vec![];
        for i in 0..self.num_bits {
            if state.symbol.is_some() {
                result.push(state.symbol.unwrap());
                state = &dict.root;
            }
            match self.get_bit(i) {
                false => state = state.left.as_ref().unwrap(),
                true => state = state.right.as_ref().unwrap(),
            }
        }

        if self.num_bits > 0 {
            result.push(state.symbol?);
        }
        Some(result)
    }
}
