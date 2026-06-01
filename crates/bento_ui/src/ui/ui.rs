use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use bento_wgpu::{DrawCommand, DrawList};
use bento_wgpu::{RectDraw, TextMeasurer};

use crate::acc::Accumulated;
use crate::events::types::{
    Click, KeyPress, KeyRelease, MouseDown, MouseEnter, MouseLeave, MouseMove, MouseScroll, MouseUp,
};
use crate::input::InputState;
use crate::input::mouse::MouseButton;
use crate::layout::{Layout, Size};
use crate::types::CursorIcon;
use crate::ui::asyncs::AsyncEventQueue;
use crate::widget::{AnyWidget, Canvas, Widget, WidgetHandle};
use crate::{FocusGained, FocusLost, Group, HoverEnter, HoverLeave, Key};

pub struct Node {
    pub widget: Box<dyn AnyWidget>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub build_fn: fn(&mut Ui, usize),
    pub update_fn: fn(&mut Ui, usize),
}

pub struct Ui {
    pub input: InputState,
    pub asyncs: AsyncEventQueue,
    pub nodes: Vec<Option<Node>>,
    pub roots: Vec<usize>,
    pub needs_redraw: bool,
    pub measurer: TextMeasurer,
    pub dirty: HashSet<usize>,
    pub layout_dirty: HashSet<usize>,
    pub viewport_w: f32,
    pub viewport_h: f32,

    listeners: Vec<Listener>,
    next_listener_id: u64,

    pub debug: bool,
    pub hovered_node: Option<usize>,
    pub hovered_rect: Option<[f32; 4]>,

    pub captured_widget: Option<usize>,

    pub focused: Option<usize>,

    pub cursor: CursorIcon,

    pub state_map: HashMap<TypeId, Box<dyn Any>>,
}

impl Ui {
    pub fn new() -> Self {
        let mut ui = Self {
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),
            nodes: Vec::new(),
            roots: Vec::new(),
            needs_redraw: false,
            measurer: TextMeasurer::new(),
            dirty: HashSet::new(),
            layout_dirty: HashSet::new(),
            viewport_w: 800.0,
            viewport_h: 600.0,

            listeners: Vec::new(),
            next_listener_id: 0,

            debug: false,
            hovered_node: None,
            hovered_rect: None,

            captured_widget: None,

            focused: None,

            cursor: CursorIcon::Default,

            state_map: HashMap::new(),
        };

