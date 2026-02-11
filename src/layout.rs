use crate::widgets::Widget;

#[derive(Clone, Copy)]
enum Direction { H, V }

/// A child slot in a container — either a widget or a nested container.
enum Child {
    Widget(usize),
    Container(usize),
}

struct Container {
    direction: Direction,
    x: f32,
    y: f32,
    padding: f32,
    gap: f32,
    children: Vec<Child>,
}

impl Container {
    /// Compute layout, returning (total_width, total_height) of this container's content.
    fn compute(&self, containers: &[Container], widgets: &mut Vec<Box<dyn Widget>>) -> (f32, f32) {
        let mut cursor = self.padding;
        let mut max_cross: f32 = 0.0;

        for child in &self.children {
            match child {
                Child::Widget(id) => {
                    if let Some(w) = widgets.iter_mut().find(|w| w.id() == *id) {
                        let mut b = w.bounds();
                        match self.direction {
                            Direction::H => {
                                b.x = self.x + cursor;
                                b.y = self.y + self.padding;
                                cursor += b.w + self.gap;
                                max_cross = max_cross.max(b.h);
                            }
                            Direction::V => {
                                b.x = self.x + self.padding;
                                b.y = self.y + cursor;
                                cursor += b.h + self.gap;
                                max_cross = max_cross.max(b.w);
                            }
                        }
                        w.set_bounds(b);
                    }
                }
                Child::Container(child_idx) => {
                    // Temporarily set child container's origin, then recurse.
                    let child_container = &containers[*child_idx];
                    // We need to get the child's size before we can advance cursor,
                    // so we do a dry-run first to measure, then a real pass.
                    let (cw, ch) = measure_container(child_container, containers, widgets);
                    match self.direction {
                        Direction::H => {
                            // Child container positioned at current cursor
                            set_container_origin(*child_idx, self.x + cursor, self.y + self.padding, containers, widgets);
                            cursor += cw + self.gap;
                            max_cross = max_cross.max(ch);
                        }
                        Direction::V => {
                            set_container_origin(*child_idx, self.x + self.padding, self.y + cursor, containers, widgets);
                            cursor += ch + self.gap;
                            max_cross = max_cross.max(cw);
                        }
                    }
                }
            }
        }

        match self.direction {
            Direction::H => (cursor - self.gap + self.padding, max_cross + self.padding * 2.0),
            Direction::V => (max_cross + self.padding * 2.0, cursor - self.gap + self.padding),
        }
    }
}

/// Measure a container's total size without modifying widget bounds.
fn measure_container(c: &Container, containers: &[Container], widgets: &[Box<dyn Widget>]) -> (f32, f32) {
    let mut cursor = c.padding;
    let mut max_cross: f32 = 0.0;
    for child in &c.children {
        match child {
            Child::Widget(id) => {
                if let Some(w) = widgets.iter().find(|w| w.id() == *id) {
                    let b = w.bounds();
                    match c.direction {
                        Direction::H => { cursor += b.w + c.gap; max_cross = max_cross.max(b.h); }
                        Direction::V => { cursor += b.h + c.gap; max_cross = max_cross.max(b.w); }
                    }
                }
            }
            Child::Container(child_idx) => {
                let (cw, ch) = measure_container(&containers[*child_idx], containers, widgets);
                match c.direction {
                    Direction::H => { cursor += cw + c.gap; max_cross = max_cross.max(ch); }
                    Direction::V => { cursor += ch + c.gap; max_cross = max_cross.max(cw); }
                }
            }
        }
    }
    match c.direction {
        Direction::H => (cursor - c.gap + c.padding, max_cross + c.padding * 2.0),
        Direction::V => (max_cross + c.padding * 2.0, cursor - c.gap + c.padding),
    }
}

/// Reposition a container at a new origin and re-run its layout.
fn set_container_origin(idx: usize, x: f32, y: f32, containers: &[Container], widgets: &mut Vec<Box<dyn Widget>>) {
    // We can't mutate containers[idx].x/y here (immutable borrow), so we
    // manually walk children using the provided x/y as origin.
    let container = &containers[idx];
    let mut cursor = container.padding;
    for child in &container.children {
        match child {
            Child::Widget(id) => {
                if let Some(w) = widgets.iter_mut().find(|w| w.id() == *id) {
                    let mut b = w.bounds();
                    match container.direction {
                        Direction::H => { b.x = x + cursor; b.y = y + container.padding; cursor += b.w + container.gap; }
                        Direction::V => { b.x = x + container.padding; b.y = y + cursor; cursor += b.h + container.gap; }
                    }
                    w.set_bounds(b);
                }
            }
            Child::Container(child_idx) => {
                let (cw, ch) = measure_container(&containers[*child_idx], containers, widgets);
                match container.direction {
                    Direction::H => { set_container_origin(*child_idx, x + cursor, y + container.padding, containers, widgets); cursor += cw + container.gap; }
                    Direction::V => { set_container_origin(*child_idx, x + container.padding, y + cursor, containers, widgets); cursor += ch + container.gap; }
                }
            }
        }
    }
}

pub struct ContainerBuilder<'a> {
    manager: &'a mut LayoutManager,
    index: usize,
}

impl<'a> ContainerBuilder<'a> {
    pub fn position(self, x: f32, y: f32) -> Self {
        let c = &mut self.manager.containers[self.index];
        c.x = x;
        c.y = y;
        self
    }

    pub fn padding(self, padding: f32) -> Self {
        self.manager.containers[self.index].padding = padding;
        self
    }

    pub fn gap(self, gap: f32) -> Self {
        self.manager.containers[self.index].gap = gap;
        self
    }

    pub fn add<T: crate::widgets::Widget>(self, handle: crate::widgets::WidgetHandle<T>) -> Self {
        self.manager.containers[self.index].children.push(Child::Widget(handle.id));
        self
    }

    /// Nest another container inside this one.
    pub fn add_container(self, other: ContainerRef) -> Self {
        self.manager.containers[self.index].children.push(Child::Container(other.index));
        self
    }

    pub fn as_ref(&self) -> ContainerRef {
        ContainerRef { index: self.index }
    }
}

/// A handle to a built container, for nesting inside another container.
#[derive(Clone, Copy)]
pub struct ContainerRef {
    pub(crate) index: usize,
}

pub struct LayoutManager {
    containers: Vec<Container>,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self { containers: Vec::new() }
    }

    pub fn hstack(&mut self) -> ContainerBuilder {
        let index = self.containers.len();
        self.containers.push(Container {
            direction: Direction::H,
            x: 0.0, y: 0.0,
            padding: 0.0, gap: 0.0,
            children: Vec::new(),
        });
        ContainerBuilder { manager: self, index }
    }

    pub fn vstack(&mut self) -> ContainerBuilder {
        let index = self.containers.len();
        self.containers.push(Container {
            direction: Direction::V,
            x: 0.0, y: 0.0,
            padding: 0.0, gap: 0.0,
            children: Vec::new(),
        });
        ContainerBuilder { manager: self, index }
    }

    pub fn compute_all(&self, widgets: &mut Vec<Box<dyn Widget>>) {
        // Find which containers are nested inside others — only run compute on roots.
        let mut is_child = vec![false; self.containers.len()];
        for c in &self.containers {
            for child in &c.children {
                if let Child::Container(idx) = child {
                    is_child[*idx] = true;
                }
            }
        }
        for (idx, container) in self.containers.iter().enumerate() {
            if !is_child[idx] {
                container.compute(&self.containers, widgets);
            }
        }
    }
}
