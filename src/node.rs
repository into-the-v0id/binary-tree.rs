use std::fmt::Debug;

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

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
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

    pub fn left_mut(&mut self) -> Option<&mut Node<T>> {
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

    pub fn right_mut(&mut self) -> Option<&mut Node<T>> {
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
        if node.data == data {
            return;
        }

        if data < node.data {
            if let Some(left_node) = node.left_mut() {
                Self::insert_data(left_node, data);
                return;
            } else {
                node.set_left(Node::new(data));
                return;
            }
        }

        if data > node.data {
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
