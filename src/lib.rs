// Copyright (C) Oliver Amann
//
// Licensed under the MIT License or the Apache License Version 2.0,
// at your option. You may not use this file except according to
// those terms.

// TODO: delete item in tree
// TODO: use into/into_iterator traits
// TODO: optimize/order tree

#![allow(dead_code)]

pub use tree::BinaryTree;
pub use node::Node;

mod tree;
mod node;
