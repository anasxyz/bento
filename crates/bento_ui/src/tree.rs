use bento_wgpu::{DrawCommand, DrawList, TextMeasurer};
use slab::Slab;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use taffy::{AvailableSpace, LengthPercentageAuto, Rect, TaffyTree};

use crate::{
    layout::LayoutProps,
    node::{EventHandler, Node},
    reactive::{owner::Owner, runtime},
    ui,
    views::ViewId,
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

pub(crate) fn add_node(node: Node) -> ViewId {
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
    ui::request_layout();
}

pub fn set_scroll(id: ViewId, sx: f32, sy: f32) {
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        node.scroll_x = sx;
        node.scroll_y = sy;
        node.paint_dirty = true;
    });
    ui::request_redraw();
}

pub fn set_scrollable(id: ViewId) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].scrollable = true;
    });
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
    ui::request_layout();
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
    ui::request_layout();
}

pub fn store_owner(id: ViewId, owner: Owner) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].owners.push(owner);
    });
}

pub fn render(id: ViewId, draw_list: &mut DrawList, ox: f32, oy: f32, clip: Option<[f32; 4]>) {
    let (paint_dirty, x, y, w, h, scroll_x, scroll_y, scrollable, children) = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (
            node.paint_dirty,
            node.x,
            node.y,
            node.w,
            node.h,
            node.scroll_x,
            node.scroll_y,
            node.scrollable,
            node.children.clone(),
        )
    });

    let rx = x + ox;
    let ry = y + oy;

    if paint_dirty || ox != 0.0 || oy != 0.0 {
        let sub_id = TREE.with(|t| t.borrow().nodes[id.0].paint_subscriber);

        let mut commands = if let Some(sub_id) = sub_id {
            runtime::push_observer(sub_id);
            let cmds = TREE.with(|t| t.borrow().nodes[id.0].view.render(rx, ry, w, h));
            runtime::pop_observer();
            cmds
        } else {
            TREE.with(|t| t.borrow().nodes[id.0].view.render(rx, ry, w, h))
        };

        apply_clip(&mut commands, clip);

        for cmd in &commands {
            draw_list.push_command(cmd.clone());
        }

        if paint_dirty {
            TREE.with(|t| {
                let mut t = t.borrow_mut();
                let node = &mut t.nodes[id.0];
                node.cache = commands;
                node.paint_dirty = false;
            });
        }
    } else {
        let mut cache = TREE.with(|t| t.borrow().nodes[id.0].cache.clone());
        apply_clip(&mut cache, clip);
        for cmd in cache {
            draw_list.push_command(cmd);
        }
    }

    let clip_self = TREE.with(|t| t.borrow().nodes[id.0].clip);

    let (child_ox, child_oy, child_clip) = if scrollable {
        let c = if clip_self { merge_clip(clip, Some([rx, ry, w, h])) } else { clip };
        (ox - scroll_x, oy - scroll_y, c)
    } else if clip_self {
        let c = merge_clip(clip, Some([rx, ry, w, h]));
        (ox, oy, c)
    } else {
        (ox, oy, clip)
    };

    for child_id in children {
        render(child_id, draw_list, child_ox, child_oy, child_clip);
    }
}

pub fn set_clip(id: ViewId) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].clip = true;
    });
}

fn apply_clip(commands: &mut Vec<DrawCommand>, clip: Option<[f32; 4]>) {
    if clip.is_none() {
        return;
    }
    for cmd in commands {
        match cmd {
            DrawCommand::Rect(r) => r.clip = merge_clip(r.clip, clip),
            DrawCommand::Text(t) => t.clip = merge_clip(t.clip, clip),
            DrawCommand::Image(i) => i.clip = merge_clip(i.clip, clip),
        }
    }
}

