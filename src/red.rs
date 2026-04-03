use std::{
    fmt::{self, Display},
    iter,
    sync::Arc,
};

use crate::{
    kinds::SyntaxKind,
    green::GreenNode,
};

pub type RedNode = Arc<RedNodeData>;
#[derive(Clone, Debug)]
// either an internal tree node, or a leaf token
pub struct RedNodeData {
    kind: SyntaxKind,
    // RedNodeData fields
    parent: Option<RedNode>,
    green: GreenNode,
    len: usize,
}

impl Display for RedNodeData {
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

impl RedNodeData {
    /// make a new internal node
    pub fn new_node(kind: SyntaxKind, children: Vec<RedNode>) -> Self {
        let len: usize = children.iter().map(|child_node| child_node.len()).sum();
        RedNodeData {
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
        RedNodeData {
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

    pub fn children(&self) -> &[RedNode] {
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

    pub fn replace_child(&self, idx: usize, new_child: RedNode) -> Self {
        assert!(self.children().len() > idx);

        let left = self.children().iter().take(idx).cloned();
        let right = self.children().iter().skip(idx + 1).cloned();
        let children: Vec<RedNode> = left.chain(iter::once(new_child)).chain(right).collect();

        Self::new_node(self.kind, children)
    }
}
