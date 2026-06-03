use crate::{
    Group, layout::{CrossAxis, Direction, MainAxis, Size}, node::Node, reactive::{effect, owner::Owner, signal::Signal}, tree, view::{View, ViewId}
};
use bento_wgpu::{DrawCommand, DrawList, TextMeasurer};
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

struct EachNode<T: Clone + 'static> {
    items: Signal<Vec<T>>,
}

impl<T: Clone + 'static> View for EachNode<T> {
    fn build(self: Box<Self>) -> ViewId {
        unreachable!()
    }
    fn render(&self, _x: f32, _y: f32, _w: f32, _h: f32) -> Vec<DrawCommand> {
        // subscribe render observer to items
        let _ = self.items.get();

        Vec::new()
    }
    fn measure(&self, _measurer: &mut TextMeasurer) -> (f32, f32) {
        (0.0, 0.0)
    }
}

pub struct Each<T: Clone + 'static, K: Eq + Hash + Clone + 'static> {
    items: Signal<Vec<T>>,
    key_fn: Box<dyn Fn(&T) -> K>,
    view_fn: Rc<dyn Fn(T) -> Box<dyn View>>,
}

impl<T, K> View for Each<T, K>
where
    T: Clone + 'static,
    K: Eq + Hash + Clone + 'static,
{
    fn build(self: Box<Self>) -> ViewId {
        let items = self.items;
        let key_fn = self.key_fn;
        let view_fn = self.view_fn;

        let owner = Owner::new();

        let parent_id = tree::add_node(Node {
            view: Box::new(EachNode { items }), // items copied into node
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            handlers: Vec::new(),
            owner: None,
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            layout_dirty: true,
            width: Size::Auto,
            height: Size::Auto,
        });

        let nodes: Rc<RefCell<HashMap<K, ViewId>>> = Rc::new(RefCell::new(HashMap::new()));
        let nodes_clone = nodes.clone();

        // items is Copy so still available here
        effect(move || {
            let new_items = items.get();
            let mut current = nodes_clone.borrow_mut();

            // find removed
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

            // find added
            for item in &new_items {
                let key = key_fn(item);
                if !current.contains_key(&key) {
                    let item: T = item.clone();
                    let id = (view_fn)(item).build();
                    tree::append_child(parent_id, id);
                    current.insert(key, id);
                }
            }

            // reorder to match new order
            let order = new_items
                .iter()
                .map(|item| current[&key_fn(item)])
                .collect();
            tree::reorder_children(parent_id, order);
        });

        let owner = owner.collect();
        tree::store_owner(parent_id, owner);

        parent_id
    }

    fn render(&self, _x: f32, _y: f32, _w: f32, _h: f32) -> Vec<DrawCommand> {
        Vec::new()
    }

    fn measure(&self, _measurer: &mut TextMeasurer) -> (f32, f32) {
        (0.0, 0.0) // size comes from children via layout
    }
}

pub fn each<T, K, V, VF>(
    items: Signal<Vec<T>>,
    key_fn: impl Fn(&T) -> K + 'static,
    view_fn: VF,
) -> Each<T, K>
where
    T: Clone + 'static,
    K: Eq + std::hash::Hash + Clone + 'static,
    VF: Fn(T) -> V + 'static,
    V: View + 'static,
{
    Each {
        items,
        key_fn: Box::new(key_fn),
        view_fn: Rc::new(move |item| Box::new(view_fn(item)) as Box<dyn View>),
    }
}
