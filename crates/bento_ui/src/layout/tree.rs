use crate::layout::convert::to_taffy_style;
use crate::layout::types::Layout;
use taffy::prelude::*;

pub struct LayoutNode {
    pub layout: Layout,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub dirty: bool,
    // index back into Ui::slots
    pub slot: usize,
    pub taffy_id: Option<NodeId>,
}

pub struct LayoutTree {
    pub nodes: Vec<LayoutNode>,
    pub taffy: TaffyTree<usize>,
}

impl LayoutTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            taffy: TaffyTree::new(),
        }
    }

    pub fn add(&mut self, slot: usize, parent: Option<usize>) -> usize {
        let index = self.nodes.len();

        // always create as internal node (can have children and be a leaf)
        let taffy_id = self
            .taffy
            .new_leaf_with_context(Style::default(), index)
            .unwrap();

        // set initial style from default layout
        let initial_style = to_taffy_style(&Layout::default());
        self.taffy.set_style(taffy_id, initial_style).unwrap();

        self.nodes.push(LayoutNode {
            layout: Layout::default(),
            children: Vec::new(),
            parent,
            dirty: true,
            slot,
            taffy_id: Some(taffy_id),
        });

        // attach to parent after pushing so index is valid
        if let Some(p) = parent {
            self.nodes[p].children.push(index);
            if let Some(parent_taffy_id) = self.nodes[p].taffy_id {
                let mut children = self.taffy.children(parent_taffy_id).unwrap();
                children.push(taffy_id);
                self.taffy.set_children(parent_taffy_id, &children).unwrap();
            }
        }

        index
    }

    pub fn remove(&mut self, index: usize) {
        // detach from parent in taffy
        if let Some(p) = self.nodes[index].parent {
            if let (Some(parent_taffy_id), Some(child_taffy_id)) =
                (self.nodes[p].taffy_id, self.nodes[index].taffy_id)
            {
                let children: Vec<NodeId> = self
                    .taffy
                    .children(parent_taffy_id)
                    .unwrap()
                    .into_iter()
                    .filter(|&c| c != child_taffy_id)
                    .collect();
                self.taffy.set_children(parent_taffy_id, &children).unwrap();
            }
            self.nodes[p].children.retain(|&c| c != index);
        }

        // recursively remove children
        let children = self.nodes[index].children.clone();
        for child in children {
            self.remove(child);
        }

        // remove from taffy
        if let Some(taffy_id) = self.nodes[index].taffy_id {
            self.taffy.remove(taffy_id).unwrap();
        }

        // mark as removed
        self.nodes[index].slot = usize::MAX;
        self.nodes[index].children.clear();
        self.nodes[index].parent = None;
        self.nodes[index].taffy_id = None;
    }

    pub fn mark_dirty(&mut self, index: usize) {
        self.nodes[index].dirty = true;
        if let Some(taffy_id) = self.nodes[index].taffy_id {
            self.taffy.mark_dirty(taffy_id).unwrap();
        }
        // bubble up
        let mut current = self.nodes[index].parent;
        while let Some(p) = current {
            self.nodes[p].dirty = true;
            current = self.nodes[p].parent;
        }
    }

    pub fn sync_styles(&mut self) -> bool {
        let mut any = false;
        for node in &mut self.nodes {
            if node.slot == usize::MAX {
                continue;
            }
            if !node.dirty {
                continue;
            }
            let Some(taffy_id) = node.taffy_id else {
                continue;
            };
            let style = to_taffy_style(&node.layout);
            self.taffy.set_style(taffy_id, style).unwrap();
            node.dirty = false;
            any = true;
        }
        any
    }

    pub fn any_dirty(&self) -> bool {
        self.nodes.iter().any(|n| n.dirty && n.slot != usize::MAX)
    }
}
