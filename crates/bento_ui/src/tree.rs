use bento_wgpu::{DrawList, TextMeasurer};
use slab::Slab;
use std::{cell::RefCell, rc::Rc};
use taffy::{AvailableSpace, LengthPercentageAuto, Rect, TaffyTree};

use crate::{
    layout::LayoutProps,
    node::{EventHandler, Node},
    reactive::{owner::Owner, runtime},
    ui,
    view::ViewId,
};

struct Tree {
    nodes: Slab<Node>,
    taffy: TaffyTree<()>,
}

impl Tree {
    fn new() -> Self {
        Self {
            nodes: Slab::new(),
            taffy: TaffyTree::new(),
        }
    }
}

thread_local! {
    static TREE: RefCell<Tree> = RefCell::new(Tree::new());
}

pub fn add_node(node: Node) -> ViewId {
    let taffy_id = TREE.with(|t| {
        let mut t = t.borrow_mut();
        let style = node.layout.to_taffy_style();
        t.taffy.new_leaf(style).unwrap()
    });

    let mut node = node;
    node.taffy_id = taffy_id;

    let id = TREE.with(|t| ViewId(t.borrow_mut().nodes.insert(node)));

    let sub_id = runtime::create_subscriber(Rc::new(move || {
        TREE.with(|t| {
            if let Some(node) = t.borrow_mut().nodes.get_mut(id.0) {
                node.paint_dirty = true;
            }
        });
        ui::request_redraw();
    }));

    TREE.with(|t| {
        if let Some(node) = t.borrow_mut().nodes.get_mut(id.0) {
            node.paint_subscriber = Some(sub_id);
        }
    });

    runtime::push_observer(sub_id);
    TREE.with(|t| t.borrow().nodes[id.0].view.render(0.0, 0.0, 0.0, 0.0));
    runtime::pop_observer();

    id
}

pub fn remove_node(id: ViewId) {
    let parent = TREE.with(|t| t.borrow().nodes[id.0].parent);
    remove_node_inner(id);
    if let Some(parent_id) = parent {
        TREE.with(|t| {
            t.borrow_mut().nodes[parent_id.0]
                .children
                .retain(|c| c.0 != id.0);
        });
    }
    ui::request_redraw();
}

fn remove_node_inner(id: ViewId) {
    let (taffy_id, children, owners) = TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        let children = node.children.clone();
        let owners = std::mem::take(&mut node.owners);
        (node.taffy_id, children, owners)
    });

    drop(owners);

    for child_id in children {
        remove_node_inner(child_id);
    }

    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let _ = t.taffy.remove(taffy_id);
        t.nodes.remove(id.0);
    });
}

pub fn append_child(parent: ViewId, child: ViewId) {
    let (parent_taffy, child_taffy) = TREE.with(|t| {
        let t = t.borrow();
        (t.nodes[parent.0].taffy_id, t.nodes[child.0].taffy_id)
    });
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        t.nodes[parent.0].children.push(child);
        t.nodes[child.0].parent = Some(parent);
        t.taffy.add_child(parent_taffy, child_taffy).unwrap();
    });
    ui::request_redraw();
}

pub fn reorder_children(parent: ViewId, order: Vec<ViewId>) {
    let taffy_children: Vec<taffy::NodeId> = TREE.with(|t| {
        let t = t.borrow();
        order.iter().map(|id| t.nodes[id.0].taffy_id).collect()
    });
    let parent_taffy = TREE.with(|t| t.borrow().nodes[parent.0].taffy_id);
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        t.nodes[parent.0].children = order;
        t.taffy.set_children(parent_taffy, &taffy_children).unwrap();
    });
}

pub fn store_owner(id: ViewId, owner: Owner) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].owners.push(owner);
    });
}

pub fn render(id: ViewId, draw_list: &mut DrawList) {
    let (paint_dirty, x, y, w, h, children) = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (
            node.paint_dirty,
            node.x,
            node.y,
            node.w,
            node.h,
            node.children.clone(),
        )
    });

    if paint_dirty {
        let sub_id = TREE.with(|t| t.borrow().nodes[id.0].paint_subscriber);

        let commands = if let Some(sub_id) = sub_id {
            runtime::push_observer(sub_id);
            let cmds = TREE.with(|t| t.borrow().nodes[id.0].view.render(x, y, w, h));
            runtime::pop_observer();
            cmds
        } else {
            TREE.with(|t| t.borrow().nodes[id.0].view.render(x, y, w, h))
        };

        for cmd in &commands {
            draw_list.push_command(cmd.clone());
        }

        TREE.with(|t| {
            let mut t = t.borrow_mut();
            let node = &mut t.nodes[id.0];
            node.cache = commands;
            node.paint_dirty = false;
        });
    } else {
        let cache = TREE.with(|t| t.borrow().nodes[id.0].cache.clone());
        for cmd in cache {
            draw_list.push_command(cmd);
        }
    }

    for child_id in children {
        render(child_id, draw_list);
    }
}

