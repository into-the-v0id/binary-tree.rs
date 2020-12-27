// TODO: allow empty tree
// TODO: delete item in tree
// TODO: optimize/order tree

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct BinaryTree<T>(Node<T>);

impl <T: PartialOrd + Eq + Clone> BinaryTree<T> {
    pub fn try_new(data: &[T]) -> Result<BinaryTree<T>, String> {
        // TODO: Check if consuming array items is possible
        if let Some(first_value) = data.first() {
            let mut root_node = Node::new(first_value.clone());

            let mut value_iter = data.iter();
            value_iter.next();
            value_iter.for_each(|value| root_node.insert(value.clone()));

            Ok(BinaryTree(root_node))
        } else {
            Err("At least one value required; Given: 0".into())
        }
    }
}

impl <T> Deref for BinaryTree<T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> DerefMut for BinaryTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    data: T,
    left: Box<Option<Node<T>>>,
    right: Box<Option<Node<T>>>,
}

#[allow(dead_code)]
impl <T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Box::new(Some(node));
    }

    fn unset_left(&mut self) {
        self.left = Box::new(None);
    }

    pub fn left(&self) -> Option<&Node<T>> {
        self.left.as_ref().as_ref()
    }

    fn left_mut(&mut self) -> Option<&mut Node<T>> {
        self.left.as_mut().as_mut()
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Box::new(Some(node));
    }

    fn unset_right(&mut self) {
        self.right = Box::new(None);
    }

    pub fn right(&self) -> Option<&Node<T>> {
        self.right.as_ref().as_ref()
    }

    fn right_mut(&mut self) -> Option<&mut Node<T>> {
        self.right.as_mut().as_mut()
    }

    pub fn invert(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    pub fn invert_recursive(&mut self) {
        Self::invert_node_recursive(self);
    }

    fn invert_node_recursive<D>(node: &mut Node<D>) {
        node.invert();

        if let Some(left_node) = node.left_mut() {
            Self::invert_node_recursive(left_node);
        }

        if let Some(right_node) = node.right_mut() {
            Self::invert_node_recursive(right_node);
        }
    }

    pub fn has_children(&self) -> bool {
        self.left().is_some() || self.right().is_some()
    }
}

impl <T: PartialOrd + Eq> Node<T>
{
    pub fn contains(&self, search: &T) -> bool {
        Self::node_contains(self, search)
    }

    fn node_contains<D: PartialOrd + Eq>(node: &Node<D>, search: &D) -> bool {
        if &node.data == search {
            return true;
        }

        if search < &node.data {
            if let Some(left_node) = node.left() {
                if Self::node_contains(left_node, search) {
                    return true;
                }
            }
        }

        if search > &node.data {
            if let Some(right_node) = node.right() {
                if Self::node_contains(right_node, search) {
                    return true;
                }
            }
        }

        false
    }

    pub fn insert(&mut self, data: T) {
        Self::insert_data(self, data)
    }

    fn insert_data<D: PartialOrd + Eq>(node: &mut Node<D>, data: D) {
        if &node.data == &data {
            return;
        }

        if &data < &node.data {
            if let Some(left_node) = node.left_mut() {
                Self::insert_data(left_node, data);
                return;
            } else {
                node.set_left(Node::new(data));
                return;
            }
        }

        if &data > &mut node.data {
            if let Some(right_node) = node.right_mut() {
                Self::insert_data(right_node, data);
                return;
            } else {
                node.set_right(Node::new(data));
                return;
            }
        }
    }
}

/* Delete

impl <T: PartialOrd + Eq + Clone> Node<T> {
    pub fn delete(&mut self, data: &T) {
        Self::delete_data(self, data)
    }

    fn delete_data<D: PartialOrd + Eq + Clone>(node: &mut Node<D>, data: &D) {
        if &node.data == data {
            Self::pull_data_from_children(node);
            return;
        }

        if data < &node.data {
            if let Some(left_node) = node.left_mut() {
                if &left_node.data == data && ! left_node.has_children() {
                    node.unset_left();
                    return;
                }

                Self::delete_data(left_node, data);
                return;
            }
        }

        if data > &node.data {
            if let Some(right_node) = node.right_mut() {
                if &right_node.data == data && ! right_node.has_children() {
                    node.unset_right();
                    return;
                }

                Self::delete_data(right_node, data);
                return;
            }
        }
    }

    fn pull_data_from_children<D: Clone>(node: &mut Node<D>) {
        let child_node = node.right().or_else(|| node.left());

        if let Some(child_node) = child_node {
            // Copy data from child
            node.data = child_node.data.clone();
        } else {
            // Delete end-node
            if node.right().is_some() {
                node.unset_right();
            } else if node.left().is_some() {
                node.unset_left();
            }
        }

        if node.right().is_some() {
            let child_node = node.right_mut().unwrap();
            Self::pull_data_from_children(child_node);
            return;
        }

        if node.left().is_some() {
            let child_node = node.left_mut().unwrap();
            Self::pull_data_from_children(child_node);
            return;
        }
    }
}
*/

fn main() {
    let mut tree = BinaryTree::try_new(&[10, 13, 15, 3, 5, 7, 17])
        .expect("Unable to create binary tree");

    println!("{:#?}", tree.contains(&17));

    tree.insert(11);
    println!("{:#?}", tree.contains(&11));

    println!("{:#?}", tree);
}