        ui
    }

    pub fn set_state<T: 'static>(&mut self, state: T) {
        self.state_map.insert(TypeId::of::<T>(), Box::new(state));
    }

    pub fn state<T: 'static>(&self) -> &T {
        self.state_map
            .get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref::<T>()
            .unwrap()
    }

    pub fn state_mut<T: 'static>(&mut self) -> &mut T {
        self.state_map
            .get_mut(&TypeId::of::<T>())
            .unwrap()
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn with_state<T: 'static>(&mut self, f: impl FnOnce(&mut T, &mut Ui)) {
        let mut state = self.state_map.remove(&TypeId::of::<T>()).unwrap();
        let s = state.downcast_mut::<T>().unwrap();
        f(s, self);
        self.state_map.insert(TypeId::of::<T>(), state);
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn request_update<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        self.dirty.insert(handle.id);
    }

    pub fn request_layout<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        self.layout_dirty.insert(handle.id);
        let mut current = self.nodes[handle.id].as_ref().and_then(|n| n.parent);
        while let Some(parent_id) = current {
            self.layout_dirty.insert(parent_id);
            current = self.nodes[parent_id].as_ref().and_then(|n| n.parent);
        }
    }

    pub fn set<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>, f: impl FnOnce(&mut W)) {
        if let Some(w) = self.get_mut(handle) {
            f(w);
            self.request_update(handle);
            self.request_layout(handle);
            self.request_redraw();
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        let index = self.nodes.len();
        widget.init();
        self.nodes.push(Some(Node {
            widget: Box::new(widget),
            children: Vec::new(),
            parent: None,
            build_fn: |ui, id| W::build(ui, WidgetHandle::<()>::from_id(id)),
            update_fn: |ui, id| W::update(ui, WidgetHandle::<()>::from_id(id)),
        }));
        self.dirty.insert(index);
        self.layout_dirty.insert(index);
        self.roots.push(index);
        self.request_redraw();
        let build_fn = self.nodes[index].as_ref().unwrap().build_fn;
        build_fn(self, index);
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
        self.nodes
            .get(handle.id)?
            .as_ref()?
            .widget
            .as_any()
            .downcast_ref::<W>()
    }

    pub fn get_mut<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        self.nodes
            .get_mut(handle.id)?
            .as_mut()?
            .widget
            .as_any_mut()
            .downcast_mut::<W>()
    }

    pub fn attach<W: Widget + 'static, C: Widget + 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        child: WidgetHandle<C>,
    ) {
        if handle.id == child.id {
            println!("[ERROR] Cannot attach widget to itself");
            return;
        }
        if let Some(Some(parent_node)) = self.nodes.get(handle.id) {
            if parent_node.children.contains(&child.id) {
                println!("[ERROR] Cannot attach, widget is already child of parent");
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
        self.layout_dirty.insert(handle.id);
    }

    pub fn update(&mut self) {
        // update / measure pass
        let t = web_time::Instant::now();
        let dirty: Vec<usize> = self.dirty.drain().collect();
        // println!("[update] dirty count: {}", dirty.len());
        for id in dirty {
            let Some(Some(node)) = self.nodes.get(id) else {
                continue;
            };
            let old_size = node.widget.size();
            let update_fn = node.update_fn;
            update_fn(self, id);
            let new_size = self.nodes[id].as_ref().unwrap().widget.size();
            self.request_redraw();
            if old_size != new_size {
                let mut current = self.nodes[id].as_ref().unwrap().parent;
                while let Some(parent_id) = current {
                    self.layout_dirty.insert(parent_id);
                    current = self.nodes[parent_id].as_ref().and_then(|n| n.parent);
                }
            }
        }
        // println!("[update] measure time: {:?} +", t.elapsed());

        // layout pass
        let t = web_time::Instant::now();
        while !self.layout_dirty.is_empty() {
            let layout_ids: Vec<usize> = self.layout_dirty.drain().collect();
            let mut layout_ids: Vec<usize> = layout_ids.into_iter().collect();
            layout_ids.sort_by(|a, b| b.cmp(a));
            for id in layout_ids {
                self.layout_node(id, self.viewport_w, self.viewport_h);
            }
        }

        // DEBUG
        // to update hovered node when hovering over a widget and it changes/moves
        // could remove as it doesn't matter to me that much
        if self.debug {
            self.hit_test();
        }

        if !self.dirty.is_empty() {
            self.dirty.clear();
            self.request_redraw();
        }
    }

    fn layout_node(&mut self, id: usize, available_w: f32, available_h: f32) {
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

        let width_sizing = self.nodes[id]
            .as_ref()
            .unwrap()
            .widget
            .width_sizing()
            .clone();
        let height_sizing = self.nodes[id]
            .as_ref()
            .unwrap()
            .widget
            .height_sizing()
            .clone();

        let inner_w = match &width_sizing {
            Size::Auto => available_w,
            s => s.resolve(available_w),
        };
        let inner_h = match &height_sizing {
            Size::Auto => available_h,
            s => s.resolve(available_h),
        };

        // set group's own computed size if not auto
        if let Some(Some(node)) = self.nodes.get_mut(id) {
            if let Some(g) = node.widget.as_any_mut().downcast_mut::<Group>() {
                if !matches!(g.width, Size::Auto) {
                    g.w = inner_w;
                }
                if !matches!(g.height, Size::Auto) {
                    g.h = inner_h;
                }
            }
        }

        // resolve and set size for non-auto, non-fill children
        // skip children that have any auto dimension — layout arms handle those
        for child_id in &children {
            if let Some(Some(node)) = self.nodes.get(*child_id) {
                let ws = node.widget.width_sizing().clone();
                let hs = node.widget.height_sizing().clone();
                let needs_w = !ws.is_auto() && !matches!(ws, Size::Fill);
                let needs_h = !hs.is_auto() && !matches!(hs, Size::Fill);
                if needs_w && needs_h {
                    let new_w = ws.resolve(inner_w);
                    let new_h = hs.resolve(inner_h);
                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                        node.widget.set_size(new_w, new_h);
                    }
                } else if needs_w && !hs.is_auto() {
                    let cur = node.widget.size();
                    let new_w = ws.resolve(inner_w);
                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                        node.widget.set_size(new_w, cur.1);
                    }
                } else if needs_h && !ws.is_auto() {
                    let cur = node.widget.size();
                    let new_h = hs.resolve(inner_h);
                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                        node.widget.set_size(cur.0, new_h);
                    }
                }
            }
        }

        match layout_info {
            None | Some(Layout::None) => {
                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        let ws = node.widget.width_sizing().clone();
                        let hs = node.widget.height_sizing().clone();
                        if matches!(ws, Size::Fill) || matches!(hs, Size::Fill) {
                            let cur = node.widget.size();
                            let new_w = if matches!(ws, Size::Fill) {
                                inner_w
                            } else {
                                cur.0
                            };
                            let new_h = if matches!(hs, Size::Fill) {
                                inner_h
                            } else {
                                cur.1
                            };
                            if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                node.widget.set_size(new_w, new_h);
                            }
                        }
                    }
                }
                for child_id in children {
                    self.layout_node(child_id, inner_w, inner_h);
                }
            }
            Some(Layout::Row { gap }) => {
                // pass 0: recurse auto-width children first to get their real sizes
                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        if matches!(node.widget.width_sizing(), Size::Auto) {
                            self.layout_node(*child_id, inner_w, inner_h);
                        }
                    }
                }

                let is_auto_w = matches!(width_sizing, Size::Auto);
                let mut fixed_total = 0.0f32;
                let mut fill_count = 0;
                let child_count = children.len();
                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        match node.widget.width_sizing() {
                            Size::Fill => fill_count += 1,
                            _ => fixed_total += node.widget.size().0,
                        }
                    }
                }
                let total_gap = gap * (child_count.saturating_sub(1)) as f32;
                let content_w = if is_auto_w {
                    fixed_total + total_gap
                } else {
                    inner_w
                };
                let remaining = (content_w - fixed_total - total_gap).max(0.0);
                let fill_w = if fill_count > 0 {
                    remaining / fill_count as f32
                } else {
                    0.0
                };

                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        let set_w = matches!(node.widget.width_sizing(), Size::Fill);
                        let set_h = matches!(node.widget.height_sizing(), Size::Fill);
                        if set_w || set_h {
                            let cur = node.widget.size();
                            let new_w = if set_w { fill_w } else { cur.0 };
                            let new_h = if set_h { inner_h } else { cur.1 };
                            if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                node.widget.set_size(new_w, new_h);
                            }
                        }
                    }
                }

                let mut cursor = 0.0;
                for child_id in &children {
                    let (w, _) = match self.nodes[*child_id].as_ref() {
                        Some(n) => n.widget.size(),
                        None => continue,
                    };
                    if let Some(n) = self.nodes[*child_id].as_mut() {
                        n.widget.set_position(cursor, 0.0);
                        self.request_redraw();
                    }
                    cursor += w + gap;
                    let child_w = self.nodes[*child_id]
                        .as_ref()
                        .map(|n| n.widget.size().0)
                        .unwrap_or(content_w);
                    let child_h = self.nodes[*child_id]
                        .as_ref()
                        .map(|n| n.widget.size().1)
                        .unwrap_or(inner_h);
                    self.layout_node(*child_id, child_w, child_h);
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
                        let final_w = if matches!(g.width, Size::Auto) {
                            total_w
                        } else {
                            g.w
                        };
                        let final_h = if matches!(g.height, Size::Auto) {
                            total_h
                        } else {
                            g.h
                        };
                        let size_changed = g.w != final_w || g.h != final_h;
                        g.w = final_w;
                        g.h = final_h;
                        if size_changed {
                            if let Some(parent_id) = self.nodes[id].as_ref().and_then(|n| n.parent)
                            {
                                self.layout_dirty.insert(parent_id);
                            }
                        }
                    }
                }
            }
            Some(Layout::Column { gap }) => {
                let is_auto_h = matches!(height_sizing, Size::Auto);

                // pass 0: recurse auto-height children first to get their real sizes
                if is_auto_h {
                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            if matches!(node.widget.height_sizing(), Size::Auto) {
                                self.layout_node(*child_id, inner_w, inner_h);
                            }
                        }
                    }
                }

                let mut fixed_total = 0.0f32;
                let mut fill_count = 0;
                let child_count = children.len();
                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        match node.widget.height_sizing() {
                            Size::Fill => fill_count += 1,
                            _ => fixed_total += node.widget.size().1,
                        }
                    }
                }
                let total_gap = gap * (child_count.saturating_sub(1)) as f32;
                let content_h = if is_auto_h {
                    fixed_total + total_gap
                } else {
                    inner_h
                };
                let remaining = (content_h - fixed_total - total_gap).max(0.0);
                let fill_h = if fill_count > 0 {
                    remaining / fill_count as f32
                } else {
                    0.0
                };

                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        let set_w = matches!(node.widget.width_sizing(), Size::Fill);
                        let set_h = matches!(node.widget.height_sizing(), Size::Fill);
                        if set_w || set_h {
                            let cur = node.widget.size();
                            let fill_w = if matches!(width_sizing, Size::Auto) {
                                // auto-width col: fill to widest non-fill child
                                let mut max_w = 0.0f32;
                                for cid in &children {
                                    if let Some(Some(cn)) = self.nodes.get(*cid) {
                                        if !matches!(cn.widget.width_sizing(), Size::Fill) {
                                            max_w = max_w.max(cn.widget.size().0);
                                        }
                                    }
                                }
                                max_w
                            } else {
                                // fixed/fill col: fill to inner_w
                                inner_w
                            };
                            let new_w = if set_w { fill_w } else { cur.0 };
                            let new_h = if set_h && !is_auto_h { fill_h } else { cur.1 };
                            if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                node.widget.set_size(new_w, new_h);
                            }
                        }
                    }
                }

                let mut cursor = 0.0;
                for child_id in &children {
                    let (_, h) = match self.nodes[*child_id].as_ref() {
                        Some(n) => n.widget.size(),
                        None => continue,
                    };
                    if let Some(n) = self.nodes[*child_id].as_mut() {
                        n.widget.set_position(0.0, cursor);
                        self.request_redraw();
                    }
                    cursor += h + gap;
                    let child_w = self.nodes[*child_id]
                        .as_ref()
                        .map(|n| n.widget.size().0)
                        .unwrap_or(inner_w);
                    let child_h = self.nodes[*child_id]
                        .as_ref()
                        .map(|n| n.widget.size().1)
                        .unwrap_or(content_h);
                    self.layout_node(*child_id, child_w, child_h);
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
                        let final_w = if matches!(g.width, Size::Auto) {
                            total_w
                        } else {
                            g.w
                        };
                        let final_h = if matches!(g.height, Size::Auto) {
                            total_h
                        } else {
                            g.h
                        };
                        let size_changed = g.w != final_w || g.h != final_h;
                        g.w = final_w;
                        g.h = final_h;
                        if size_changed {
                            if let Some(parent_id) = self.nodes[id].as_ref().and_then(|n| n.parent)
                            {
                                self.layout_dirty.insert(parent_id);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn collect_draw_list(&mut self) -> DrawList {
        let roots = self.roots.clone();
        let mut draw_list = DrawList::new();
        for id in roots {
            self.render_node(id, &mut draw_list, Accumulated::identity());
        }
        draw_list.sort_by_z();

        // DEBUG
        if self.debug {
            if let Some([x, y, w, h]) = self.hovered_rect {
                let rect_color = [0.0, 0.384, 1.0, 0.549];
                let crosshair_color = [0.0, 0.533, 1.0, 1.0];
                let t = 1.0;

                draw_list.push_rect(RectDraw {
                    x,
                    y,
                    w,
                    h,
                    color: rect_color,
                    radii: [0.0; 4],
                    border_color: [0.0; 4],
                    border_widths: [0.0; 4],
                    rotate: 0.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    opacity: 1.0,
                    clip: None,
                    z: i32::MAX,
                });

                // top
                draw_list.push_rect(RectDraw {
                    x: 0.0,
                    y: y - t,
                    w: self.viewport_w,
                    h: t,
                    color: crosshair_color,
                    radii: [0.0; 4],
                    border_color: [0.0; 4],
                    border_widths: [0.0; 4],
                    rotate: 0.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    opacity: 1.0,
                    clip: None,
                    z: i32::MAX,
                });
                // bottom
                draw_list.push_rect(RectDraw {
                    x: 0.0,
                    y: y + h,
                    w: self.viewport_w,
                    h: t,
                    color: crosshair_color,
                    radii: [0.0; 4],
                    border_color: [0.0; 4],
                    border_widths: [0.0; 4],
                    rotate: 0.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    opacity: 1.0,
                    clip: None,
                    z: i32::MAX,
                });
                // left
                draw_list.push_rect(RectDraw {
                    x: x - t,
                    y: 0.0,
                    w: t,
                    h: self.viewport_h,
                    color: crosshair_color,
                    radii: [0.0; 4],
                    border_color: [0.0; 4],
                    border_widths: [0.0; 4],
                    rotate: 0.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    opacity: 1.0,
                    clip: None,
                    z: i32::MAX,
                });
                // right
                draw_list.push_rect(RectDraw {
                    x: x + w,
                    y: 0.0,
                    w: t,
                    h: self.viewport_h,
                    color: crosshair_color,
                    radii: [0.0; 4],
                    border_color: [0.0; 4],
                    border_widths: [0.0; 4],
                    rotate: 0.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    opacity: 1.0,
                    clip: None,
                    z: i32::MAX,
                });
            }
        }

        draw_list
    }

    fn intersects_clip(clip: Option<[f32; 4]>, ox: f32, oy: f32, w: f32, h: f32) -> bool {
        let Some([cx, cy, cw, ch]) = clip else {
            return true;
        };
        ox + w > cx && ox < cx + cw && oy + h > cy && oy < cy + ch
    }

    fn render_node(&mut self, id: usize, draw_list: &mut DrawList, acc: Accumulated) {
        let (ox, oy, z, scroll, w, h, do_clip, visible, children) = {
            let Some(Some(s)) = self.nodes.get(id) else {
                return;
            };
            let (ox, oy) = s.widget.position();
            let z = s.widget.z();
            let group = s.widget.as_any().downcast_ref::<Group>();
            let scroll = group
                .map(|g| (g.scroll_x, g.scroll_y))
                .unwrap_or((0.0, 0.0));
            let (w, h) = s.widget.size();
            let do_clip = group.map(|g| g.clip).unwrap_or(false);
            let visible = group.map(|g| g.visible).unwrap_or(true);
            let children = s.children.clone();
            (ox, oy, z, scroll, w, h, do_clip, visible, children)
        };

        if !visible {
            return;
        }

        let my_acc = acc.push(ox, oy, None, z);
        if !Self::intersects_clip(acc.clip, my_acc.offset_x, my_acc.offset_y, w, h) {
            return;
        }

        let clip = if do_clip {
            Some([my_acc.offset_x, my_acc.offset_y, w, h])
        } else {
            None
        };
        let children_acc = acc.push(ox + scroll.0, oy + scroll.1, clip, z);

        {
            let Some(Some(s)) = self.nodes.get_mut(id) else {
                return;
            };
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
        }

        for child_id in children {
            self.render_node(child_id, draw_list, children_acc);
        }
    }

    pub fn set_cursor(&mut self, cursor: CursorIcon) {
        self.cursor = cursor;
    }

    pub fn set_focus<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        self.focused = Some(handle.id);
    }

    pub fn clear_focus(&mut self) {
        self.focused = None;
    }

    pub fn capture_mouse(&mut self, handle: WidgetHandle<impl Widget + 'static>) {
        self.captured_widget = Some(handle.id);
    }

    pub fn release_mouse(&mut self) {
        self.captured_widget = None;
    }
}

pub struct ListenerHandle(u64);

struct Listener {
    id: u64,
    node_id: usize,
    type_id: TypeId,
    f: Box<dyn FnMut(&dyn Any, &mut Ui)>,
}

impl Ui {
    pub fn listen<W: Widget + 'static, E: 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        let id = self.next_listener_id;
        self.next_listener_id += 1;
        let mut f = f;
        self.listeners.push(Listener {
            id,
            node_id: handle.id,
            type_id: TypeId::of::<E>(),
            f: Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
            }),
        });
        ListenerHandle(id)
    }

    pub fn listen_global<E: 'static>(
        &mut self,
        f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        let id = self.next_listener_id;
        self.next_listener_id += 1;
        let mut f = f;
        self.listeners.push(Listener {
            id,
            node_id: usize::MAX,
            type_id: TypeId::of::<E>(),
            f: Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
            }),
        });
        ListenerHandle(id)
    }

    pub fn unlisten(&mut self, handle: ListenerHandle) {
        self.listeners.retain(|l| l.id != handle.0);
    }

    fn fire(&mut self, node_id: usize, event: Box<dyn Any>) {
        let type_id = (*event).type_id();
        let mut i = 0;
        while i < self.listeners.len() {
            if self.listeners[i].node_id == node_id && self.listeners[i].type_id == type_id {
                let mut listener = self.listeners.remove(i);
                (listener.f)(event.as_ref(), self);
                self.listeners.insert(i, listener);
            }
            i += 1;
        }
    }

    fn fire_global(&mut self, event: Box<dyn Any>) {
        let type_id = (*event).type_id();
        let mut i = 0;
        while i < self.listeners.len() {
            if self.listeners[i].node_id == usize::MAX && self.listeners[i].type_id == type_id {
                let mut listener = self.listeners.remove(i);
                (listener.f)(event.as_ref(), self);
                self.listeners.insert(i, listener);
            }
            i += 1;
        }
    }
}

