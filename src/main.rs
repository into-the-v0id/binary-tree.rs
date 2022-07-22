// TODO: delete item in tree
// TODO: use into/into_iterator traits
// TODO: optimize/order tree

use tree::BinaryTree;
use std::iter::FromIterator;

mod tree;
mod node;

fn main() {
    let mut tree = BinaryTree::from_iter(&[10, 13, 15, 3, 5, 7, 17]);

    println!("{:#?}", tree.contains(&17));

    tree.insert(11);
    println!("{:#?}", tree.contains(&11));

    println!("{:#?}", tree);
}
