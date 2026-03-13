use crate::element::element::Element;
use crate::element::handle::Handle;
use std::any::Any;

struct Slot {
    element: Box<dyn Element>,
    generation: u32,
}

pub struct Ui {
    slots: Vec<Option<Slot>>,
    children: Vec<Vec<Handle>>,
    root: Option<Handle>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            children: Vec::new(),
            root: None,
        }
    }

    pub fn add(&mut self, element: Box<dyn Element>) -> Handle {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                let generation = 0;
                let handle = Handle {
                    id: i as u32,
                    generation,
                };
                *slot = Some(Slot {
                    element,
                    generation,
                });
                return handle;
            }
        }
        let id = self.slots.len() as u32;
        let handle = Handle { id, generation: 0 };
        self.slots.push(Some(Slot {
            element,
            generation: 0,
        }));
        self.children.push(Vec::new());
        handle
    }

    pub fn append(&mut self, parent: Handle, child: Handle) {
        self.children[parent.id as usize].push(child);
    }

    pub fn set_root(&mut self, handle: Handle) {
        self.root = Some(handle);
    }

    pub fn root(&self) -> Option<Handle> {
        self.root
    }

    pub fn children(&self, handle: Handle) -> &[Handle] {
        &self.children[handle.id as usize]
    }

    pub fn get(&self, handle: Handle) -> Option<&dyn Element> {
        self.slots.get(handle.id as usize)?.as_ref().and_then(|s| {
            if s.generation == handle.generation {
                Some(s.element.as_ref())
            } else {
                None
            }
        })
    }

    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut dyn Element> {
        self.slots
            .get_mut(handle.id as usize)?
            .as_mut()
            .and_then(|s| {
                if s.generation == handle.generation {
                    Some(s.element.as_mut())
                } else {
                    None
                }
            })
    }
}

impl std::ops::Index<Handle> for Ui {
    type Output = dyn Element;
    fn index(&self, handle: Handle) -> &dyn Element {
        self.get(handle).expect("stale handle")
    }
}

impl std::ops::IndexMut<Handle> for Ui {
    fn index_mut(&mut self, handle: Handle) -> &mut dyn Element {
        self.get_mut(handle).expect("stale handle")
    }
}
