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

fn fire_on(ui: &mut Ui, target: Handle<()>, signal: Option<u32>) {
    if let Some(s) = signal {
        ui.emit_bubbling(target, s);
    }
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
    if mouse.left_just_pressed {
        if let Some(target) = new_hovered {
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_left_press());
            fire_on(ui, target, signal);
            ui.interaction.pressed = Some(target);

            if mouse.left_just_double_clicked {
                let signal = ui
                    .get_dyn_mut(target)
                    .and_then(|e| e.on_left_double_click());
                fire_on(ui, target, signal);
            }
        }
    }

    if mouse.left_just_released {
        if let Some(target) = new_hovered {
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_left_release());
            fire_on(ui, target, signal);

            if ui.interaction.pressed == Some(target) {
                let signal = ui.get_dyn_mut(target).and_then(|e| e.on_left_click());
                fire_on(ui, target, signal);
            }
        }
        ui.interaction.pressed = None;
    }

    // --- right ---
    if mouse.right_just_pressed {
        if let Some(target) = new_hovered {
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_right_press());
            fire_on(ui, target, signal);
        }
    }

    if mouse.right_just_released {
        if let Some(target) = new_hovered {
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_right_release());
            fire_on(ui, target, signal);
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_right_click());
            fire_on(ui, target, signal);
        }
    }

    // --- middle ---
    if mouse.middle_just_pressed {
        if let Some(target) = new_hovered {
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_middle_press());
            fire_on(ui, target, signal);
        }
    }

    if mouse.middle_just_released {
        if let Some(target) = new_hovered {
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_middle_release());
            fire_on(ui, target, signal);
            let signal = ui.get_dyn_mut(target).and_then(|e| e.on_middle_click());
            fire_on(ui, target, signal);
        }
    }

    // --- focus on left click ---
    if mouse.left_just_pressed {
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
