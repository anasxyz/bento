use super::context::DrawContext;
use super::shapes::RectParams;
use super::text::TextParams;
use crate::color::Color;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::values::Position;
use crate::ui::Ui;
use wgpu;

// ── DrawCall ──────────────────────────────────────────────────────────────────
// Plain data — no render imports. Elements produce these; renderer consumes them.

#[derive(Clone)]
pub enum DrawCall {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        radius: f32,
        border_color: [f32; 4],
        border_widths: [f32; 4],
        clip: Option<[f32; 4]>,
        z_index: i32,
    },
    Text {
        x: f32,
        y: f32,
        content: String,
        family: String,
        size: f32,
        weight: u16,
        italic: bool,
        color: [f32; 4],
        width: f32,
        clip: Option<[f32; 4]>,
        z_index: i32,
    },
}

impl DrawCall {
    fn z_index(&self) -> i32 {
        match self {
            DrawCall::Rect { z_index, .. } | DrawCall::Text { z_index, .. } => *z_index,
        }
    }
}

// ── DrawList ──────────────────────────────────────────────────────────────────

struct ElementDrawData {
    calls: Vec<DrawCall>,
}

struct DrawList {
    elements: Vec<(Handle<()>, ElementDrawData)>,
    sorted: Vec<DrawCall>,
}

impl DrawList {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            sorted: Vec::new(),
        }
    }
    fn invalidate(&mut self) {
        self.elements.clear();
        self.sorted.clear();
    }
}

// ── traversal ─────────────────────────────────────────────────────────────────

fn clip_intersect(a: Option<[f32; 4]>, b: Option<[f32; 4]>) -> Option<[f32; 4]> {
    match (a, b) {
        (Some([ax, ay, ax2, ay2]), Some([bx, by, bx2, by2])) => {
            Some([ax.max(bx), ay.max(by), ax2.min(bx2), ay2.min(by2)])
        }
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn traverse(
    ui: &Ui,
    handle: Handle<()>,
    clip: Option<[f32; 4]>,
    parent_z: i32,
    parent_opacity: f32,
    parent_dirty: bool,
    draw_list: &mut DrawList,
) {
    let el = match ui.get_any(handle) {
        Some(e) => e,
        None => return,
    };
    let layout = el.layout();
    if !layout.visible {
        return;
    }

    let z = parent_z + layout.z_index;
    let opacity = parent_opacity * layout.opacity;
    let my_clip = Some([layout.x, layout.y, layout.x + layout.w, layout.y + layout.h]);
    let should_rebuild = el.is_dirty() || parent_dirty;

    if should_rebuild {
        let calls = el.draw_calls(clip, z, opacity);
        if let Some(entry) = draw_list.elements.iter_mut().find(|(h, _)| *h == handle) {
            entry.1 = ElementDrawData { calls };
        } else {
            draw_list.elements.push((handle, ElementDrawData { calls }));
        }
    }

    let children = ui.children(handle).to_vec();
    for child in children {
        let child_clip = match ui.get_any(child) {
            Some(el) if el.layout().position == Position::Absolute => clip,
            _ => clip_intersect(clip, my_clip),
        };
        traverse(ui, child, child_clip, z, opacity, should_rebuild, draw_list);
    }
}

fn rebuild_sorted(draw_list: &mut DrawList) {
    draw_list.sorted.clear();
    for (_, data) in &draw_list.elements {
        draw_list.sorted.extend(data.calls.iter().cloned());
    }
    draw_list.sorted.sort_by_key(|c| c.z_index());
}

fn submit(draw_list: &DrawList, ctx: &mut DrawContext) {
    for call in &draw_list.sorted {
        match call {
            DrawCall::Rect {
                x,
                y,
                w,
                h,
                color,
                radius,
                border_color,
                border_widths,
                clip,
                ..
            } => ctx.draw_rect(
                *x,
                *y,
                *w,
                *h,
                RectParams {
                    color: *color,
                    radius: *radius,
                    border_color: *border_color,
                    border_widths: *border_widths,
                    clip: *clip,
                },
            ),
            DrawCall::Text {
                x,
                y,
                content,
                family,
                size,
                weight,
                italic,
                color,
                width,
                clip,
                ..
            } => ctx.draw_text(
                *x,
                *y,
                content,
                TextParams {
                    family: family.clone(),
                    size: *size,
                    weight: *weight,
                    italic: *italic,
                    color: Color::from_array(*color),
                    width: *width,
                    clip: *clip,
                },
            ),
        }
    }
}

// ── Renderer ──────────────────────────────────────────────────────────────────

pub struct Renderer {
    pub(super) ctx: DrawContext,
    draw_list: DrawList,
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        width: f32,
        height: f32,
        scale: f32,
    ) -> Self {
        Self {
            ctx: DrawContext::new(device, queue, format, width, height, scale),
            draw_list: DrawList::new(),
        }
    }

    pub fn resize(&mut self, scale: f32, width: f32, height: f32) {
        self.ctx.set_scale(scale, width, height);
        self.draw_list.invalidate();
    }

    pub fn invalidate(&mut self) {
        self.draw_list.invalidate();
    }

    pub fn paint(&mut self, ui: &Ui, clear_color: [f32; 4]) {
        self.ctx.clear();
        self.ctx.draw_clear(clear_color);
        let root = match ui.root() {
            Some(r) => r,
            None => return,
        };
        traverse(ui, root, None, 0, 1.0, false, &mut self.draw_list);
        rebuild_sorted(&mut self.draw_list);
        submit(&self.draw_list, &mut self.ctx);
    }
}
