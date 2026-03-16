use crate::Color;
use crate::element::element::AnyElement;
use crate::element::handle::Handle;
use crate::element::values::Position;
use crate::render::draw_ctx::DrawContext;
use crate::render::shape_renderer::ShapeDrawParams;
use crate::render::text_renderer::TextDrawParams;
use crate::ui::Ui;

pub enum DrawCall {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        params: ShapeDrawParams,
        z_index: i32,
    },
    Text {
        x: f32,
        y: f32,
        content: String,
        params: TextDrawParams,
        z_index: i32,
    },
}

impl DrawCall {
    fn z_index(&self) -> i32 {
        match self {
            DrawCall::Rect { z_index, .. } => *z_index,
            DrawCall::Text { z_index, .. } => *z_index,
        }
    }
}

pub fn clip_intersect(a: Option<[f32; 4]>, b: Option<[f32; 4]>) -> Option<[f32; 4]> {
    match (a, b) {
        (Some([ax, ay, ax2, ay2]), Some([bx, by, bx2, by2])) => {
            Some([ax.max(bx), ay.max(by), ax2.min(bx2), ay2.min(by2)])
        }
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

pub fn collect_draws(
    ui: &Ui,
    handle: Handle<()>,
    clip: Option<[f32; 4]>,
    parent_z: i32,
    parent_opacity: f32,
    calls: &mut Vec<DrawCall>,
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

    match el {
        AnyElement::Rect(rect) => {
            let mut color = rect.bg_color().to_array();
            color[3] *= opacity;
            let mut border_color = rect.border_color().unwrap_or(Color::BLACK).to_array();
            border_color[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: layout.x,
                y: layout.y,
                w: layout.w,
                h: layout.h,
                params: ShapeDrawParams {
                    color,
                    radius: rect.border_radius().unwrap_or(0.0),
                    border_color,
                    border_widths: rect.border_widths(),
                    clip,
                },
                z_index: z,
            });
        }
        AnyElement::Label(label) => {
            let mut text_color = label.text_color();
            text_color.a *= opacity;
            calls.push(DrawCall::Text {
                x: layout.x,
                y: layout.y,
                content: label.text().to_string(),
                params: TextDrawParams {
                    family: label.font_family().to_string(),
                    size: label.font_size(),
                    weight: label.font_weight(),
                    italic: label.font_italic(),
                    color: text_color,
                    width: if layout.w > 0.0 { layout.w } else { f32::MAX },
                    clip,
                },
                z_index: z,
            });
        }
        AnyElement::Container(container) => {
            if let Some(bg) = container.bg_color() {
                let mut color = bg.to_array();
                color[3] *= opacity;
                let mut border_color = container.border_color().unwrap_or(Color::BLACK).to_array();
                border_color[3] *= opacity;
                calls.push(DrawCall::Rect {
                    x: layout.x,
                    y: layout.y,
                    w: layout.w,
                    h: layout.h,
                    params: ShapeDrawParams {
                        color,
                        radius: container.border_radius().unwrap_or(0.0),
                        border_color,
                        border_widths: container.border_widths(),
                        clip,
                    },
                    z_index: z,
                });
            }
        }
    }

    let my_clip = Some([layout.x, layout.y, layout.x + layout.w, layout.y + layout.h]);
    let children = ui.children(handle).to_vec();

    for child_handle in children {
        let child_clip = match ui.get_any(child_handle) {
            Some(el) if el.layout().position == Position::Absolute => clip,
            _ => clip_intersect(clip, my_clip),
        };
        collect_draws(ui, child_handle, child_clip, z, opacity, calls);
    }
}

pub fn draw_tree(ui: &Ui, draw: &mut DrawContext) {
    let root = match ui.root() {
        Some(r) => r,
        None => return,
    };

    let mut calls: Vec<DrawCall> = Vec::new();
    collect_draws(ui, root, None, 0, 1.0, &mut calls);
    calls.sort_by_key(|c| c.z_index());

    for call in calls {
        match call {
            DrawCall::Rect {
                x, y, w, h, params, ..
            } => draw.draw_rect(x, y, w, h, params),
            DrawCall::Text {
                x,
                y,
                content,
                params,
                ..
            } => draw.draw_text(x, y, &content, params),
        }
    }
}

// returns the union of all dirty element rects in logical pixels
// None means nothing dirty, Some means the region that needs repainting
pub fn dirty_region(ui: &Ui) -> Option<[f32; 4]> {
    let mut region: Option<[f32; 4]> = None;
    for slot in ui.slots.iter().filter_map(|s| s.as_ref()) {
        if !slot.element.is_dirty() {
            continue;
        }
        let l = slot.element.layout();
        // skip elements with no size yet
        if l.w == 0.0 && l.h == 0.0 {
            continue;
        }
        let rect = [l.x, l.y, l.x + l.w, l.y + l.h];
        region = Some(match region {
            None => rect,
            Some([ax, ay, ax2, ay2]) => [
                ax.min(rect[0]),
                ay.min(rect[1]),
                ax2.max(rect[2]),
                ay2.max(rect[3]),
            ],
        });
    }
    region
}