impl Ui {
    pub fn process_input(&mut self) {
        self.keyboard_stuff();
        self.mouse_stuff();
    }

    pub fn keyboard_stuff(&mut self) {
        for (k, _) in self.input.keyboard.just_pressed() {
            if *k == Key::Equals {
                self.print_nodes();
            }
        }
        self.fire_key_events();
    }

    pub fn mouse_stuff(&mut self) {
        self.fire_hover_events();
        self.fire_mouse_move();
        self.fire_click_events();
        self.fire_scroll_events();
    }

    fn fire_key_events(&mut self) {
        let pressed: Vec<(Key, Option<char>)> = self.input.keyboard.just_pressed().to_vec();
        let released: Vec<Key> = self.input.keyboard.just_released().to_vec();

        if let Some(node_id) = self.focused {
            for (key, ch) in &pressed {
                self.fire(node_id, Box::new(KeyPress { key: *key, ch: *ch }));
            }
            for key in &released {
                self.fire(node_id, Box::new(KeyRelease { key: *key }));
            }
        }
    }

    fn fire_hover_events(&mut self) {
        if self.input.mouse.dx != 0.0 || self.input.mouse.dy != 0.0 {
            let prev = self.hovered_node;
            self.hit_test();
            if prev != self.hovered_node {
                self.cursor = CursorIcon::Default;
                if let Some(old_id) = prev {
                    self.fire(old_id, Box::new(HoverLeave));
                }
                if let Some(new_id) = self.hovered_node {
                    self.fire(new_id, Box::new(HoverEnter));
                }
            }
        }
    }

