use crate::Color;
use crate::element::{Element, ElementType, Position};
use crate::render::draw_ctx::DrawContext;
use crate::render::shape_renderer::ShapeDrawParams;
use crate::render::text_renderer::TextDrawParams;

enum DrawCall {
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
    el: &Element,
    clip: Option<[f32; 4]>,
    parent_z: i32,
    parent_opacity: f32,
    calls: &mut Vec<DrawCall>,
) {
    // skip invisible elements entirely
    if !el.style.visible {
        return;
    }

    let z = parent_z + el.style.z_index;
    let opacity = parent_opacity * el.style.opacity;

    match el._type {
        ElementType::Rect => {
            let mut color = el.style.fill.to_array();
            color[3] *= opacity;

            let mut border_color = el.style.border_color.unwrap_or(Color::BLACK).to_array();
            border_color[3] *= opacity;

            calls.push(DrawCall::Rect {
                x: el.style.x,
                y: el.style.y,
                w: el.style.w,
                h: el.style.h,
                params: ShapeDrawParams {
                    color,
                    radius: el.style.border_radius.unwrap_or(0.0),
                    border_color,
                    border_width: el.style.border_thickness,
                    clip,
                },
                z_index: z,
            });
        }
        ElementType::Text => {
            let mut text_color = el.style.text_color;
            text_color.a *= opacity;

            calls.push(DrawCall::Text {
                x: el.style.x,
                y: el.style.y,
                content: el.style.text_content.clone(),
                params: TextDrawParams {
                    family: el.style.font_family.clone(),
                    size: el.style.font_size,
                    weight: el.style.font_weight,
                    italic: el.style.font_italic,
                    color: text_color,
                    width: if el.style.w > 0.0 {
                        el.style.w
                    } else {
                        f32::MAX
                    },
                    clip,
                },
                z_index: z,
            });
        }
        ElementType::Row | ElementType::Col => {
            let my_clip = Some([
                el.style.x,
                el.style.y,
                el.style.x + el.style.w,
                el.style.y + el.style.h,
            ]);

            if let Some(children) = &el.children {
                for child in children {
                    // absolutely positioned children escape parent clip
                    let child_clip = if child.style.position == Position::Absolute {
                        clip // only outer clip, not this container's clip
                    } else {
                        clip_intersect(clip, my_clip)
                    };
                    collect_draws(child, child_clip, z, opacity, calls);
                }
            }
        }
    }
}

pub fn draw_tree(el: &Element, draw: &mut DrawContext) {
    let mut calls: Vec<DrawCall> = Vec::new();
    collect_draws(el, None, 0, 1.0, &mut calls);

    // stable sort so tree order is preserved within same z
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
