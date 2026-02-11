use crate::widgets::Widget;

#[derive(Clone, Copy)]
enum Direction { H, V }

struct Container {
    direction: Direction,
    x: f32,
    y: f32,
    padding: f32,
    gap: f32,
    widget_ids: Vec<usize>,
}

impl Container {
    fn compute(&self, widgets: &mut Vec<Box<dyn Widget>>) {
        let mut cursor = self.padding;
        for id in &self.widget_ids {
            if let Some(w) = widgets.iter_mut().find(|w| w.id() == *id) {
                let mut b = w.bounds();
                match self.direction {
                    Direction::H => {
                        b.x = self.x + cursor;
                        b.y = self.y + self.padding;
                        cursor += b.w + self.gap;
                    }
                    Direction::V => {
                        b.x = self.x + self.padding;
                        b.y = self.y + cursor;
                        cursor += b.h + self.gap;
                    }
                }
                w.set_bounds(b);
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
        self.manager.containers[self.index].widget_ids.push(handle.id);
        self
    }
}

pub struct LayoutManager {
    containers: Vec<Container>,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self { containers: Vec::new() }
    }

    pub fn hstack(&mut self) -> ContainerBuilder {
        self.containers.push(Container {
            direction: Direction::H,
            x: 0.0, y: 0.0,
            padding: 0.0, gap: 0.0,
            widget_ids: Vec::new(),
        });
        let index = self.containers.len() - 1;
        ContainerBuilder { manager: self, index }
    }

    pub fn vstack(&mut self) -> ContainerBuilder {
        self.containers.push(Container {
            direction: Direction::V,
            x: 0.0, y: 0.0,
            padding: 0.0, gap: 0.0,
            widget_ids: Vec::new(),
        });
        let index = self.containers.len() - 1;
        ContainerBuilder { manager: self, index }
    }

    pub fn compute_all(&self, widgets: &mut Vec<Box<dyn Widget>>) {
        for container in &self.containers {
            container.compute(widgets);
        }
    }
}