pub fn layout(root: ViewId, available_w: f32, available_h: f32) {
    let root_taffy = TREE.with(|t| t.borrow().nodes[root.0].taffy_id);

    TREE.with(|t| {
        t.borrow_mut()
            .taffy
            .compute_layout(
                root_taffy,
                taffy::Size {
                    width: AvailableSpace::Definite(available_w),
                    height: AvailableSpace::Definite(available_h),
                },
            )
            .unwrap();
    });

    apply_layout(root, 0.0, 0.0);
}

fn apply_layout(id: ViewId, parent_x: f32, parent_y: f32) {
    let (taffy_id, children) = TREE.with(|t| {
        let t = t.borrow();
        (t.nodes[id.0].taffy_id, t.nodes[id.0].children.clone())
    });

    let layout = TREE.with(|t| *t.borrow().taffy.layout(taffy_id).unwrap());

    let x = parent_x + layout.location.x;
    let y = parent_y + layout.location.y;
    let w = layout.size.width;
    let h = layout.size.height;

    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        node.x = x;
        node.y = y;
        node.w = w;
        node.h = h;
        node.paint_dirty = true;
    });

    for child_id in children {
        apply_layout(child_id, x, y);
    }
}

pub fn set_layout(id: ViewId, layout: LayoutProps) {
    let taffy_id = TREE.with(|t| t.borrow().nodes[id.0].taffy_id);
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        t.nodes[id.0].layout = layout.clone();
        t.taffy
            .set_style(taffy_id, layout.to_taffy_style())
            .unwrap();
    });
}

pub fn set_inset(id: ViewId, x: f32, y: f32) {
    let taffy_id = TREE.with(|t| t.borrow().nodes[id.0].taffy_id);
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        node.x = x;
        node.y = y;
        let mut style = node.layout.to_taffy_style();
        style.inset = Rect {
            left: LengthPercentageAuto::length(x),
            top: LengthPercentageAuto::length(y),
            right: LengthPercentageAuto::auto(),
            bottom: LengthPercentageAuto::auto(),
        };
        t.taffy.set_style(taffy_id, style).unwrap();
    });
    ui::request_redraw();
}

pub(crate) fn add_handler<E: 'static>(id: ViewId, f: impl Fn(&E) + 'static) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].handlers.push(EventHandler {
            type_id: std::any::TypeId::of::<E>(),
            handler: Rc::new(move |any| {
                if let Some(event) = any.downcast_ref::<E>() {
                    f(event);
                }
            }),
        });
    });
}

pub fn get_rect(id: ViewId) -> (f32, f32, f32, f32) {
    TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (node.x, node.y, node.w, node.h)
    })
}

pub(crate) fn dispatch<E: 'static>(id: ViewId, event: &E) {
    let type_id = std::any::TypeId::of::<E>();
    let mut current = Some(id);
    while let Some(node_id) = current {
        let handlers: Vec<Rc<dyn Fn(&dyn std::any::Any)>> = TREE.with(|t| {
            let t = t.borrow();
            let node = &t.nodes[node_id.0];
            node.handlers
                .iter()
                .filter(|h| h.type_id == type_id)
                .map(|h| h.handler.clone())
                .collect()
        });

        for handler in handlers {
            handler(event as &dyn std::any::Any);
        }

        current = TREE.with(|t| t.borrow().nodes[node_id.0].parent);
    }
}

pub(crate) fn hit_test(id: ViewId, x: f32, y: f32) -> Option<ViewId> {
    let (node_x, node_y, node_w, node_h, children) = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (node.x, node.y, node.w, node.h, node.children.clone())
    });

    for child_id in children.iter().rev() {
        if let Some(hit) = hit_test(*child_id, x, y) {
            return Some(hit);
        }
    }

    if x >= node_x && x <= node_x + node_w && y >= node_y && y <= node_y + node_h {
        return Some(id);
    }

    None
}

pub(crate) fn print_tree(id: ViewId, depth: usize) {
    TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        let indent = "  ".repeat(depth);
        println!(
            "{}{} (id: {}) x: {} y: {} w: {} h: {}",
            indent,
            node.view.name(),
            id.0,
            node.x,
            node.y,
            node.w,
            node.h
        );
        let children = node.children.clone();
        drop(t);
        for child_id in children {
            print_tree(child_id, depth + 1);
        }
    });
}
