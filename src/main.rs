// TODO: delete item in tree
// TODO: use from/into traits
// TODO: optimize/order tree

use std::fmt::Debug;
use node::*;

mod node;

#[derive(Clone, Debug)]
pub struct BinaryTree<T>(Option<Node<T>>);

impl <T: PartialOrd + Eq + Clone> BinaryTree<T> {
    pub fn new(data: &[T]) -> BinaryTree<T> {
        // TODO: Check if consuming array items is possible
        if let Some(first_value) = data.first() {
            let mut root_node = Node::new(first_value.clone());

            let mut value_iter = data.iter();
            value_iter.next();
            value_iter.for_each(|value| root_node.insert(value.clone()));

            BinaryTree(Some(root_node))
        } else {
            BinaryTree::empty()
        }
    }

    pub fn empty() -> BinaryTree<T> {
        BinaryTree(None)
    }
}

impl <T> BinaryTree<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_filled(&self) -> bool {
        self.0.is_some()
    }

    pub fn invert(&mut self) {
        if let Some(root_node) = &mut self.0 {
            root_node.invert_recursive();
        }
    }
}

impl <T: PartialOrd + Eq> BinaryTree<T> {
    pub fn contains(&self, search: &T) -> bool {
        if let Some(root_node) = &self.0 {
            root_node.contains(search)
        } else {
            false
        }
    }

    pub fn insert(&mut self, data: T) {
        if let Some(root_node) = &mut self.0 {
            root_node.insert(data);
        } else {
            self.0 = Some(Node::new(data));
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new(&[10, 13, 15, 3, 5, 7, 17]);

    println!("{:#?}", tree.contains(&17));

    tree.insert(11);
    println!("{:#?}", tree.contains(&11));

    println!("{:#?}", tree);
}
