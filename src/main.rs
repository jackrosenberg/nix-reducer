mod green;
mod kinds;
mod red;

use crate::{green::GreenNodeData, red::RedNodeData};
use std::{iter, sync::Arc};

/*
 green tree:
    - holds the actual content
 red tree:
    - can traverse up the tree
    - computes the length of each node
*/

fn main() {}

#[test]
fn test() {
    // "1 + 2 * 1 + 2"
    let space = Arc::new(GreenNodeData::new_leaf(kinds::WHITESPACE, " ".to_string()));
    let one = Arc::new(GreenNodeData::new_leaf(kinds::INT, "1".to_string()));
    let two = Arc::new(GreenNodeData::new_leaf(kinds::INT, "2".to_string()));
    let star = Arc::new(GreenNodeData::new_leaf(kinds::STAR, "*".to_string()));
    let plus = Arc::new(GreenNodeData::new_leaf(kinds::PLUS, "+".to_string()));
    let three = Arc::new(GreenNodeData::new_leaf(kinds::INT, "3".to_string()));

    let add = Arc::new(GreenNodeData::new_node(
        kinds::BIN_EXPR,
        vec![one, space.clone(), plus, space.clone(), two],
    ));
    let mul = Arc::new(GreenNodeData::new_node(
        kinds::BIN_EXPR,
        vec![add.clone(), space.clone(), star, space, add.clone()],
    ));
    // println!("{:#?}", mul);
    // println!("{}", mul);
    assert_eq!("1 + 2 * 1 + 2", mul.to_string().as_str());
    // println!("add child: {:?}", add.children().next().unwrap());

    let mul = RedNodeData::new_root(mul);
    let mul = mul.children().next().unwrap().replace_child(0, three);
    let node = mul.children().nth(4).unwrap();

    // println!("{}", node);
    // println!("{}", node.text_offset());
    assert_eq!("3 + 2 * 1 + 2", node.parent().unwrap().to_string().as_str());
}
