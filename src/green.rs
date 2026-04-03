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
    // GreenNodeData fields
    children: Vec<GreenNode>,
    len: usize,
    // Token fields
    text: Option<String>,
    text_len: Option<usize>,
}

impl Display for GreenNodeData {
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

impl GreenNodeData {
    /// make a new internal node
    pub fn new_node(kind: SyntaxKind, children: Vec<GreenNode>) -> Self {
        let len: usize = children.iter().map(|child_node| child_node.len()).sum();
        GreenNodeData {
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
        GreenNodeData {
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

    pub fn children(&self) -> &[GreenNode] {
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

    pub fn replace_child(&self, idx: usize, new_child: GreenNode) -> Self {
        assert!(self.children().len() > idx);

        let left = self.children().iter().take(idx).cloned();
        let right = self.children().iter().skip(idx + 1).cloned();
        let children: Vec<GreenNode> = left.chain(iter::once(new_child)).chain(right).collect();

        Self::new_node(self.kind, children)
    }
}
