use crate::Color;
use crate::element::button::Button;
use crate::element::container::Container;
use crate::element::handle::Handle;
use crate::element::label::Label;
use crate::element::rect::Rect;
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
    let el = match ui.get_dyn(handle) {
        Some(e) => e,
        None => return,
    };

    let layout = el.layout();

    if !layout.visible {
        return;
    }

    let z = parent_z + layout.z_index;
    let opacity = parent_opacity * layout.opacity;

    if let Some(rect) = el.as_any().downcast_ref::<Rect>() {
        let mut color = rect.bg_color.to_array();
        color[3] *= opacity;

        let mut border_color = rect.border_color.unwrap_or(Color::BLACK).to_array();
        border_color[3] *= opacity;

        calls.push(DrawCall::Rect {
            x: layout.x,
            y: layout.y,
            w: layout.w,
            h: layout.h,
            params: ShapeDrawParams {
                color,
                radius: rect.border_radius.unwrap_or(0.0),
                border_color,
                border_width: rect.border_thickness,
                clip,
            },
            z_index: z,
        });
    } else if let Some(label) = el.as_any().downcast_ref::<Label>() {
        let mut text_color = label.text_color;
        text_color.a *= opacity;

        calls.push(DrawCall::Text {
            x: layout.x,
            y: layout.y,
            content: label.text.clone(),
            params: TextDrawParams {
                family: label.font_family.clone(),
                size: label.font_size,
                weight: label.font_weight,
                italic: label.font_italic,
                color: text_color,
                width: if layout.w > 0.0 { layout.w } else { f32::MAX },
                clip,
            },
            z_index: z,
        });
    } else if let Some(container) = el.as_any().downcast_ref::<Container>() {
        if let Some(bg) = container.bg_color {
            let mut color = bg.to_array();
            color[3] *= opacity;

            let mut border_color = container.border_color.unwrap_or(Color::BLACK).to_array();
            border_color[3] *= opacity;

            calls.push(DrawCall::Rect {
                x: layout.x,
                y: layout.y,
                w: layout.w,
                h: layout.h,
                params: ShapeDrawParams {
                    color,
                    radius: container.border_radius.unwrap_or(0.0),
                    border_color,
                    border_width: container.border_thickness,
                    clip,
                },
                z_index: z,
            });
        }
    } else if let Some(btn) = el.as_any().downcast_ref::<Button>() {
        let color = if btn.disabled {
            btn.color.desaturate(0.6).lighten(0.05)
        } else if btn.pressed {
            btn.color.darken(0.08)
        } else if btn.hovered {
            btn.color.lighten(0.08)
        } else {
            btn.color
        };

        let mut bg = color.to_array();
        bg[3] *= opacity;

        let mut border_color = btn.border_color.unwrap_or(Color::TRANSPARENT).to_array();
        border_color[3] *= opacity;

        calls.push(DrawCall::Rect {
            x: layout.x,
            y: layout.y,
            w: layout.w,
            h: layout.h,
            params: ShapeDrawParams {
                color: bg,
                radius: btn.border_radius,
                border_color,
                border_width: btn.border_thickness,
                clip,
            },
            z_index: z,
        });

        let mut text_color = btn.text_color;
        if btn.disabled {
            text_color.a *= 0.4;
        }
        text_color.a *= opacity;

        let tw = btn.text_w.get();
        let th = btn.text_h.get();
        let tx = layout.x + (layout.w - tw) / 2.0;
        let ty = layout.y + (layout.h - th) / 2.0;

        calls.push(DrawCall::Text {
            x: tx,
            y: ty,
            content: btn.text.clone(),
            params: TextDrawParams {
                family: btn.font_family.clone(),
                size: btn.font_size,
                weight: btn.font_weight,
                italic: false,
                color: text_color,
                width: f32::MAX,
                clip,
            },
            z_index: z,
        });
    }

    // always recurse into children
    let my_clip = Some([layout.x, layout.y, layout.x + layout.w, layout.y + layout.h]);
    let children = ui.children(handle).to_vec();

    for child_handle in children {
        let child_position = ui
            .get_dyn(child_handle)
            .map(|c| c.layout().position.clone());

        let child_clip = if child_position == Some(Position::Absolute) {
            clip
        } else {
            clip_intersect(clip, my_clip)
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
            } => {
                draw.draw_rect(x, y, w, h, params);
            }
            DrawCall::Text {
                x,
                y,
                content,
                params,
                ..
            } => {
                draw.draw_text(x, y, &content, params);
            }
        }
    }
}
