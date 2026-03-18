use super::types::Event;
use crate::element::element::EventResult;
use crate::element::handle::Handle;
use crate::input::MouseButton;
use crate::ui::Ui;

// builds hit chain from outermost to innermost for all elements under cursor
fn hit_chain(ui: &Ui, handle: Handle<()>, mx: f32, my: f32, chain: &mut Vec<Handle<()>>) {
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
    chain.push(handle);
    let children = ui.children(handle).to_vec();
    for child in children {
        hit_chain(ui, child, mx, my, chain);
    }
}

// builds hit chain for connected elements only 
// for event emission
fn connected_chain(ui: &Ui, handle: Handle<()>, mx: f32, my: f32, chain: &mut Vec<Handle<()>>) {
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
        connected_chain(ui, child, mx, my, chain);
    }
    if ui.has_connections(handle) {
        chain.push(handle);
    }
}

fn top_hit(ui: &Ui, chain: &[Handle<()>]) -> Option<Handle<()>> {
    chain
        .iter()
        .copied()
        .enumerate()
        .max_by(|(i, a), (j, b)| {
            let az = ui.get_any(*a).map(|e| e.layout().z_index).unwrap_or(0);
            let bz = ui.get_any(*b).map(|e| e.layout().z_index).unwrap_or(0);
            az.cmp(&bz).then(i.cmp(j))
        })
        .map(|(_, h)| h)
}

// walk chain innermost first, stop when Handled
macro_rules! propagate {
    ($chain:expr, $ui:expr, $method:ident ( $($arg:expr),* )) => {{
        let mut claimed: Option<Handle<()>> = None;
        for handle in $chain.iter().rev() {
            let result = $ui.get_any_mut(*handle)
                .map(|e| e.$method($($arg),*))
                .unwrap_or(EventResult::Propagate);
            if result == EventResult::Handled {
                claimed = Some(*handle);
                break;
            }
        }
        claimed
    }};
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
    let just_scrolled = ui.mouse.just_scrolled;
    let scroll_dx = ui.mouse.scroll_delta_x;
    let scroll_dy = ui.mouse.scroll_delta_y;

    let global = ui.global();

    // full chain (all elements)
    // for hook propagation
    let mut chain: Vec<Handle<()>> = Vec::new();
    hit_chain(ui, root, mx, my, &mut chain);

    // connected chain
    // for event emission and hover tracking
    let mut conn_chain: Vec<Handle<()>> = Vec::new();
    connected_chain(ui, root, mx, my, &mut conn_chain);
    let new_hovered = top_hit(ui, &conn_chain);

    // mouse move 
    // on pressed element first (drag), then propagate through chain
    if let Some(pressed) = ui.interaction.pressed {
        let result = ui
            .get_any_mut(pressed)
            .map(|e| e.on_mouse_move(mx, my))
            .unwrap_or(EventResult::Propagate);
        if result == EventResult::Propagate {
            propagate!(chain, ui, on_mouse_move(mx, my));
        }
    } else {
        propagate!(chain, ui, on_mouse_move(mx, my));
    }

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

    // scroll — propagate innermost to outermost, stop when handled
    if just_scrolled {
        let claimed = propagate!(chain, ui, on_mouse_scroll(scroll_dx, scroll_dy));
        let emit_target = claimed.or(chain.last().copied());
        if let Some(target) = emit_target {
            ui.emit_bubbling(
                target,
                Event::Scroll {
                    x: scroll_dx,
                    y: scroll_dy,
                },
            );
        }
    }

    // left press
    if left_pressed {
        let claimed = propagate!(chain, ui, on_mouse_press(lx, ly, MouseButton::Left));
        ui.interaction.pressed = claimed.or_else(|| top_hit(ui, &chain));

        if double_clicked {
            if let Some(target) = ui.interaction.pressed {
                ui.get_any_mut(target)
                    .map(|e| e.on_mouse_double_click(lx, ly, MouseButton::Left));
            }
        }
        if let Some(target) = new_hovered {
            ui.emit_bubbling(target, Event::Press { x: lx, y: ly });
            if double_clicked {
                ui.emit_bubbling(target, Event::DoubleClick { x: lx, y: ly });
            }
        } else {
            ui.emit(global, Event::Press { x: lx, y: ly });
        }
    }

    // left release
    if left_released {
        if let Some(pressed) = ui.interaction.pressed {
            ui.get_any_mut(pressed)
                .map(|e| e.on_mouse_release(lx, ly, MouseButton::Left));
        } else {
            propagate!(chain, ui, on_mouse_release(lx, ly, MouseButton::Left));
        }
        if let Some(target) = new_hovered {
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
        propagate!(chain, ui, on_mouse_press(rx, ry, MouseButton::Right));
        if let Some(target) = new_hovered {
            ui.emit_bubbling(target, Event::Press { x: rx, y: ry });
        } else {
            ui.emit(global, Event::Press { x: rx, y: ry });
        }
    }
    if right_released {
        propagate!(chain, ui, on_mouse_release(rx, ry, MouseButton::Right));
        propagate!(chain, ui, on_mouse_click(rx, ry, MouseButton::Right));
        if let Some(target) = new_hovered {
            ui.emit_bubbling(target, Event::RightClick { x: rx, y: ry });
        } else {
            ui.emit(global, Event::RightClick { x: rx, y: ry });
        }
    }

    // middle
    if middle_pressed {
        propagate!(chain, ui, on_mouse_press(midx, midy, MouseButton::Middle));
    }
    if middle_released {
        propagate!(chain, ui, on_mouse_release(midx, midy, MouseButton::Middle));
        propagate!(chain, ui, on_mouse_click(midx, midy, MouseButton::Middle));
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
