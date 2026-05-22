use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use bento_shared::CosmicTextMeasurer;
use bento_wgpu::DrawList;

use crate::accumulated::Accumulated;
use crate::events::types::{
    Click, KeyPress, KeyRelease, MouseDown, MouseEnter, MouseLeave, MouseMove, MouseScroll, MouseUp,
};
use crate::input::InputState;
use crate::input::mouse::MouseButton;
use crate::ui::asyncs::AsyncEventQueue;
use crate::widget::{AnyWidget, Widget, WidgetHandle, WidgetMut};
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
        widget.set_id(index);
        self.nodes.push(None);
        widget.build(self);
        let children: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().filter(|s| s.parent == Some(index)).map(|_| i))
            .collect();
        self.nodes[index] = Some(Node {
            widget: Box::new(widget),
            children,
            parent: None,
        });
        self.dirty.insert(index);
        self.roots.push(index);
        self.request_redraw();
        WidgetHandle::from_id(index)
    }

    pub fn add_child<P: Widget + 'static, C: Widget + 'static>(
        &mut self,
        parent: &P,
        child: C,
    ) -> WidgetHandle<C> {
        let child_handle = self.add(child);
        let parent_handle = WidgetHandle::<P>::from_id(parent.id());
        self.append(parent_handle, child_handle);
        child_handle
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

    pub fn get<W: Widget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        let id = handle.id;
        self.nodes
            .get(id)?
            .as_ref()?
            .widget
            .as_any()
            .downcast_ref::<W>()
    }

    pub fn get_mut<W: Widget + 'static>(
        &mut self,
        handle: WidgetHandle<W>,
    ) -> Option<WidgetMut<'_, W>> {
        let id = handle.id;
        let widget = self
            .nodes
            .get_mut(id)?
            .as_mut()?
            .widget
            .as_any_mut()
            .downcast_mut::<W>()?;
        Some(WidgetMut {
            widget,
            id,
            dirty: &mut self.dirty,
        })
    }

    pub(crate) fn get_mut_raw<W: Widget + 'static>(
        &mut self,
        handle: WidgetHandle<W>,
    ) -> Option<&mut W> {
        let id = handle.id;
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
        for id in dirty {
            if let Some(mut node) = self.nodes[id].take() {
                node.widget.update(self);
                self.nodes[id] = Some(node);
                self.request_redraw();
            }
        }

        // layout
        let roots = self.roots.clone();
        for id in roots {
            self.layout_node(id);
        }

        // pass 2: sync positions to children
        let dirty: Vec<usize> = self.dirty.drain().collect();
        for id in dirty {
            if let Some(mut node) = self.nodes[id].take() {
                node.widget.update(self);
                self.nodes[id] = Some(node);
                self.request_redraw();
            }
        }
    }

    fn layout_node(&mut self, id: usize) {
        let children = match self.nodes[id].as_ref() {
            Some(n) => n.children.clone(),
            None => return,
        };

        let layout_info = match self.nodes[id].as_ref() {
            Some(n) => n
                .widget
                .as_any()
                .downcast_ref::<Group>()
                .map(|g| (g.layout.clone(), g.x, g.y)),
            None => return,
        };

        match layout_info {
            None => {
                for child_id in children {
                    self.layout_node(child_id);
                }
            }
            Some((Layout::None, _, _)) => {
                for child_id in children {
                    self.layout_node(child_id);
                }
            }
            Some((Layout::Row { gap }, gx, gy)) => {
                let mut cursor = gx;
                for child_id in &children {
                    self.layout_node(*child_id);
                    let (_, _, w, _) = match self.nodes[*child_id].as_ref() {
                        Some(n) => n.widget.hitbox(),
                        None => continue,
                    };
                    if let Some(n) = self.nodes[*child_id].as_mut() {
                        n.widget.set_position(cursor, gy);
                        if n.widget.is_dirty() {
                            self.dirty.insert(*child_id);
                        }
                    }
                    cursor += w + gap;
                }

                let mut total_w = 0.0f32;
                let mut total_h = 0.0f32;
                for child_id in &children {
                    if let Some(n) = self.nodes[*child_id].as_ref() {
                        let (_, _, cw, ch) = n.widget.hitbox();
                        total_w += cw + gap;
                        total_h = total_h.max(ch);
                    }
                }
                if let Some(n) = self.nodes[id].as_mut() {
                    if let Some(g) = n.widget.as_any_mut().downcast_mut::<Group>() {
                        g.w = total_w;
                        g.h = total_h;
                    }
                }
            }
            Some((Layout::Column { gap }, gx, gy)) => {
                let mut cursor = gy;
                for child_id in &children {
                    self.layout_node(*child_id);
                    let (_, _, _, h) = match self.nodes[*child_id].as_ref() {
                        Some(n) => n.widget.hitbox(),
                        None => continue,
                    };
                    if let Some(n) = self.nodes[*child_id].as_mut() {
                        n.widget.set_position(gx, cursor);
                        if n.widget.is_dirty() {
                            self.dirty.insert(*child_id);
                        }
                    }
                    cursor += h + gap;
                }

                let mut total_w = 0.0f32;
                let mut total_h = 0.0f32;
                for child_id in &children {
                    if let Some(n) = self.nodes[*child_id].as_ref() {
                        let (_, _, cw, ch) = n.widget.hitbox();
                        total_w = total_w.max(cw);
                        total_h += ch + gap;
                    }
                }
                if let Some(n) = self.nodes[id].as_mut() {
                    if let Some(g) = n.widget.as_any_mut().downcast_mut::<Group>() {
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
        draw_list
    }

    fn render_node(&self, id: usize, draw_list: &mut DrawList, acc: Accumulated) {
        if let Some(Some(s)) = self.nodes.get(id) {
            s.widget.render(draw_list, &acc);
            let (ox, oy) = s.widget.render_offset();
            let child_acc = acc.push(ox, oy, None);
            for &child_id in &s.children {
                self.render_node(child_id, draw_list, child_acc);
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
                s.widget.hitbox()
            );
            for &child_id in &s.children {
                self.print_node(child_id, depth + 1);
            }
        }
    }
}
