use std::{
    fmt::{self, Display},
    iter,
    rc::Rc,
    sync::Arc,
};

use crate::{green::GreenNode, kinds::SyntaxKind};

pub type RedNode = Rc<RedNodeData>;
#[derive(Clone, Debug)]
pub struct RedNodeData {
    parent: Option<RedNode>,
    index_in_parent: usize,
    green: GreenNode,
    text_offset: usize,
}

impl Display for RedNodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.green(), f)
    }
}

impl RedNodeData {
    pub fn new_root(root: GreenNode) -> RedNode {
        Rc::new(RedNodeData {
            parent: None,
            index_in_parent: 0,
            text_offset: 0,
            green: root,
        })
    }

    pub fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }

    pub fn children<'a>(self: &'a RedNode) -> impl Iterator<Item = RedNode> + 'a {
        let mut offset_in_parent = 0;
        self.green().children().enumerate().map(move |(index_in_parent, child): (usize, GreenNode)| {
            let text_offset = self.text_offset() + offset_in_parent;
            offset_in_parent += child.text_len();
            Rc::new(RedNodeData {
                parent: Some(Rc::clone(self)),
                index_in_parent,
                green: child,
                text_offset,
            })
        })
    }

    /// replaces a child and return the new ROOT of tree
    pub fn replace_child(self: &RedNode, idx: usize, new_child: GreenNode) -> RedNode {
        let new_green = self.green().replace_child(idx, new_child).into();
        match &self.parent {
            Some(parent) => parent.replace_child(self.index_in_parent, new_green),
            None => RedNodeData::new_root(new_green)
        }
    }


    pub fn green(&self) -> &GreenNode {
        &self.green
    }

    pub fn text_len(&self) -> usize {
        self.green().text_len()
    }

    pub fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }

    pub fn text_offset(&self) -> usize {
        self.text_offset
    }
}
