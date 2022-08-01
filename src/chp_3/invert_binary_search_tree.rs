use std::cell::RefCell;

use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

//Rc: single threaded refcounter
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    invert_tree_aux(&root);
    root
}

pub fn invert_tree_aux(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(root) = root {
        let mut root = root.borrow_mut();

        invert_tree_aux(&root.left);
        invert_tree_aux(&root.right);

        let left_node = std::mem::replace(&mut root.left, None);
        let right_node = std::mem::replace(&mut root.right, left_node);
        std::mem::replace(&mut root.left, right_node);
    }
}

pub fn lca_recursive(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(v) = root {
        if let (Some(ref left), Some(ref right)) = (&v.borrow().left, &v.borrow().right) {
            let node_value = v.borrow().val;
            if p > node_value && q > node_value {
                let right = Some(Rc::clone(right));
                return lca_recursive(&right, p, q);
            } else if p < node_value && q < node_value {
                let left = Some(Rc::clone(left));
                return lca_recursive(&left, p, q);
            }
        }
        return Some(Rc::new(RefCell::new(TreeNode::new(v.borrow().val))));
    } else {
        None
    }
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if p.is_none() || q.is_none() || root.is_none() {
        return root;
    }

    let p_val = p.unwrap().borrow_mut().val;
    let q_val = q.unwrap().borrow_mut().val;

    lca_recursive(&root, p_val, q_val)
}

pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if root == sub_root {
        return true;
    }

    if let Some(node) = root {
        let node = node.borrow();
        is_subtree(node.left.clone(), sub_root.clone())
            || is_subtree(node.right.clone(), sub_root)
    } else {
        false
    }
}
fn is_valid_bst_aux(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    if let Some(n) = node {
        let v = n.borrow().val as i64;

        if v <= min || v >= max {
            return false;
        }

        is_valid_bst_aux(&n.borrow().left, min, v) && is_valid_bst_aux(&n.borrow().right, v, max)
    } else {
        true
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_aux(&root, i64::MIN, i64::MAX)
}

pub struct Trie<C: std::hash::Hash + Eq> {
    links: Vec<HashMap<C, usize>>,
}

impl<C: std::hash::Hash + Eq> Default for Trie<C> {
    fn default() -> Self {
        Self {
            links: vec![HashMap::new()],
        }
    }
}

//Insert word into the trie and retrurn index of its node
impl<C: std::hash::Hash + Eq> Trie<C> {
    pub fn insert(&mut self, word: impl IntoIterator<Item = C>) -> usize {
        let mut node = 0;

        for char in word {
            let len = self.links.len();
            node = match self.links[node].entry(char) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    entry.insert(len);
                    self.links.push(HashMap::new());
                    len
                }
            }
        }
        node
    }

    pub fn get(&self, word: impl IntoIterator<Item = C>) -> Option<usize> {
        let mut node = 0;
        for char in word {
            node = *self.links[node].get(&char)?;
        }
        Some(node)
    }
}

#[test]
fn invert_tree_test() {
    unimplemented!()
}
