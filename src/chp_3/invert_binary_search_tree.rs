use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
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
        return None;
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

    return lca_recursive(&root, p_val, q_val);
}


pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>,
                  sub_root: Option<Rc<RefCell<TreeNode>>>)
    -> bool {
        if root == sub_root {
            return true
        }

        if let Some(node) = root {
            let node = node.borrow();
            is_subtree(node.left.clone(), sub_root.clone()) ||
            is_subtree(node.right.clone(), sub_root.clone())
        } else {
            return false
        }
    }
