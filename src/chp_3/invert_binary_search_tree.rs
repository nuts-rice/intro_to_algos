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

impl Soln {
    #[inline]
    //Rc: single threaded refcounter
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree_aux(&mut root);
        root
    }

    pub fn invert_tree_aux(root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root {
            let mut root = root.borrow_mut();

            Self::invert_tree_aux(&mut root.left);
            Self::invert_tree_aux(&mut root.right);

            let left_node = std::mem::replace(&mut root.left, None);
            let right_node = std::mem::replace(&mut root.right, left_node);
            std::mem::replace(&mut root.left, right_node);
        }
    }
}
