use std::{
    fmt::{self, Display},
    iter,
    sync::Arc,
};

use crate::kinds::SyntaxKind;

pub type GreenNode = Arc<GreenNodeData>;
#[derive(Clone, Debug)]
// either an internal tree node, or a leaf token
pub struct GreenNodeData {
    kind: SyntaxKind,
    text_len: usize, // either the token length, or the sum of the token lengths
    // NodeData fields
    children: Vec<GreenNode>,
    // Token fields
    text: Option<String>,
}

impl Display for GreenNodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for child in self.children() {
            if child.children.is_empty() {
                Display::fmt(child.text().unwrap(), f)?
            } else {
                child.fmt(f)?
            }
        }
        Ok(())
    }
}

impl GreenNodeData {
    /// make a new internal node
    pub fn new_node(kind: SyntaxKind, children: Vec<GreenNode>) -> Self {
        let text_len: usize = children
            .iter()
            .map(|child_node| child_node.text_len())
            .sum();
        GreenNodeData {
            kind,
            children,
            text: None,
            text_len,
        }
    }
    // make a new leaf node
    pub fn new_leaf(kind: SyntaxKind, text: String) -> Self {
        let text_len = text.len();
        GreenNodeData {
            kind,
            children: vec![],
            text: Some(text),
            text_len,
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn children(&self) -> impl Iterator<Item = GreenNode> {
        self.children.iter().cloned()
    }

    pub fn text(&self) -> Option<&str> {
        if let Some(a) = &self.text {
            return Some(a.as_str());
        }
        None
    }

    pub fn text_len(&self) -> usize {
        self.text_len
    }

    pub fn replace_child(&self, idx: usize, new_child: GreenNode) -> Self {
        assert!(self.children.len() > idx);

        let left = self.children().take(idx);
        let right = self.children().skip(idx + 1);
        let children: Vec<GreenNode> = left.chain(iter::once(new_child)).chain(right).collect();

        Self::new_node(self.kind, children)
    }
}
