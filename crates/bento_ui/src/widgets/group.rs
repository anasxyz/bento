use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::RefCell, hash::Hash};

use bento_wgpu::{DrawCommand, RectDraw};

use crate::layout::Position;
use crate::{Owner, effect};
use crate::{
    layout::{Container, CrossAxis, Direction, MainAxis, Size},
    node::Node,
    reactive::signal::Signal,
    tree,
    view::{View, ViewId},
};

pub struct Group {
    children: Vec<Box<dyn View>>,
    pub direction: Direction,
    pub gap: f32,
    pub padding: f32,
    pub main_axis: MainAxis,
    pub cross_axis: CrossAxis,
    pub width: Size,
    pub height: Size,
    // deferred each setup
    each: Option<Box<dyn FnOnce(ViewId, Vec<ViewId>)>>,
    when: Vec<(Signal<bool>, Rc<dyn Fn() -> Box<dyn View>>)>,
}

impl Group {
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn direction(mut self, d: Direction) -> Self {
        self.direction = d;
        self
    }
    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }
    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }
    pub fn main_axis(mut self, m: MainAxis) -> Self {
        self.main_axis = m;
        self
    }
    pub fn cross_axis(mut self, c: CrossAxis) -> Self {
        self.cross_axis = c;
        self
    }
    pub fn width(mut self, size: Size) -> Self {
        self.width = size;
        self
    }
    pub fn height(mut self, size: Size) -> Self {
        self.height = size;
        self
    }

    pub fn when<V: View + 'static>(
        mut self,
        condition: Signal<bool>,
        view_fn: impl Fn() -> V + 'static,
    ) -> Self {
        self.when
            .push((condition, Rc::new(move || Box::new(view_fn()))));
        self
    }

    pub fn each<T, K, V, VF>(
        mut self,
        items: Signal<Vec<T>>,
        key_fn: impl Fn(&T) -> K + 'static,
        view_fn: VF,
    ) -> Self
    where
        T: Clone + 'static,
        K: Eq + Hash + Clone + 'static,
        VF: Fn(T) -> V + 'static,
        V: View + 'static,
    {
        let key_fn = Box::new(key_fn);
        let view_fn = Rc::new(move |item| Box::new(view_fn(item)) as Box<dyn View>);

        self.each = Some(Box::new(move |parent_id, static_children: Vec<ViewId>| {
            let nodes: Rc<RefCell<HashMap<K, ViewId>>> = Rc::new(RefCell::new(HashMap::new()));
            let nodes_clone = nodes.clone();

            effect(move || {
                let new_items = items.get();
                let mut current = nodes_clone.borrow_mut();

                let new_keys: Vec<K> = new_items.iter().map(|item| key_fn(item)).collect();
                let removed: Vec<K> = current
                    .keys()
                    .filter(|k| !new_keys.contains(k))
                    .cloned()
                    .collect();
                for key in removed {
                    if let Some(id) = current.remove(&key) {
                        tree::remove_node(id);
                    }
                }

                for item in &new_items {
                    let key = key_fn(item);
                    if !current.contains_key(&key) {
                        let item: T = item.clone();
                        let owner = Owner::new();
                        let id = (view_fn)(item).build();
                        let owner = owner.collect();
                        tree::store_owner(id, owner);
                        tree::append_child(parent_id, id);
                        current.insert(key, id);
                    }
                }

                // each items first, then static children
                let mut order: Vec<ViewId> = new_items
                    .iter()
                    .map(|item| current[&key_fn(item)])
                    .collect();
                order.extend_from_slice(&static_children);
                tree::reorder_children(parent_id, order);
            });
        }));

        self
    }
}

impl Container for Group {
    fn direction(&self) -> Direction {
        self.direction
    }
    fn gap(&self) -> f32 {
        self.gap
    }
    fn padding(&self) -> f32 {
        self.padding
    }
    fn main_axis(&self) -> MainAxis {
        self.main_axis
    }
    fn cross_axis(&self) -> CrossAxis {
        self.cross_axis
    }
}

impl View for Group {
    fn name(&self) -> &'static str {
        "Group"
    }

    fn as_container(&self) -> Option<&dyn Container> {
        Some(self)
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        vec![DrawCommand::Rect(RectDraw {
            x,
            y,
            w,
            h,
            color: [0.8, 0.2, 0.2, 1.0],
            radii: [0.0; 4],
            border_color: [0.0, 0.0, 0.0, 0.8],
            border_widths: [3.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        })]
    }

    fn build(self: Box<Self>) -> ViewId {
        let direction = self.direction;
        let gap = self.gap;
        let padding = self.padding;
        let main_axis = self.main_axis;
        let cross_axis = self.cross_axis;
        let each = self.each;
        let child_ids: Vec<ViewId> = self.children.into_iter().map(|c| c.build()).collect();

        let id = tree::add_node(Node {
            view: Box::new(Group {
                children: Vec::new(),
                direction,
                gap,
                padding,
                main_axis,
                cross_axis,
                width: self.width,
                height: self.height,
                each: None,
                when: Vec::new(),
            }),
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            handlers: Vec::new(),
            owners: Vec::new(),
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            layout_dirty: true,
            width: self.width,
            height: self.height,
            position: Position::Relative,
            last_available_w: -1.0,
            last_available_h: -1.0,
        });

        for child_id in &child_ids {
            tree::append_child(id, *child_id);
        }

        if let Some(setup) = each {
            setup(id, child_ids);
        }

        // when stuff
        for (condition, view_fn) in self.when {
            let current: Rc<RefCell<Option<ViewId>>> = Rc::new(RefCell::new(None));
            let current_clone = current.clone();

            effect(move || {
                let show = condition.get();
                let mut current = current_clone.borrow_mut();

                if show {
                    if current.is_none() {
                        let owner = Owner::new();
                        let child_id = (view_fn)().build();
                        let owner = owner.collect();
                        tree::store_owner(child_id, owner);
                        tree::append_child(id, child_id);
                        *current = Some(child_id);
                    }
                } else {
                    if let Some(child_id) = current.take() {
                        tree::remove_node(child_id);
                    }
                }
            });
        }

        id
    }
}

pub fn group() -> Group {
    Group {
        children: Vec::new(),
        direction: Direction::Row,
        gap: 0.0,
        padding: 0.0,
        main_axis: MainAxis::Start,
        cross_axis: CrossAxis::Start,
        width: Size::Auto,
        height: Size::Auto,
        each: None,
        when: Vec::new(),
    }
}
