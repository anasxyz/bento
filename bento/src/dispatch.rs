use crate::input::{InputState, MouseButton};
use crate::ui::{Event, Ui};
use crate::widget::Handle;

/// walk the widget tree and collect all widgets that contain (mx, my),
/// innermost last
fn hit_chain(ui: &Ui, handle: Handle<()>, mx: f32, my: f32, chain: &mut Vec<Handle<()>>) {
    let Some(rect) = ui.layout.get_rect(handle) else {
        return;
    };
    let (x, y, w, h) = rect;

    if mx < x || mx > x + w || my < y || my > y + h {
        return;
    }

    chain.push(handle);

    let children = ui.children(handle).to_vec();
    for child in children {
        hit_chain(ui, child, mx, my, chain);
    }
}

/// topmost hit
/// innermost widget under the cursor
fn top_hit(chain: &[Handle<()>]) -> Option<Handle<()>> {
    chain.last().copied()
}

// main dispatch 

pub fn dispatch(ui: &mut Ui, input: &InputState) {
    let Some(root) = ui.root() else { return };

    let mx = input.mouse.x;
    let my = input.mouse.y;

    // build hit chain
    let mut chain: Vec<Handle<()>> = Vec::new();
    hit_chain(ui, root, mx, my, &mut chain);

    let new_hovered = top_hit(&chain);
    let global = ui.global();

    // mouse move 
    {
        // always send move to pressed widget first (for dragging outside bounds)
        if let Some(pressed) = ui.interaction.pressed {
            if !chain.contains(&pressed) {
                ui.with_widget(pressed, |w, _| w.on_mouse_move(mx, my));
            }
        }
        let targets = chain.clone();
        for handle in targets.iter().rev() {
            ui.with_widget(*handle, |w, _| w.on_mouse_move(mx, my));
        }
        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Event::MouseMove { x: mx, y: my }),
            None => ui.emit(global, Event::MouseMove { x: mx, y: my }),
        }
    }

    // hover enter / leave 
    let prev_hovered = ui.interaction.hovered;
    if prev_hovered != new_hovered {
        if let Some(prev) = prev_hovered {
            ui.with_widget(prev, |w, _| w.on_mouse_leave());
            ui.emit_bubbling(prev, Event::HoverEnd);
        }
        if let Some(next) = new_hovered {
            ui.with_widget(next, |w, _| w.on_mouse_enter());
            ui.emit_bubbling(next, Event::Hover);
        }
        ui.interaction.hovered = new_hovered;
    }

    // scroll 
    if input.mouse.just_scrolled {
        let dx = input.mouse.scroll_x;
        let dy = input.mouse.scroll_y;
        let targets = chain.clone();
        for handle in targets.iter().rev() {
            ui.with_widget(*handle, |w, _| w.on_mouse_scroll(dx, dy));
        }
        if let Some(target) = new_hovered.or(new_hovered) {
            ui.emit_bubbling(target, Event::Scroll { x: dx, y: dy });
        }
    }

    // left press 
    if input.mouse.left.just_pressed {
        let (lx, ly) = (input.mouse.left.click_x, input.mouse.left.click_y);
        let targets = chain.clone();
        for handle in targets.iter().rev() {
            ui.with_widget(*handle, |w, _| w.on_mouse_press(lx, ly, MouseButton::Left));
        }
        ui.interaction.pressed = new_hovered;

        if input.mouse.left.just_double_clicked {
            if let Some(target) = ui.interaction.pressed {
                ui.with_widget(target, |w, _| {
                    w.on_mouse_double_click(lx, ly, MouseButton::Left)
                });
                ui.emit_bubbling(target, Event::DoubleClick { x: lx, y: ly });
            }
        }

        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Event::Press { x: lx, y: ly }),
            None => ui.emit(global, Event::Press { x: lx, y: ly }),
        }

        // focus
        // update on left press
        let new_focused = new_hovered;
        if ui.interaction.focused != new_focused {
            if let Some(prev) = ui.interaction.focused {
                ui.with_widget(prev, |w, _| w.on_focus_lost());
                ui.emit_bubbling(prev, Event::FocusLost);
            }
            if let Some(next) = new_focused {
                ui.with_widget(next, |w, _| w.on_focus_gained());
                ui.emit_bubbling(next, Event::FocusGained);
            }
            ui.interaction.focused = new_focused;
        }
    }

    // left release 
    if input.mouse.left.just_released {
        let (lx, ly) = (input.mouse.left.click_x, input.mouse.left.click_y);
        let pressed = ui.interaction.pressed;

        // release hook goes to the originally pressed widget
        if let Some(p) = pressed {
            ui.with_widget(p, |w, _| w.on_mouse_release(lx, ly, MouseButton::Left));
            // click = press and release on same widget
            if new_hovered == Some(p) {
                ui.with_widget(p, |w, _| w.on_mouse_click(lx, ly, MouseButton::Left));
                ui.emit_bubbling(p, Event::Click { x: lx, y: ly });
            }
        }

        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Event::Release { x: lx, y: ly }),
            None => ui.emit(global, Event::Release { x: lx, y: ly }),
        }

        ui.interaction.pressed = None;
    }

    // right press / release 
    if input.mouse.right.just_pressed {
        let (rx, ry) = (input.mouse.right.click_x, input.mouse.right.click_y);
        let targets = chain.clone();
        for handle in targets.iter().rev() {
            ui.with_widget(*handle, |w, _| w.on_mouse_press(rx, ry, MouseButton::Right));
        }
        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Event::Press { x: rx, y: ry }),
            None => ui.emit(global, Event::Press { x: rx, y: ry }),
        }
    }

    if input.mouse.right.just_released {
        let (rx, ry) = (input.mouse.right.click_x, input.mouse.right.click_y);
        let targets = chain.clone();
        for handle in targets.iter().rev() {
            ui.with_widget(*handle, |w, _| {
                w.on_mouse_release(rx, ry, MouseButton::Right)
            });
            ui.with_widget(*handle, |w, _| w.on_mouse_click(rx, ry, MouseButton::Right));
        }
        if let Some(h) = new_hovered {
            ui.emit_bubbling(h, Event::RightClick { x: rx, y: ry });
        }
    }

    // middle press / release 
    if input.mouse.middle.just_pressed {
        let (mx2, my2) = (input.mouse.middle.click_x, input.mouse.middle.click_y);
        let targets = chain.clone();
        for handle in targets.iter().rev() {
            ui.with_widget(*handle, |w, _| {
                w.on_mouse_press(mx2, my2, MouseButton::Middle)
            });
        }
    }

    if input.mouse.middle.just_released {
        let (mx2, my2) = (input.mouse.middle.click_x, input.mouse.middle.click_y);
        let targets = chain.clone();
        for handle in targets.iter().rev() {
            ui.with_widget(*handle, |w, _| {
                w.on_mouse_release(mx2, my2, MouseButton::Middle)
            });
        }
    }

    // keyboard
    // routed to focused widget 
    if let Some(focused) = ui.interaction.focused {
        let mods = input.keyboard.modifiers.clone();
        for (key, text) in input.keyboard.just_pressed().to_vec() {
            let (k, m, t) = (key.clone(), mods.clone(), text);
            ui.with_widget(focused, |w, _| w.on_key_press(k, m, t));
            ui.emit_bubbling(
                focused,
                Event::KeyPress {
                    key: format!("{:?}", key),
                    text,
                },
            );
        }
        for key in input.keyboard.just_released().to_vec() {
            let (k, m) = (key.clone(), mods.clone());
            ui.with_widget(focused, |w, _| w.on_key_release(k, m));
            ui.emit_bubbling(
                focused,
                Event::KeyRelease {
                    key: format!("{:?}", key),
                },
            );
        }
    }
}
