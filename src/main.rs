mod kinds;
mod green;
mod red;

use std::{
    iter,
    sync::Arc,
};
use crate::{
    {green::GreenNodeData}
};

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

    let add = Arc::new(GreenNodeData::new_node(
        kinds::BIN_EXPR,
        vec![one, space.clone(), plus, space.clone(), two],
    ));
    let mul = Arc::new(GreenNodeData::new_node(
        kinds::BIN_EXPR,
        vec![add.clone(), space.clone(), star, space, add],
    ));
    // println!("{:#?}", mul);
    // println!("{}", mul);
    assert_eq!("1 + 2 * 1 + 2" , mul.to_string().as_str());
}
