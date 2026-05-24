use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use bento_shared::CosmicTextMeasurer;
use bento_wgpu::{DrawCommand, DrawList};

use crate::acc::Accumulated;
use crate::events::types::{
    Click, KeyPress, KeyRelease, MouseDown, MouseEnter, MouseLeave, MouseMove, MouseScroll, MouseUp,
};
use crate::input::InputState;
use crate::input::mouse::MouseButton;
use crate::ui::asyncs::AsyncEventQueue;
use crate::widget::{AnyWidget, Canvas, Widget, WidgetHandle};
use crate::{Group, Key, Layout};

pub struct Node {
    pub widget: Box<dyn AnyWidget>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
}

pub struct Ui {
    pub input: InputState,
    pub asyncs: AsyncEventQueue,
    pub nodes: Vec<Option<Node>>,
    pub roots: Vec<usize>,
    pub needs_redraw: bool,
    pub measurer: CosmicTextMeasurer,
    pub dirty: HashSet<usize>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),
            nodes: Vec::new(),
            roots: Vec::new(),
            needs_redraw: false,
            measurer: CosmicTextMeasurer::new(),
            dirty: HashSet::new(),
        }
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        let index = self.nodes.len();
        self.nodes.push(None);
        self.nodes[index] = Some(Node {
            widget: Box::new(widget),
            children: Vec::new(),
            parent: None,
        });
        self.dirty.insert(index);
        self.roots.push(index);
        self.request_redraw();
        WidgetHandle::from_id(index)
    }

    pub fn remove<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        self.remove_id(handle.id);
        self.request_redraw();
    }

    fn remove_id(&mut self, id: usize) {
        let children = self
            .nodes
            .get(id)
            .and_then(|s| s.as_ref())
            .map(|s| s.children.clone())
            .unwrap_or_default();
        for child_id in children {
            self.remove_id(child_id);
        }
        self.roots.retain(|&r| r != id);
        if let Some(node) = self.nodes.get_mut(id) {
            *node = None;
        }
    }

    pub fn node_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.get_mut(id)?.as_mut()
    }

    pub fn get<W: Widget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        let id = handle.id;
        self.nodes
            .get(id)?
            .as_ref()?
            .widget
            .as_any()
            .downcast_ref::<W>()
    }

    pub fn get_mut<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        let id = handle.id;
        self.dirty.insert(id);
        self.nodes
            .get_mut(id)?
            .as_mut()?
            .widget
            .as_any_mut()
            .downcast_mut::<W>()
    }

    pub fn append<W: Widget + 'static, C: Widget + 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        child: WidgetHandle<C>,
    ) {
        if handle.id == child.id {
            println!("[ERROR] Cannot append widget to itself");
            return;
        }
        if let Some(Some(parent_node)) = self.nodes.get(handle.id) {
            if parent_node.children.contains(&child.id) {
                println!("[ERROR] Cannot append, widget is already child of parent");
                return;
            }
        }
        if let Some(Some(parent_node)) = self.nodes.get_mut(handle.id) {
            parent_node.children.push(child.id);
        }
        if let Some(Some(child_node)) = self.nodes.get_mut(child.id) {
            child_node.parent = Some(handle.id);
        }
        self.roots.retain(|&r| r != child.id);
    }

    pub fn update(&mut self) {
        // pass 1: measure
        let dirty: Vec<usize> = self.dirty.drain().collect();
        let mut layout_dirty: HashSet<usize> = HashSet::new();
        let mut width_changed: HashSet<usize> = HashSet::new();
        let mut height_changed: HashSet<usize> = HashSet::new();
        for id in dirty {
            let Some(Some(node)) = self.nodes.get_mut(id) else {
                continue;
            };
            let old_size = node.widget.size();
            node.widget.update(&mut self.measurer);
            let new_size = node.widget.size();
            self.request_redraw();
            if old_size.0 != new_size.0 {
                width_changed.insert(id);
            }
            if old_size.1 != new_size.1 {
                height_changed.insert(id);
            }
            if old_size.0 != new_size.0 || old_size.1 != new_size.1 {
                let mut current = self.nodes[id].as_ref().unwrap().parent;
                while let Some(parent_id) = current {
                    layout_dirty.insert(parent_id);
                    current = self.nodes[parent_id].as_ref().and_then(|n| n.parent);
                }
            }
        }

        // layout
        let t = std::time::Instant::now();
        let mut layout_dirty: Vec<usize> = layout_dirty.into_iter().collect();
        layout_dirty.sort_by(|a, b| b.cmp(a));
        for id in layout_dirty {
            self.layout_node(id, &mut width_changed, &mut height_changed);
        }
        println!("layout time: {:?}", t.elapsed());

        // pass 2: sync
        let dirty: Vec<usize> = self.dirty.drain().collect();
        for id in dirty {
            if let Some(Some(node)) = self.nodes.get_mut(id) {
                node.widget.update(&mut self.measurer);
                self.request_redraw();
            }
        }
    }

    fn layout_node(
        &mut self,
        id: usize,
        width_changed: &mut HashSet<usize>,
        height_changed: &mut HashSet<usize>,
    ) {
        let children = match self.nodes[id].as_ref() {
            Some(n) => n.children.clone(),
            None => return,
        };

        let layout_info = match self.nodes[id].as_ref() {
            Some(n) => n
                .widget
                .as_any()
                .downcast_ref::<Group>()
                .map(|g| g.layout.clone()),
            None => return,
        };

        match layout_info {
            None => {
                for child_id in children {
                    self.layout_node(child_id, width_changed, height_changed);
                }
            }
            Some((Layout::None)) => {
                for child_id in children {
                    self.layout_node(child_id, width_changed, height_changed);
                }
            }
            Some((Layout::Row { gap })) => {
                let first_changed = children
                    .iter()
                    .position(|cid| width_changed.contains(cid))
                    .unwrap_or(children.len());

                let mut cursor = 0.0;
                for child_id in &children[..first_changed] {
                    if let Some(n) = self.nodes[*child_id].as_ref() {
                        let (w, _) = n.widget.size();
                        cursor += w + gap;
                    }
                }

                for child_id in &children[first_changed..] {
                    let (w, _) = match self.nodes[*child_id].as_ref() {
                        Some(n) => n.widget.size(),
                        None => continue,
                    };
                    if let Some(n) = self.nodes[*child_id].as_mut() {
                        n.widget.set_position(cursor, 0.0);
                        self.dirty.insert(*child_id);
                    }
                    cursor += w + gap;
                    self.layout_node(*child_id, width_changed, height_changed);
                }

                let mut total_w = 0.0f32;
                let mut total_h = 0.0f32;
                for child_id in &children {
                    if let Some(n) = self.nodes[*child_id].as_ref() {
                        let (cw, ch) = n.widget.size();
                        total_w += cw + gap;
                        total_h = total_h.max(ch);
                    }
                }
                if let Some(n) = self.nodes[id].as_mut() {
                    if let Some(g) = n.widget.as_any_mut().downcast_mut::<Group>() {
                        if g.w != total_w {
                            width_changed.insert(id);
                        }
                        if g.h != total_h {
                            height_changed.insert(id);
                        }
                        g.w = total_w;
                        g.h = total_h;
                    }
                }
            }
            Some((Layout::Column { gap })) => {
                let first_changed = children
                    .iter()
                    .position(|cid| height_changed.contains(cid))
                    .unwrap_or(children.len());

                let mut cursor = 0.0;
                for child_id in &children[..first_changed] {
                    if let Some(n) = self.nodes[*child_id].as_ref() {
                        let (_, h) = n.widget.size();
                        cursor += h + gap;
                    }
                }

                for child_id in &children[first_changed..] {
                    let (_, h) = match self.nodes[*child_id].as_ref() {
                        Some(n) => n.widget.size(),
                        None => continue,
                    };
                    if let Some(n) = self.nodes[*child_id].as_mut() {
                        n.widget.set_position(0.0, cursor);
                        self.dirty.insert(*child_id);
                    }
                    cursor += h + gap;
                    self.layout_node(*child_id, width_changed, height_changed);
                }

                let mut total_w = 0.0f32;
                let mut total_h = 0.0f32;
                for child_id in &children {
                    if let Some(n) = self.nodes[*child_id].as_ref() {
                        let (cw, ch) = n.widget.size();
                        total_w = total_w.max(cw);
                        total_h += ch + gap;
                    }
                }
                if let Some(n) = self.nodes[id].as_mut() {
                    if let Some(g) = n.widget.as_any_mut().downcast_mut::<Group>() {
                        if g.w != total_w {
                            width_changed.insert(id);
                        }
                        if g.h != total_h {
                            height_changed.insert(id);
                        }
                        g.w = total_w;
                        g.h = total_h;
                    }
                }
            }
        }
    }

    pub fn collect_draw_list(&self) -> DrawList {
        let mut draw_list = DrawList::new();
        for &id in &self.roots {
            self.render_node(id, &mut draw_list, Accumulated::identity());
        }
        draw_list.sort_by_z();
        draw_list
    }

    fn render_node(&self, id: usize, draw_list: &mut DrawList, acc: Accumulated) {
        if let Some(Some(s)) = self.nodes.get(id) {
            let (ox, oy) = s
                .widget
                .as_any()
                .downcast_ref::<Group>()
                .map(|g| (g.x + g.scroll_x, g.y + g.scroll_y))
                .unwrap_or_else(|| s.widget.position());
            let my_acc = acc.push(ox, oy, None, s.widget.z());
            let mut canvas = Canvas {
                draw_list,
                x: my_acc.offset_x,
                y: my_acc.offset_y,
                z: my_acc.z,
                opacity: my_acc.opacity,
                clip: my_acc.clip,
                rotate: my_acc.rotate,
                scale_x: my_acc.scale_x,
                scale_y: my_acc.scale_y,
            };
            s.widget.render(&mut canvas);
            for &child_id in &s.children {
                self.render_node(child_id, draw_list, my_acc);
            }
        }
    }
}

impl Ui {
    pub fn process_input(&mut self) {
        self.keyboard_stuff();
    }

    pub fn keyboard_stuff(&mut self) {
        for (k, _) in self.input.keyboard.just_pressed() {
            if *k == Key::D {
                self.print_nodes();
            }
        }
    }
}

impl Ui {
    pub fn print_nodes(&self) {
        println!("\n[Ui]");
        if self.roots.is_empty() {
            println!("  empty");
            return;
        }
        for &id in &self.roots {
            self.print_node(id, 0);
        }
        println!("\n");
    }

    fn print_node(&self, index: usize, depth: usize) {
        let indent = "  ".repeat(depth);
        if let Some(Some(s)) = self.nodes.get(index) {
            println!(
                "{}[{}] {} {:?}",
                indent,
                index,
                s.widget.name(),
                s.widget.size()
            );
            for &child_id in &s.children {
                self.print_node(child_id, depth + 1);
            }
        }
    }
}
