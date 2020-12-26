// TODO: Search in binary tree (int)

#[derive(Clone, Debug)]
pub struct Node<T> {
    data: T,
    left: Box<Option<Node<T>>>,
    right: Box<Option<Node<T>>>,
}

impl <T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    pub fn set_left(&mut self, node: Node<T>) {
        self.left = Box::new(Some(node));
    }

    pub fn unset_left(&mut self) {
        self.left = Box::new(None);
    }

    pub fn left(&self) -> Option<&Node<T>> {
        self.left.as_ref().as_ref()
    }

    pub fn left_mut(&mut self) -> Option<&mut Node<T>> {
        self.left.as_mut().as_mut()
    }

    pub fn set_right(&mut self, node: Node<T>) {
        self.right = Box::new(Some(node));
    }

    pub fn unset_right(&mut self) {
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
}

fn main() {
    let mut tree = Node::new(10);
    tree.set_left({
        let mut node = Node::new(5);
        node.set_left(Node::new(3));
        node.set_right(Node::new(7));

        node
    });
    tree.set_right({
        let mut node = Node::new(15);
        node.set_left(Node::new(13));
        node.set_right(Node::new(17));

        node
    });
}
