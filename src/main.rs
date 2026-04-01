mod kinds;

use std::{
    fmt::{self, Display},
    iter,
    sync::Arc,
};

// token/node uuid
#[derive(Clone, Copy, Debug)]
pub struct SyntaxKind(u16);

pub type Node = Arc<NodeData>;
#[derive(Clone, Debug)]
// either an internal tree node, or a leaf token
pub struct NodeData {
    kind: SyntaxKind,
    // NodeData fields
    children: Vec<Node>,
    len: usize,
    // Token fields
    text: Option<String>,
    text_len: Option<usize>,
}

impl Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for child in self.children() {
            if child.children().is_empty() {
                Display::fmt(child.text().unwrap(), f)?
            } else {
                child.fmt(f)?
            }
        }
        Ok(())
    }
}

impl NodeData {
    /// make a new internal node
    pub fn new_node(kind: SyntaxKind, children: Vec<Node>) -> Self {
        let len: usize = children.iter().map(|child_node| child_node.len()).sum();
        NodeData {
            kind,
            children,
            len,
            text: None,
            text_len: None,
        }
    }
    // make a new leaf node
    pub fn new_leaf(kind: SyntaxKind, text: String) -> Self {
        let text_len = text.len();
        NodeData {
            kind,
            children: vec![],
            len: 0, // todo, check if this can be cleaner
            text: Some(text),
            text_len: Some(text_len),
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn children(&self) -> &[Node] {
        self.children.as_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn text(&self) -> Option<&str> {
        if let Some(a) = &self.text {
            return Some(a.as_str());
        }
        None
    }

    pub fn text_len(&self) -> Option<usize> {
        if let Some(a) = &self.text {
            return Some(a.len());
        }
        None
    }

    pub fn replace_child(&self, idx: usize, new_child: Node) -> Self {
        assert!(self.children().len() > idx);

        let left = self.children().iter().take(idx).cloned();
        let right = self.children().iter().skip(idx + 1).cloned();
        let children: Vec<Node> = left.chain(iter::once(new_child)).chain(right).collect();

        Self::new_node(self.kind, children)
    }
}

fn main() {}

#[test]
fn test() {
    // "1 + 2 * 1 + 2"
    let space = Arc::new(NodeData::new_leaf(kinds::WHITESPACE, " ".to_string()));
    let one = Arc::new(NodeData::new_leaf(kinds::INT, "1".to_string()));
    let two = Arc::new(NodeData::new_leaf(kinds::INT, "2".to_string()));
    let star = Arc::new(NodeData::new_leaf(kinds::STAR, "*".to_string()));
    let plus = Arc::new(NodeData::new_leaf(kinds::PLUS, "+".to_string()));

    let add = Arc::new(NodeData::new_node(
        kinds::BIN_EXPR,
        vec![one, space.clone(), plus, space.clone(), two],
    ));
    let mul = Arc::new(NodeData::new_node(
        kinds::BIN_EXPR,
        vec![add.clone(), space.clone(), star, space, add],
    ));
    // println!("{:#?}", mul);
    // println!("{}", mul);
    assert_eq!("1 + 2 * 1 + 2" , mul.to_string().as_str());
}