fn merge_clip(a: Option<[f32; 4]>, b: Option<[f32; 4]>) -> Option<[f32; 4]> {
    match (a, b) {
        (Some(a), Some(b)) => Some(intersect_clip(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn intersect_clip(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let x = a[0].max(b[0]);
    let y = a[1].max(b[1]);
    let x2 = (a[0] + a[2]).min(b[0] + b[2]);
    let y2 = (a[1] + a[3]).min(b[1] + b[3]);
    [x, y, (x2 - x).max(0.0), (y2 - y).max(0.0)]
}

thread_local! {
    static MEASURER: RefCell<Option<TextMeasurer>> = RefCell::new(None);
}

pub fn layout(root: ViewId, available_w: f32, available_h: f32, measurer: &mut TextMeasurer) {
    let root_taffy = TREE.with(|t| t.borrow().nodes[root.0].taffy_id);

    // precollect everything needed
    // no TREE access inside closure
    let lookup: HashMap<taffy::NodeId, (f32, f32)> = TREE.with(|t| {
        let t = t.borrow();
        t.nodes
            .iter()
            .map(|(_, n)| {
                let size = if n.paint_dirty {
                    n.view.measure(measurer)
                } else {
                    (n.w, n.h) // use cached size
                };
                (n.taffy_id, size)
            })
            .collect()
    });

    let mut taffy = TREE.with(|t| std::mem::replace(&mut t.borrow_mut().taffy, TaffyTree::new()));

    taffy
        .compute_layout_with_measure(
            root_taffy,
            taffy::Size {
                width: AvailableSpace::Definite(available_w),
                height: AvailableSpace::Definite(available_h),
            },
            |known_dimensions, _, node_id, _, _| {
                let (mw, mh) = lookup.get(&node_id).copied().unwrap_or((0.0, 0.0));
                taffy::Size {
                    width: known_dimensions.width.unwrap_or(mw),
                    height: known_dimensions.height.unwrap_or(mh),
                }
            },
        )
        .unwrap();

    TREE.with(|t| {
        t.borrow_mut().taffy = taffy;
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
    ui::request_layout();
}

pub fn update_inset(
    id: ViewId,
    left: Option<LengthPercentageAuto>,
    right: Option<LengthPercentageAuto>,
    top: Option<LengthPercentageAuto>,
    bottom: Option<LengthPercentageAuto>,
) {
    let taffy_id = TREE.with(|t| t.borrow().nodes[id.0].taffy_id);
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        let mut style = node.layout.to_taffy_style();
        if let Some(v) = left {
            style.inset.left = v;
        }
        if let Some(v) = right {
            style.inset.right = v;
        }
        if let Some(v) = top {
            style.inset.top = v;
        }
        if let Some(v) = bottom {
            style.inset.bottom = v;
        }
        t.taffy.set_style(taffy_id, style).unwrap();
    });
    ui::request_layout();
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

pub(crate) fn hit_test(id: ViewId, x: f32, y: f32, ox: f32, oy: f32) -> Option<ViewId> {
    let (node_x, node_y, node_w, node_h, scroll_x, scroll_y, scrollable, children) =
        TREE.with(|t| {
            let t = t.borrow();
            let node = &t.nodes[id.0];
            (
                node.x,
                node.y,
                node.w,
                node.h,
                node.scroll_x,
                node.scroll_y,
                node.scrollable,
                node.children.clone(),
            )
        });

    let rx = node_x + ox;
    let ry = node_y + oy;

    let child_ox = if scrollable { ox - scroll_x } else { ox };
    let child_oy = if scrollable { oy - scroll_y } else { oy };

    for child_id in children.iter().rev() {
        if let Some(hit) = hit_test(*child_id, x, y, child_ox, child_oy) {
            return Some(hit);
        }
    }

    if x >= rx && x <= rx + node_w && y >= ry && y <= ry + node_h {
        return Some(id);
    }

    None
}

pub fn set_name(id: ViewId, name: &'static str) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].name = Some(name);
    });
}

pub(crate) fn print_tree(id: ViewId, depth: usize) {
    TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        let display_name = node.name.unwrap_or_else(|| node.view.name());
        let indent = "  ".repeat(depth);
        println!(
            "{}{} (id: {}) x: {} y: {} w: {} h: {}",
            indent, display_name, id.0, node.x, node.y, node.w, node.h
        );
        let children = node.children.clone();
        drop(t);
        for child_id in children {
            print_tree(child_id, depth + 1);
        }
    });
}
