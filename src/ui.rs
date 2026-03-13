use crate::element::element::Element;
use crate::element::handle::Handle;

struct Slot {
    element: Box<dyn Element + 'static>,
    generation: u32,
}

pub struct Ui {
    slots: Vec<Option<Slot>>,
    children: Vec<Vec<Handle<()>>>,
    root: Option<Handle<()>>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            children: Vec::new(),
            root: None,
        }
    }

    pub fn add<T: Element + 'static>(&mut self, element: T) -> Handle<T> {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                let handle = Handle::new(i as u32, 0);
                *slot = Some(Slot {
                    element: Box::new(element),
                    generation: 0,
                });
                self.children[i] = Vec::new();
                return handle;
            }
        }
        let id = self.slots.len() as u32;
        self.slots.push(Some(Slot {
            element: Box::new(element),
            generation: 0,
        }));
        self.children.push(Vec::new());
        Handle::new(id, 0)
    }

    pub fn append<P: Element + 'static, C: Element + 'static>(
        &mut self,
        parent: Handle<P>,
        child: Handle<C>,
    ) {
        self.children[parent.id as usize].push(Handle::new(child.id, child.generation));
    }

    pub fn set_root<T: Element + 'static>(&mut self, handle: Handle<T>) {
        self.root = Some(Handle::new(handle.id, handle.generation));
    }

    pub fn root(&self) -> Option<Handle<()>> {
        self.root
    }

    pub fn children<T>(&self, handle: Handle<T>) -> &[Handle<()>] {
        &self.children[handle.id as usize]
    }

    pub fn get<T: Element + 'static>(&self, handle: Handle<T>) -> Option<&T> {
        self.slots.get(handle.id as usize)?.as_ref().and_then(|s| {
            if s.generation == handle.generation {
                s.element.as_any().downcast_ref::<T>()
            } else {
                None
            }
        })
    }

    pub fn get_mut<T: Element + 'static>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.slots
            .get_mut(handle.id as usize)?
            .as_mut()
            .and_then(|s| {
                if s.generation == handle.generation {
                    s.element.as_any_mut().downcast_mut::<T>()
                } else {
                    None
                }
            })
    }

    pub fn get_dyn(&self, handle: Handle<()>) -> Option<&dyn Element> {
        self.slots.get(handle.id as usize)?.as_ref().and_then(|s| {
            if s.generation == handle.generation {
                Some(s.element.as_ref())
            } else {
                None
            }
        })
    }

    pub fn get_dyn_mut(&mut self, handle: Handle<()>) -> Option<&mut (dyn Element + 'static)> {
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

impl<T: Element + 'static> std::ops::Index<Handle<T>> for Ui {
    type Output = T;
    fn index(&self, handle: Handle<T>) -> &T {
        self.get(handle).expect("stale handle")
    }
}

impl<T: Element + 'static> std::ops::IndexMut<Handle<T>> for Ui {
    fn index_mut(&mut self, handle: Handle<T>) -> &mut T {
        self.get_mut(handle).expect("stale handle")
    }
}
