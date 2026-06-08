use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::RefCell, hash::Hash};

use bento_wgpu::{DrawCommand, RectDraw};
use taffy::prelude::*;

use crate::events::MouseScroll;
use crate::layout::{LayoutProps, Val};
use crate::node::{self, Node};
use crate::reactive::signal::Signal;
use crate::tree;
use crate::views::ViewConfig;
use crate::views::{View, ViewId};
use crate::{Owner, effect};

pub struct Group {
    children: Vec<Box<dyn View>>,
    each: Option<Box<dyn FnOnce(ViewId, Vec<ViewId>)>>,
    when: Vec<(Signal<bool>, Rc<dyn Fn() -> Box<dyn View>>)>,
    scroll: bool,
    clip: bool,
}

impl Group {
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn scroll(mut self) -> Self {
        self.scroll = true;
        self
    }

    pub fn clip(mut self) -> Self {
        self.clip = true;
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

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        vec![DrawCommand::Rect(RectDraw {
            x,
            y,
            w,
            h,
            color: [1.0, 0.0, 0.0, 0.3],
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        })]
    }

    fn build(self: Box<Self>) -> ViewId {
        let each = self.each;
        let when = self.when;
        let scroll = self.scroll;
        let clip = self.clip;
        let child_ids: Vec<ViewId> = self.children.into_iter().map(|c| c.build()).collect();

        let id = tree::add_node(Node {
            name: Some("Group (Primitive)"),
            view: Box::new(Group {
                children: Vec::new(),
                each: None,
                when: Vec::new(),
                scroll: false,
                clip: false,
            }),
            taffy_id: node::placeholder_taffy_id(),
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            layout: LayoutProps::default(),
            handlers: Vec::new(),
            owners: Vec::new(),
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            scroll_x: 0.0,
            scroll_y: 0.0,
            scrollable: false,
            clip: false,
        });

        if scroll {
            tree::set_scrollable(id);
            tree::set_clip(id);
            let scroll_x = crate::state(0.0f32);
            let scroll_y = crate::state(0.0f32);
            effect(move || {
                tree::set_scroll(id, scroll_x.get(), scroll_y.get());
            });
            tree::add_handler(id, move |e: &MouseScroll| {
                scroll_x.update(|v| (v - e.x).max(0.0));
                scroll_y.update(|v| (v - e.y).max(0.0));
            });
        }
        if clip && !scroll {
            tree::set_clip(id);
        }

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

impl ViewConfig<Group> {
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.inner.children.push(Box::new(child));
        self
    }

    pub fn scroll(mut self) -> Self {
        self.inner.scroll = true;
        self
    }

    pub fn clip(mut self) -> Self {
        self.inner.clip = true;
        self
    }

    pub fn when<V: View + 'static>(
        mut self,
        condition: Signal<bool>,
        view_fn: impl Fn() -> V + 'static,
    ) -> Self {
        self.inner
            .when
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
        self.inner = self.inner.each(items, key_fn, view_fn);
        self
    }
}

pub fn group() -> Group {
    Group {
        children: Vec::new(),
        each: None,
        when: Vec::new(),
        scroll: false,
        clip: false,
    }
}
