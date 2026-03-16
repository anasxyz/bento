use crate::element::handle::Handle;
use crate::mouse::MouseButton;
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

fn fire_on(ui: &mut Ui, target: Handle<()>, signal: Option<u32>) {
    if let Some(s) = signal {
        ui.emit_bubbling(target, s);
    }
}

pub fn fire_events(ui: &mut Ui) {
    let root = match ui.root() {
        Some(r) => r,
        None => return,
    };

    // copy all mouse state we need upfront to avoid borrow conflicts
    let mx = ui.mouse.x;
    let my = ui.mouse.y;
    let lx = ui.mouse.left_click_x;
    let ly = ui.mouse.left_click_y;
    let rx = ui.mouse.right_click_x;
    let ry = ui.mouse.right_click_y;
    let midx = ui.mouse.middle_click_x;
    let midy = ui.mouse.middle_click_y;
    let left_pressed = ui.mouse.left_just_pressed;
    let left_released = ui.mouse.left_just_released;
    let right_pressed = ui.mouse.right_just_pressed;
    let right_released = ui.mouse.right_just_released;
    let middle_pressed = ui.mouse.middle_just_pressed;
    let middle_released = ui.mouse.middle_just_released;
    let double_clicked = ui.mouse.left_just_double_clicked;

    // --- hover ---
    let mut hover_hits: Vec<Handle<()>> = Vec::new();
    hit_test(ui, root, mx, my, &mut hover_hits);
    let new_hovered = hover_hits.first().copied();

    if ui.interaction.hovered != new_hovered {
        if let Some(prev) = ui.interaction.hovered {
            let signal = ui.get_dyn_mut(prev).and_then(|e| e.on_mouse_leave());
            fire_on(ui, prev, signal);
        }
        if let Some(next) = new_hovered {
            let signal = ui.get_dyn_mut(next).and_then(|e| e.on_mouse_enter());
            fire_on(ui, next, signal);
        }
        ui.interaction.hovered = new_hovered;
    }

    // --- left ---
    if left_pressed {
        if let Some(target) = new_hovered {
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_press(lx, ly, MouseButton::Left));
            fire_on(ui, target, signal);
            ui.interaction.pressed = Some(target);

            if double_clicked {
                let signal = ui
                    .get_dyn_mut(target)
                    .and_then(|e| e.on_mouse_double_click(lx, ly, MouseButton::Left));
                fire_on(ui, target, signal);
            }
        }
    }

    if left_released {
        if let Some(target) = new_hovered {
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_release(lx, ly, MouseButton::Left));
            fire_on(ui, target, signal);

            if ui.interaction.pressed == Some(target) {
                let signal = ui
                    .get_dyn_mut(target)
                    .and_then(|e| e.on_mouse_click(lx, ly, MouseButton::Left));
                fire_on(ui, target, signal);
            }
        }
        ui.interaction.pressed = None;
    }

    // --- right ---
    if right_pressed {
        if let Some(target) = new_hovered {
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_press(rx, ry, MouseButton::Right));
            fire_on(ui, target, signal);
        }
    }

    if right_released {
        if let Some(target) = new_hovered {
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_release(rx, ry, MouseButton::Right));
            fire_on(ui, target, signal);
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_click(rx, ry, MouseButton::Right));
            fire_on(ui, target, signal);
        }
    }

    // --- middle ---
    if middle_pressed {
        if let Some(target) = new_hovered {
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_press(midx, midy, MouseButton::Middle));
            fire_on(ui, target, signal);
        }
    }

    if middle_released {
        if let Some(target) = new_hovered {
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_release(midx, midy, MouseButton::Middle));
            fire_on(ui, target, signal);
            let signal = ui
                .get_dyn_mut(target)
                .and_then(|e| e.on_mouse_click(midx, midy, MouseButton::Middle));
            fire_on(ui, target, signal);
        }
    }

    // --- focus on left click ---
    if left_pressed {
        let new_focused = new_hovered;
        if ui.interaction.focused != new_focused {
            if let Some(prev) = ui.interaction.focused {
                let signal = ui.get_dyn_mut(prev).and_then(|e| e.on_focus_lost());
                fire_on(ui, prev, signal);
            }
            if let Some(next) = new_focused {
                let signal = ui.get_dyn_mut(next).and_then(|e| e.on_focus_gained());
                fire_on(ui, next, signal);
            }
            ui.interaction.focused = new_focused;
        }
    }
}
