use crate::element::handle::Handle;
use crate::mouse::MouseState;
use crate::ui::Ui;

fn hit_test(ui: &Ui, handle: Handle<()>, mx: f32, my: f32, hits: &mut Vec<Handle<()>>) {
    let el = match ui.get_dyn(handle) {
        Some(e) => e,
        None => return,
    };

    let layout = el.layout();

    if !layout.visible {
        return;
    }

    let inside =
        mx >= layout.x && mx <= layout.x + layout.w && my >= layout.y && my <= layout.y + layout.h;

    if !inside {
        return;
    }

    let children = ui.children(handle).to_vec();
    for child in children {
        hit_test(ui, child, mx, my, hits);
    }

    hits.push(handle);
}

pub fn fire_events(ui: &mut Ui, mouse: &MouseState) {
    let root = match ui.root() {
        Some(r) => r,
        None => return,
    };

    let mx = mouse.x;
    let my = mouse.y;

    // --- hover ---
    let mut hover_hits: Vec<Handle<()>> = Vec::new();
    hit_test(ui, root, mx, my, &mut hover_hits);
    let new_hovered = hover_hits.first().copied();

    if ui.interaction.hovered != new_hovered {
        // leave old
        if let Some(prev) = ui.interaction.hovered {
            if let Some(signal) = ui.get_dyn_mut(prev).and_then(|e| e.on_mouse_leave()) {
                ui.emit(prev, signal);
            }
        }
        // enter new
        if let Some(next) = new_hovered {
            if let Some(signal) = ui.get_dyn_mut(next).and_then(|e| e.on_mouse_enter()) {
                ui.emit(next, signal);
            }
        }
        ui.interaction.hovered = new_hovered;
    }

    // --- press ---
    if mouse.left_just_pressed {
        if let Some(target) = new_hovered {
            if let Some(signal) = ui.get_dyn_mut(target).and_then(|e| e.on_press()) {
                ui.emit(target, signal);
            }
            ui.interaction.pressed = Some(target);
        }
    }

    // --- release + click ---
    if mouse.left_just_released {
        if let Some(target) = new_hovered {
            if let Some(signal) = ui.get_dyn_mut(target).and_then(|e| e.on_release()) {
                let is_click = ui.interaction.pressed == Some(target);
                if is_click {
                    ui.emit(target, signal);
                }
            }
        }
        ui.interaction.pressed = None;
    }
}
