use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type Id = usize;

#[derive(Debug, Default)]
pub struct UnionFind {
    nodes: HashMap<Id, Rc<RefCell<UFNode>>>,
}

impl UnionFind {
    pub fn find(&mut self, id: Id) -> Id {
        self.find_node(id).borrow().id()
    }

    fn find_node(&mut self, id: Id) -> UFNodeRefCell {
        let node = self.nodes.entry(id);
        let node = Rc::clone(node.or_insert(Rc::new(RefCell::new(UFNode::Root { id }))));
        node
    }

    pub fn union(&mut self, id1: Id, id2: Id) -> Id {
        let r1 = uf_node_root_with_path_shortening(&self.find_node(id1));
        let r2 = uf_node_root_with_path_shortening(&self.find_node(id2));

        if r1 != r2 {
            *r1.borrow_mut() = UFNode::Next(r2);
        }
        r1.borrow().id()
    }
}

type UFNodeRefCell = Rc<RefCell<UFNode>>;

#[derive(Debug, PartialEq, Eq)]
enum UFNode {
    Root { id: Id },
    Next(UFNodeRefCell),
}

impl UFNode {
    fn id(&self) -> Id {
        match self {
            UFNode::Root { id } => *id,
            UFNode::Next(next) => next.borrow().id(),
        }
    }

    fn next(&self) -> Option<UFNodeRefCell> {
        match self {
            UFNode::Root { .. } => None,
            UFNode::Next(next) => Some(Rc::clone(next)),
        }
    }
}

fn uf_node_root_with_path_shortening(node: &UFNodeRefCell) -> UFNodeRefCell {
    let mut node_borrowed = node.borrow_mut();
    let next_root = match &mut *node_borrowed {
        UFNode::Root { .. } => return Rc::clone(node),
        UFNode::Next(next) => {
            let next_root = uf_node_root_with_path_shortening(next);
            next_root
        }
    };

    *node_borrowed = UFNode::Next(Rc::clone(&next_root));
    next_root
}

struct UFNodeIter {
    node: Option<UFNodeRefCell>,
}

impl UFNodeIter {
    fn from(node_cell: UFNodeRefCell) -> Self {
        Self {
            node: Some(node_cell),
        }
    }
}

impl Iterator for UFNodeIter {
    type Item = UFNodeRefCell;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.as_ref()?;
        let next = node.borrow().next();
        self.node = next.as_ref().map(Rc::clone);
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find_api() {
        let mut uf = UnionFind::default();

        assert_ne!(uf.find(1), uf.find(2));

        uf.union(1, 2);
        assert_eq!(uf.find(1), uf.find(2));

        assert_ne!(uf.find(3), uf.find(4));

        uf.union(3, 4);
        assert_eq!(uf.find(3), uf.find(4));

        uf.union(1, 4);
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(1), uf.find(3));
        assert_eq!(uf.find(1), uf.find(4));
    }

    #[test]
    fn test_union_find_depth() {
        let mut uf = UnionFind::default();

        uf.union(1, 2);
        uf.union(3, 4);
        uf.union(1, 4);

        // NOTE: all of them should have count() < 2, due to shortening
        assert_eq!(UFNodeIter::from(uf.find_node(1)).count(), 2);
        assert_eq!(UFNodeIter::from(uf.find_node(2)).count(), 1);
        assert_eq!(UFNodeIter::from(uf.find_node(3)).count(), 1);
        assert_eq!(UFNodeIter::from(uf.find_node(4)).count(), 0);
    }
}
