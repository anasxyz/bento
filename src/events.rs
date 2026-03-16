use crate::element::handle::Handle;
use crate::event::Event;
use crate::mouse::MouseButton;
use crate::ui::Ui;

fn hit_test(ui: &Ui, handle: Handle<()>, mx: f32, my: f32, hits: &mut Vec<Handle<()>>) {
    let el = match ui.get_any(handle) {
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

    if ui.has_connections(handle) {
        hits.push(handle);
    }
}

fn top_hit(ui: &Ui, hits: &[Handle<()>]) -> Option<Handle<()>> {
    hits.iter()
        .copied()
        .enumerate()
        .max_by(|(i, a), (j, b)| {
            let az = ui.get_any(*a).map(|e| e.layout().z_index).unwrap_or(0);
            let bz = ui.get_any(*b).map(|e| e.layout().z_index).unwrap_or(0);
            az.cmp(&bz).then(i.cmp(j))
        })
        .map(|(_, h)| h)
}

pub fn fire_events(ui: &mut Ui) {
    let root = match ui.root() {
        Some(r) => r,
        None => return,
    };

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

    let global = ui.global();

    let mut hover_hits: Vec<Handle<()>> = Vec::new();
    hit_test(ui, root, mx, my, &mut hover_hits);
    let new_hovered = top_hit(ui, &hover_hits);

    // mouse move
    if let Some(hovered) = new_hovered {
        ui.emit_bubbling(hovered, Event::MouseMove { x: mx, y: my });
    } else {
        ui.emit(global, Event::MouseMove { x: mx, y: my });
    }

    // hover enter/leave
    if ui.interaction.hovered != new_hovered {
        if let Some(prev) = ui.interaction.hovered {
            ui.get_any_mut(prev).map(|e| e.on_mouse_leave());
            ui.emit_bubbling(prev, Event::HoverEnd);
        }
        if let Some(next) = new_hovered {
            ui.get_any_mut(next).map(|e| e.on_mouse_enter());
            ui.emit_bubbling(next, Event::Hover);
        }
        ui.interaction.hovered = new_hovered;
    }

    // left press
    if left_pressed {
        if let Some(target) = new_hovered {
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_press(lx, ly, MouseButton::Left));
            ui.emit_bubbling(target, Event::Press { x: lx, y: ly });
            ui.interaction.pressed = Some(target);

            if double_clicked {
                ui.get_any_mut(target)
                    .map(|e| e.on_mouse_double_click(lx, ly, MouseButton::Left));
                ui.emit_bubbling(target, Event::DoubleClick { x: lx, y: ly });
            }
        } else {
            ui.emit(global, Event::Press { x: lx, y: ly });
        }
    }

    // left release
    if left_released {
        if let Some(target) = new_hovered {
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_release(lx, ly, MouseButton::Left));
            ui.emit_bubbling(target, Event::Release { x: lx, y: ly });

            if ui.interaction.pressed == Some(target) {
                ui.get_any_mut(target)
                    .map(|e| e.on_mouse_click(lx, ly, MouseButton::Left));
                ui.emit_bubbling(target, Event::Click { x: lx, y: ly });
            }
        } else {
            ui.emit(global, Event::Release { x: lx, y: ly });
        }
        ui.interaction.pressed = None;
    }

    // right
    if right_pressed {
        if let Some(target) = new_hovered {
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_press(rx, ry, MouseButton::Right));
            ui.emit_bubbling(target, Event::Press { x: rx, y: ry });
        } else {
            ui.emit(global, Event::Press { x: rx, y: ry });
        }
    }
    if right_released {
        if let Some(target) = new_hovered {
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_release(rx, ry, MouseButton::Right));
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_click(rx, ry, MouseButton::Right));
            ui.emit_bubbling(target, Event::RightClick { x: rx, y: ry });
        } else {
            ui.emit(global, Event::RightClick { x: rx, y: ry });
        }
    }

    // middle
    if middle_pressed {
        if let Some(target) = new_hovered {
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_press(midx, midy, MouseButton::Middle));
        }
    }
    if middle_released {
        if let Some(target) = new_hovered {
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_release(midx, midy, MouseButton::Middle));
            ui.get_any_mut(target)
                .map(|e| e.on_mouse_click(midx, midy, MouseButton::Middle));
        }
    }

    // focus
    if left_pressed {
        let new_focused = new_hovered;
        if ui.interaction.focused != new_focused {
            if let Some(prev) = ui.interaction.focused {
                ui.get_any_mut(prev).map(|e| e.on_focus_lost());
                ui.emit_bubbling(prev, Event::FocusLost);
            }
            if let Some(next) = new_focused {
                ui.get_any_mut(next).map(|e| e.on_focus_gained());
                ui.emit_bubbling(next, Event::FocusGained);
            }
            ui.interaction.focused = new_focused;
        }
    }
}
