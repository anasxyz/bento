use super::context::DrawContext;
use super::shapes::RectParams;
use super::text::TextParams;
use crate::color::Color;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::values::Position;
use crate::ui::Ui;
use wgpu;

// DrawCall

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

// DrawList

struct ElementDrawData {
    calls: Vec<DrawCall>,
    offset_x: f32,
    offset_y: f32,
    culled: bool,
}

struct DrawList {
    elements: Vec<(Handle<()>, ElementDrawData)>,
    // z-sorted draw calls rebuilt when sort_dirty
    sorted: Vec<DrawCall>,
    sort_dirty: bool,
    // any element rebuilt this frame (need to re-submit rects)
    any_rebuilt: bool,
}

impl DrawList {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            sorted: Vec::new(),
            sort_dirty: true,
            any_rebuilt: false,
        }
    }
    fn invalidate(&mut self) {
        self.elements.clear();
        self.sorted.clear();
        self.sort_dirty = true;
        self.any_rebuilt = true;
    }
}

// helpers

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

fn offset_call(call: DrawCall, ox: f32, oy: f32) -> DrawCall {
    if ox == 0.0 && oy == 0.0 {
        return call;
    }
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
            z_index,
        } => DrawCall::Rect {
            x: x + ox,
            y: y + oy,
            w,
            h,
            color,
            radius,
            border_color,
            border_widths,
            clip,
            z_index,
        },
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
            z_index,
        } => DrawCall::Text {
            x: x + ox,
            y: y + oy,
            content,
            family,
            size,
            weight,
            italic,
            color,
            width,
            clip,
            z_index,
        },
    }
}

fn is_outside_clip(clip: Option<[f32; 4]>, x: f32, y: f32, w: f32, h: f32) -> bool {
    match clip {
        Some([cx, cy, cx2, cy2]) => x + w <= cx || y + h <= cy || x >= cx2 || y >= cy2,
        None => false,
    }
}

fn traverse(
    ui: &Ui,
    handle: Handle<()>,
    clip: Option<[f32; 4]>,
    parent_z: i32,
    parent_opacity: f32,
    parent_dirty: bool,
    offset_x: f32,
    offset_y: f32,
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

    let (tx, ty) = layout.transform.unwrap_or((0.0, 0.0));
    let child_offset_x = offset_x + tx;
    let child_offset_y = offset_y + ty;

    let ex = layout.x + offset_x;
    let ey = layout.y + offset_y;

    let my_clip = Some([ex, ey, ex + layout.w, ey + layout.h]);

    if is_outside_clip(clip, ex, ey, layout.w, layout.h) {
        if let Some(entry) = draw_list.elements.iter_mut().find(|(h, _)| *h == handle) {
            if !entry.1.culled {
                draw_list.sort_dirty = true;
                draw_list.any_rebuilt = true;
            }
            entry.1.culled = true;
        } else {
            draw_list.elements.push((
                handle,
                ElementDrawData {
                    calls: vec![],
                    offset_x: f32::NAN,
                    offset_y: f32::NAN,
                    culled: true,
                },
            ));
        }
        let children = ui.children(handle).to_vec();
        for child in children {
            let child_clip = match ui.get_any(child) {
                Some(el) if el.layout().position == Position::Absolute => clip,
                _ => clip_intersect(clip, my_clip),
            };
            traverse(
                ui,
                child,
                child_clip,
                z,
                opacity,
                parent_dirty,
                child_offset_x,
                child_offset_y,
                draw_list,
            );
        }
        return;
    }

    let existing = draw_list.elements.iter().find(|(h, _)| *h == handle);
    let offset_changed = existing
        .map(|(_, data)| data.culled || data.offset_x != offset_x || data.offset_y != offset_y)
        .unwrap_or(true);

    let should_rebuild = el.is_dirty() || parent_dirty || offset_changed;

    if should_rebuild {
        let calls: Vec<DrawCall> = el
            .draw_calls(clip, z, opacity)
            .into_iter()
            .map(|c| offset_call(c, offset_x, offset_y))
            .collect();

        // check if z-indices changed
        // if so the sorted list needs rebuilding
        let z_changed = existing
            .map(|(_, data)| {
                data.culled
                    || data.calls.len() != calls.len()
                    || data
                        .calls
                        .iter()
                        .zip(calls.iter())
                        .any(|(a, b)| a.z_index() != b.z_index())
            })
            .unwrap_or(true);

        if z_changed {
            draw_list.sort_dirty = true;
        }

        draw_list.any_rebuilt = true;

        if let Some(entry) = draw_list.elements.iter_mut().find(|(h, _)| *h == handle) {
            entry.1 = ElementDrawData {
                calls,
                offset_x,
                offset_y,
                culled: false,
            };
        } else {
            draw_list.elements.push((
                handle,
                ElementDrawData {
                    calls,
                    offset_x,
                    offset_y,
                    culled: false,
                },
            ));
            draw_list.sort_dirty = true;
        }
    } else {
        if let Some(entry) = draw_list.elements.iter_mut().find(|(h, _)| *h == handle) {
            entry.1.culled = false;
        }
    }

    let children = ui.children(handle).to_vec();
    for child in children {
        let child_clip = match ui.get_any(child) {
            Some(el) if el.layout().position == Position::Absolute => clip,
            _ => clip_intersect(clip, my_clip),
        };
        traverse(
            ui,
            child,
            child_clip,
            z,
            opacity,
            should_rebuild,
            child_offset_x,
            child_offset_y,
            draw_list,
        );
    }
}

fn rebuild_sorted(draw_list: &mut DrawList) {
    draw_list.sorted.clear();
    for (_, data) in &draw_list.elements {
        if !data.culled {
            draw_list.sorted.extend(data.calls.iter().cloned());
        }
    }
    draw_list.sorted.sort_by_key(|c| c.z_index());
}

// submit rects and text from the sorted list into the draw context
// rects go through the slot api (write_slot only uploads if data changed),
// text goes through the text renderers own caching
// slot_cursor tracks how many rect slots ive used this frame so slots
// are reused in the same z-sorted positions across frames
fn submit(draw_list: &DrawList, ctx: &mut DrawContext) {
    let mut rect_idx = 0usize;
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
            } => {
                // ensure slot exists at this index
                ctx.ensure_rect_slot(rect_idx);
                ctx.write_rect_slot(
                    rect_idx,
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
                );
                rect_idx += 1;
            }
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
            } => {
                ctx.draw_text(
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
                );
            }
        }
    }
    // free any slots beyond what we used this frame
    ctx.truncate_rect_slots(rect_idx);
}

// Renderer

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
        self.ctx.invalidate_rects();
    }

    pub fn paint(&mut self, ui: &Ui, clear_color: [f32; 4]) {
        self.ctx.clear_text();
        self.ctx.draw_clear(clear_color);

        let root = match ui.root() {
            Some(r) => r,
            None => return,
        };

        traverse(ui, root, None, 0, 1.0, false, 0.0, 0.0, &mut self.draw_list);

        if self.draw_list.any_rebuilt {
            println!("renderer: rebuilt sort_dirty={}", self.draw_list.sort_dirty);
            rebuild_sorted(&mut self.draw_list);
            if self.draw_list.sort_dirty {
                // z-order changed
                // invalidate all rect slots so they get
                // rewritten in the new sorted order
                self.ctx.invalidate_rects();
                self.draw_list.sort_dirty = false;
            }
            submit(&self.draw_list, &mut self.ctx);
            self.draw_list.any_rebuilt = false;
        } else {
            println!("renderer: skipped");
        }
    }
}
