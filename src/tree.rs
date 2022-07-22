use crate::node::*;
use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct BinaryTree<T>(Option<Node<T>>);

impl <T> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        Default::default()
    }
}

impl <T> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree(None)
    }
}

impl <'a, T: 'a + PartialOrd + Eq + Clone> FromIterator<&'a T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item=&'a T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(first_value) = iter.next() {
            let mut root_node = Node::new(first_value.clone());

            iter.for_each(|value| root_node.insert(value.clone()));

            BinaryTree(Some(root_node))
        } else {
            BinaryTree(None)
        }
    }
}

impl <'a, T: 'a + PartialOrd + Eq + Clone> FromIterator<&'a mut T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item=&'a mut T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(first_value) = iter.next() {
            let mut root_node = Node::new(first_value.clone());

            iter.for_each(|value| root_node.insert(value.clone()));

            BinaryTree(Some(root_node))
        } else {
            BinaryTree(None)
        }
    }
}

impl <T: PartialOrd + Eq> FromIterator<T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(first_value) = iter.next() {
            let mut root_node = Node::new(first_value);

            iter.for_each(|value| root_node.insert(value));

            BinaryTree(Some(root_node))
        } else {
            BinaryTree(None)
        }
    }
}

impl <T> BinaryTree<T> {
    pub fn is_empty(&self) -> bool {
        self.root().is_none()
    }

    pub fn is_filled(&self) -> bool {
        self.root().is_some()
    }

    pub fn root(&self) -> Option<&Node<T>> {
        self.0.as_ref()
    }

    pub fn root_mut(&mut self) -> Option<&mut Node<T>> {
        self.0.as_mut()
    }

    pub fn invert(&mut self) {
        if let Some(root_node) = self.root_mut() {
            root_node.invert_recursive();
        }
    }
}

impl <T: PartialOrd + Eq> BinaryTree<T> {
    pub fn contains(&self, search: &T) -> bool {
        if let Some(root_node) = self.root() {
            root_node.contains(search)
        } else {
            false
        }
    }

    pub fn insert(&mut self, data: T) {
        if let Some(root_node) = self.root_mut() {
            root_node.insert(data);
        } else {
            self.0 = Some(Node::new(data));
        }
    }
}