    fn fire_mouse_move(&mut self) {
        if self.input.mouse.dx != 0.0 || self.input.mouse.dy != 0.0 {
            let target = self.captured_widget.or(self.hovered_node);
            if let Some(node_id) = target {
                self.fire(
                    node_id,
                    Box::new(MouseMove {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        dx: self.input.mouse.dx,
                        dy: self.input.mouse.dy,
                    }),
                );
            }
        }
    }

    fn fire_click_events(&mut self) {
        let mouse_up_target = self.captured_widget.or(self.hovered_node);
        let hover_target = self.hovered_node;

        if let Some(node_id) = hover_target {
            if self.input.mouse.left.just_pressed {
                let now = web_time::Instant::now();
                let dt = now
                    .duration_since(self.input.mouse.left.last_click_time)
                    .as_millis();
                let dx = (self.input.mouse.x - self.input.mouse.left.last_click_x).abs();
                let dy = (self.input.mouse.y - self.input.mouse.left.last_click_y).abs();
                if dt < 400 && dx < 5.0 && dy < 5.0 {
                    self.input.mouse.left.click_count += 1;
                } else {
                    self.input.mouse.left.click_count = 1;
                }
                self.input.mouse.left.last_click_time = now;
                self.input.mouse.left.last_click_x = self.input.mouse.x;
                self.input.mouse.left.last_click_y = self.input.mouse.y;
                self.fire(
                    node_id,
                    Box::new(MouseDown {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: MouseButton::Left,
                    }),
                );
                self.fire_global(Box::new(MouseDown {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: MouseButton::Left,
                }));
                if self.focused != Some(node_id) {
                    if let Some(old_id) = self.focused {
                        self.fire(old_id, Box::new(FocusLost));
                    }
                    self.focused = Some(node_id);
                    self.fire(node_id, Box::new(FocusGained));
                }
            }
            if self.input.mouse.right.just_pressed {
                let now = web_time::Instant::now();
                let dt = now
                    .duration_since(self.input.mouse.right.last_click_time)
                    .as_millis();
                let dx = (self.input.mouse.x - self.input.mouse.right.last_click_x).abs();
                let dy = (self.input.mouse.y - self.input.mouse.right.last_click_y).abs();
                if dt < 400 && dx < 5.0 && dy < 5.0 {
                    self.input.mouse.right.click_count += 1;
                } else {
                    self.input.mouse.right.click_count = 1;
                }
                self.input.mouse.right.last_click_time = now;
                self.input.mouse.right.last_click_x = self.input.mouse.x;
                self.input.mouse.right.last_click_y = self.input.mouse.y;
                self.fire(
                    node_id,
                    Box::new(MouseDown {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: MouseButton::Right,
                    }),
                );
                self.fire_global(Box::new(MouseDown {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: MouseButton::Right,
                }));
            }
            if self.input.mouse.middle.just_pressed {
                let now = web_time::Instant::now();
                let dt = now
                    .duration_since(self.input.mouse.middle.last_click_time)
                    .as_millis();
                let dx = (self.input.mouse.x - self.input.mouse.middle.last_click_x).abs();
                let dy = (self.input.mouse.y - self.input.mouse.middle.last_click_y).abs();
                if dt < 400 && dx < 5.0 && dy < 5.0 {
                    self.input.mouse.middle.click_count += 1;
                } else {
                    self.input.mouse.middle.click_count = 1;
                }
                self.input.mouse.middle.last_click_time = now;
                self.input.mouse.middle.last_click_x = self.input.mouse.x;
                self.input.mouse.middle.last_click_y = self.input.mouse.y;
                self.fire(
                    node_id,
                    Box::new(MouseDown {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: MouseButton::Middle,
                    }),
                );
                self.fire_global(Box::new(MouseDown {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: MouseButton::Middle,
                }));
            }
        }

        if let Some(node_id) = mouse_up_target {
            if self.input.mouse.left.just_released {
                self.fire(
                    node_id,
                    Box::new(MouseUp {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: MouseButton::Left,
                    }),
                );
                if hover_target == Some(node_id) {
                    self.fire(
                        node_id,
                        Box::new(Click {
                            x: self.input.mouse.x,
                            y: self.input.mouse.y,
                            button: MouseButton::Left,
                        }),
                    );
                }
            }
            if self.input.mouse.right.just_released {
                self.fire(
                    node_id,
                    Box::new(MouseUp {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: MouseButton::Right,
                    }),
                );
                if hover_target == Some(node_id) {
                    self.fire(
                        node_id,
                        Box::new(Click {
                            x: self.input.mouse.x,
                            y: self.input.mouse.y,
                            button: MouseButton::Right,
                        }),
                    );
                }
            }
            if self.input.mouse.middle.just_released {
                self.fire(
                    node_id,
                    Box::new(MouseUp {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: MouseButton::Middle,
                    }),
                );
                if hover_target == Some(node_id) {
                    self.fire(
                        node_id,
                        Box::new(Click {
                            x: self.input.mouse.x,
                            y: self.input.mouse.y,
                            button: MouseButton::Middle,
                        }),
                    );
                }
            }
        } else if self.input.mouse.left.just_released {
            if let Some(old_id) = self.focused {
                self.fire(old_id, Box::new(FocusLost));
            }
            self.focused = None;
        }
    }

