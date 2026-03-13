use crate::element::element::{Element, ElementType};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Handle {
    id: u32,
    generation: u32,
}

struct Slot {
    element: Element,
    generation: u32,
    children: Vec<Handle>,
    parent: Option<Handle>,
}

pub struct Ui {
    slots: Vec<Option<Slot>>,
    root: Option<Handle>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            root: None,
        }
    }

    pub fn add(&mut self, element: Element) -> Handle {
        // find an empty slot first
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                let generation = 0; // will increment on reuse
                let handle = Handle {
                    id: i as u32,
                    generation,
                };
                *slot = Some(Slot {
                    element,
                    generation,
                    children: Vec::new(),
                    parent: None,
                });
                return handle;
            }
        }
        // no empty slots, push a new one
        let id = self.slots.len() as u32;
        let handle = Handle { id, generation: 0 };
        self.slots.push(Some(Slot {
            element,
            generation: 0,
            children: Vec::new(),
            parent: None,
        }));
        handle
    }

    pub fn remove(&mut self, handle: Handle) {
        if let Some(slot) = self.slots.get_mut(handle.id as usize) {
            if let Some(s) = slot {
                if s.generation == handle.generation {
                    // bump generation so old handles are stale
                    let next_gen = s.generation + 1;
                    *slot = None;
                    // put a tombstone generation back so reuse increments correctly
                    _ = next_gen; // will use this in a moment
                }
            }
        }
    }

    pub fn append(&mut self, parent: Handle, child: Handle) {
        // set child's parent
        if let Some(Some(child_slot)) = self.slots.get_mut(child.id as usize) {
            child_slot.parent = Some(parent);
        }
        // add to parent's children
        if let Some(Some(parent_slot)) = self.slots.get_mut(parent.id as usize) {
            parent_slot.children.push(child);
        }
    }

    pub fn set_root(&mut self, handle: Handle) {
        self.root = Some(handle);
    }

    pub fn get(&self, handle: Handle) -> Option<&Element> {
        self.slots.get(handle.id as usize)?.as_ref().and_then(|s| {
            if s.generation == handle.generation {
                Some(&s.element)
            } else {
                None
            }
        })
    }

    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut Element> {
        self.slots
            .get_mut(handle.id as usize)?
            .as_mut()
            .and_then(|s| {
                if s.generation == handle.generation {
                    Some(&mut s.element)
                } else {
                    None
                }
            })
    }

    pub fn children(&self, handle: Handle) -> &[Handle] {
        self.slots
            .get(handle.id as usize)
            .and_then(|s| s.as_ref())
            .map(|s| s.children.as_slice())
            .unwrap_or(&[])
    }

    pub fn root(&self) -> Option<Handle> {
        self.root
    }
}

// ui[btn] syntax
impl Index<Handle> for Ui {
    type Output = Element;
    fn index(&self, handle: Handle) -> &Element {
        self.get(handle).expect("stale handle")
    }
}

impl IndexMut<Handle> for Ui {
    fn index_mut(&mut self, handle: Handle) -> &mut Element {
        self.get_mut(handle).expect("stale handle")
    }
}
