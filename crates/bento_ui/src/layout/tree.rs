use crate::layout::types::Layout;

pub struct LayoutNode {
    pub layout: Layout,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub dirty: bool,
    // index back into Ui::slots
    pub slot: usize,  
}

pub struct LayoutTree {
    pub nodes: Vec<LayoutNode>,
}

impl LayoutTree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add(&mut self, slot: usize, parent: Option<usize>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(LayoutNode {
            layout: Layout::default(),
            children: Vec::new(),
            parent,
            dirty: true,
            slot,
        });
        if let Some(p) = parent {
            self.nodes[p].children.push(index);
        }
        index
    }

    pub fn remove(&mut self, index: usize) {
        // detach from parent
        if let Some(p) = self.nodes[index].parent {
            self.nodes[p].children.retain(|&c| c != index);
        }
        // recursively remove children
        let children = self.nodes[index].children.clone();
        for child in children {
            self.remove(child);
        }
        // clear the node but keep the slot to avoid index shifting
        // mark slot as usize::MAX to indicate removed
        self.nodes[index].slot = usize::MAX;
        self.nodes[index].children.clear();
        self.nodes[index].parent = None;
    }

    pub fn mark_dirty(&mut self, index: usize) {
        self.nodes[index].dirty = true;
        // bubble up so ancestors know a descendant changed
        let mut current = self.nodes[index].parent;
        while let Some(p) = current {
            self.nodes[p].dirty = true;
            current = self.nodes[p].parent;
        }
    }

    pub fn any_dirty(&self) -> bool {
        self.nodes.iter().any(|n| n.dirty && n.slot != usize::MAX)
    }
}