    fn fire_scroll_events(&mut self) {
        if self.input.mouse.scroll_x != 0.0 || self.input.mouse.scroll_y != 0.0 {
            if let Some(node_id) = self.hovered_node {
                self.fire(
                    node_id,
                    Box::new(MouseScroll {
                        x: self.input.mouse.scroll_x,
                        y: self.input.mouse.scroll_y,
                    }),
                );
            }
        }
    }

    pub fn debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn hit_test(&mut self) {
        let mx = self.input.mouse.x;
        let my = self.input.mouse.y;
        let prev_hovered = self.hovered_node;
        self.hovered_node = None;
        self.hovered_rect = None;
        let mut best_z = i32::MIN;
        for &id in &self.roots {
            if let Some((hit, rect, z)) = self.hit_test_node(id, mx, my, Accumulated::identity()) {
                if z >= best_z {
                    best_z = z;
                    self.hovered_node = Some(hit);
                    self.hovered_rect = Some(rect);
                }
            }
        }
        if self.hovered_node != prev_hovered {
            self.needs_redraw = true;
        }
    }

    fn hit_test_node(
        &self,
        id: usize,
        mx: f32,
        my: f32,
        acc: Accumulated,
    ) -> Option<(usize, [f32; 4], i32)> {
        let Some(Some(node)) = self.nodes.get(id) else {
            return None;
        };
        let group = node.widget.as_any().downcast_ref::<Group>();
        let visible = group.map(|g| g.visible).unwrap_or(true);
        if !visible {
            return None;
        }

        let (ox, oy) = node.widget.position();
        let my_acc = acc.push(ox, oy, None, node.widget.z());
        let scroll = group
            .map(|g| (g.scroll_x, g.scroll_y))
            .unwrap_or((0.0, 0.0));
        let children_acc = acc.push(ox + scroll.0, oy + scroll.1, None, node.widget.z());
        let (w, h) = node.widget.size();
        let x = my_acc.offset_x;
        let y = my_acc.offset_y;

        let mut result: Option<(usize, [f32; 4], i32)> = None;
        for &child_id in &node.children {
            if let Some(hit) = self.hit_test_node(child_id, mx, my, children_acc) {
                match &result {
                    None => result = Some(hit),
                    Some(prev) => {
                        if hit.2 >= prev.2 {
                            result = Some(hit);
                        }
                    }
                }
            }
        }

        if w > 0.0 && h > 0.0 && mx >= x && mx <= x + w && my >= y && my <= y + h {
            let my_z = my_acc.z;
            match &result {
                None => result = Some((id, [x, y, w, h], my_z)),
                Some(prev) => {
                    if my_z > prev.2 {
                        result = Some((id, [x, y, w, h], my_z));
                    }
                }
            }
        }

        result
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
