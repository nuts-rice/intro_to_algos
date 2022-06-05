use std::cmp::Ordering;
use std::ops::Deref;

pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => match key.cmp(value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left {
                    Some(node) => node.search(value),
                    None => false,
                },

                Ordering::Less => match &self.right {
                    Some(node) => node.search(value),
                    None => false,
                },
            },
            None => false,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        } else {
            match &self.value {
                None => (),
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(ref mut node) => {
                            node.insert(value);
                        }
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }

        pub fn minimum(&self) -> Option<&T> {
            match &self.left {
                Some(node) => node.minimum(),
                None => self.value.as_ref(),
            }
        }

        pub fn maximum(&self) -> Option<&T> {
            match &self.right {
                Some(node) => node.maximum(),
                None => self.value.as_ref(),
            }
        }
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    pub fn new() {
        return None;
    }
}
