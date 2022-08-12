use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_idx: usize,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
        Node {
            keys: match _keys {
                Some(_keys) => _keys,
                None => Vec::with_capacity(degree - 1),
            },
            children: match _children {
                Some(_children) => _children,
                None => Vec::with_capacity(degree),
            },
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
        BTreeProps {
            degree,
            max_keys: degree - 1,
            mid_key_idx: (degree - 1) / 2,
        }
    }

    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }

    //split asserts child node is full
    //Move middle_key to parent node and split child node's keys/children into half
    //splits a full node y (having 2 t − 1 keys) around its median key y .
    //keyt into two nodes having only t − 1 keys each.
    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[self.mid_key_idx];
        let right_keys = match child.keys.split_off(self.mid_key_idx).split_first() {
            Some((_first, _others)) => _others.to_vec(),
            None => Vec::with_capacity(self.max_keys),
        };

        let right_children = if !child.is_leaf() {
            Some(child.children.split_off(self.mid_key_idx + 1))
        } else {
            None
        };
        //new child node consists of split right of other child
        let new_child_node: Node<T> = Node::new(self.degree, Some(right_keys), right_children);

        parent.keys.insert(child_index, middle_key);
        parent.children.insert(child_index + 1, new_child_node);
    }

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
        //borrow semantics lol
        let mut idx: isize = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while idx >= 0 && node.keys[idx as usize] >= key {
            idx -= 1;
        }

        let mut u_idx: usize = usize::try_from(idx + 1).ok().unwrap();
        if node.is_leaf() {
            node.keys.insert(u_idx, key)
        } else {
            if self.is_maxed_out(&node.children[u_idx]) {
                self.split_child(node, u_idx);
                if node.keys[u_idx] < key {
                    u_idx += 1;
                }
            }

            self.insert_non_full(&mut node.children[u_idx], key);
        }
    }
}
