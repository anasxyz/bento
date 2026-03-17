use crate::Color;
use crate::element::element::AnyElement;
use crate::element::handle::Handle;
use crate::element::values::Position;
use crate::render::draw_ctx::DrawContext;
use crate::render::shape_renderer::ShapeDrawParams;
use crate::render::text_renderer::TextDrawParams;
use crate::ui::Ui;
use std::collections::HashMap;

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
    pub fn z_index(&self) -> i32 {
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

// per element draw data
struct ElementDrawData {
    calls: Vec<DrawCall>,
}

// persistent draw list
// only dirty elements get rebuilt
pub struct DrawList {
    elements: Vec<(Handle<()>, ElementDrawData)>,
    sorted: Vec<DrawCall>,
}

impl DrawList {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            sorted: Vec::new(),
        }
    }

    pub fn invalidate(&mut self) {
        self.elements.clear();
        self.sorted.clear();
    }
}

// build draw calls for a single element (not its children)
fn element_calls(
    ui: &Ui,
    handle: Handle<()>,
    clip: Option<[f32; 4]>,
    z: i32,
    opacity: f32,
) -> Vec<DrawCall> {
    let mut calls = Vec::new();
    let el = match ui.get_any(handle) {
        Some(e) => e,
        None => return calls,
    };

    match el {
        AnyElement::Rect(rect) => {
            let mut color = rect.bg_color().to_array();
            color[3] *= opacity;
            let mut border_color = rect.border_color().unwrap_or(Color::BLACK).to_array();
            border_color[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: el.layout().x,
                y: el.layout().y,
                w: el.layout().w,
                h: el.layout().h,
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
            let layout = el.layout();
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
            let layout = el.layout();
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

    calls
}

// traverse the tree computing clip/z/opacity for each element
// only rebuilds draw data for dirty elements
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
        // println!("rebuilding draw calls for handle id={}", handle.id);
        let calls = element_calls(ui, handle, clip, z, opacity);
        if let Some(entry) = draw_list.elements.iter_mut().find(|(h, _)| *h == handle) {
            entry.1 = ElementDrawData { calls };
        } else {
            draw_list.elements.push((handle, ElementDrawData { calls }));
        }
    }

    let children = ui.children(handle).to_vec();
    for child_handle in children {
        let child_clip = match ui.get_any(child_handle) {
            Some(el) if el.layout().position == Position::Absolute => clip,
            _ => clip_intersect(clip, my_clip),
        };
        traverse(
            ui,
            child_handle,
            child_clip,
            z,
            opacity,
            should_rebuild,
            draw_list,
        );
    }
}

pub fn update_draw_list(ui: &Ui, draw_list: &mut DrawList) {
    let root = match ui.root() {
        Some(r) => r,
        None => return,
    };
    traverse(ui, root, None, 0, 1.0, false, draw_list);

    // always rebuild sorted from elements every dirty frame
    draw_list.sorted.clear();
    for (_, data) in &draw_list.elements {
        for call in &data.calls {
            match call {
                DrawCall::Rect {
                    x,
                    y,
                    w,
                    h,
                    params,
                    z_index,
                } => {
                    draw_list.sorted.push(DrawCall::Rect {
                        x: *x,
                        y: *y,
                        w: *w,
                        h: *h,
                        params: params.clone(),
                        z_index: *z_index,
                    });
                }
                DrawCall::Text {
                    x,
                    y,
                    content,
                    params,
                    z_index,
                } => {
                    draw_list.sorted.push(DrawCall::Text {
                        x: *x,
                        y: *y,
                        content: content.clone(),
                        params: params.clone(),
                        z_index: *z_index,
                    });
                }
            }
        }
    }
    draw_list.sorted.sort_by_key(|c| c.z_index());
}

// submit the sorted draw list to the GPU
pub fn submit_draw_list(draw_list: &DrawList, draw: &mut DrawContext) {
    for call in &draw_list.sorted {
        match call {
            DrawCall::Rect {
                x, y, w, h, params, ..
            } => {
                draw.draw_rect(*x, *y, *w, *h, params.clone());
            }
            DrawCall::Text {
                x,
                y,
                content,
                params,
                ..
            } => {
                draw.draw_text(*x, *y, content, params.clone());
            }
        }
    }
}
