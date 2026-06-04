use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::RefCell, hash::Hash};

use bento_wgpu::DrawCommand;
use taffy::prelude::*;

use crate::layout::LayoutProps;
use crate::node::{self, Node};
use crate::reactive::signal::Signal;
use crate::tree;
use crate::view::{View, ViewId};
use crate::{Owner, effect};

pub struct Group {
    children: Vec<Box<dyn View>>,
    layout: LayoutProps,
    each: Option<Box<dyn FnOnce(ViewId, Vec<ViewId>)>>,
    when: Vec<(Signal<bool>, Rc<dyn Fn() -> Box<dyn View>>)>,
}

impl Group {
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn direction(mut self, d: FlexDirection) -> Self {
        self.layout.flex_direction = d;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.layout.gap = Size {
            width: LengthPercentage::length(gap),
            height: LengthPercentage::length(gap),
        };
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.layout.padding = Rect {
            left: LengthPercentage::length(p),
            right: LengthPercentage::length(p),
            top: LengthPercentage::length(p),
            bottom: LengthPercentage::length(p),
        };
        self
    }

    pub fn width(mut self, width: Dimension) -> Self {
        self.layout.width = width;
        self
    }

    pub fn height(mut self, height: Dimension) -> Self {
        self.layout.height = height;
        self
    }

    pub fn align_items(mut self, v: AlignItems) -> Self {
        self.layout.align_items = Some(v);
        self
    }

    pub fn justify_content(mut self, v: JustifyContent) -> Self {
        self.layout.justify_content = Some(v);
        self
    }

    pub fn align_content(mut self, v: AlignContent) -> Self {
        self.layout.align_content = Some(v);
        self
    }

    pub fn flex_wrap(mut self, v: FlexWrap) -> Self {
        self.layout.flex_wrap = v;
        self
    }

    pub fn display(mut self, v: Display) -> Self {
        self.layout.display = v;
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

impl View for Group {
    fn name(&self) -> &'static str {
        "Group"
    }

    fn render(&self, _x: f32, _y: f32, _w: f32, _h: f32) -> Vec<DrawCommand> {
        vec![]
    }

    fn build(self: Box<Self>) -> ViewId {
        let layout = self.layout.clone();
        let each = self.each;
        let when = self.when;
        let child_ids: Vec<ViewId> = self.children.into_iter().map(|c| c.build()).collect();

        let id = tree::add_node(Node {
            view: Box::new(Group {
                children: Vec::new(),
                layout: layout.clone(),
                each: None,
                when: Vec::new(),
            }),
            taffy_id: node::placeholder_taffy_id(),
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            layout,
            handlers: Vec::new(),
            owners: Vec::new(),
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
        });

        for child_id in &child_ids {
            tree::append_child(id, *child_id);
        }

        if let Some(setup) = each {
            setup(id, child_ids);
        }

        for (condition, view_fn) in when {
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
        layout: LayoutProps::default(),
        each: None,
        when: Vec::new(),
    }
}
