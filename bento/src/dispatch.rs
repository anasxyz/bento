use crate::input::InputState;
use crate::ui::Ui;
use crate::ui::{
    Click, DoubleClick, FocusGained, FocusLost, Hover, HoverEnd, KeyPress, KeyRelease, MouseMove,
    Press, Release, RightClick, Scroll,
};
use crate::widget::Handle;

fn hit_chain(ui: &Ui, handle: Handle<()>, mx: f32, my: f32, chain: &mut Vec<(Handle<()>, u32)>) {
    let Some(rect) = ui.layout.get_rect(handle) else {
        return;
    };
    let (x, y, w, h) = rect;
    if mx < x || mx > x + w || my < y || my > y + h {
        return;
    }
    let layer = ui.get_any(handle).map(|w| w.base().layer).unwrap_or(0);
    chain.push((handle, layer));
    let children = ui.children(handle).to_vec();
    for child in children {
        hit_chain(ui, child, mx, my, chain);
    }
}

fn top_hit(ui: &Ui, chain: &[(Handle<()>, u32)]) -> Option<Handle<()>> {
    if chain.is_empty() {
        return None;
    }

    // find the highest layer that has any widget at this position
    let max_layer = chain.iter().map(|&(_, l)| l).max().unwrap_or(0);

    // only consider widgets on the highest layer
    let top_layer: Vec<Handle<()>> = chain
        .iter()
        .filter(|&&(_, l)| l == max_layer)
        .map(|&(h, _)| h)
        .collect();

    // within that layer, prefer interactive widgets
    // otherwise take the last deepest one
    for &handle in top_layer.iter().rev() {
        if let Some(w) = ui.get_any(handle) {
            if w.is_interactive() {
                return Some(handle);
            }
        }
    }
    top_layer.last().copied()
}

pub fn dispatch(ui: &mut Ui, input: &InputState) {
    let Some(root) = ui.root() else { return };

    let mx = input.mouse.x;
    let my = input.mouse.y;

    let mut chain: Vec<(Handle<()>, u32)> = Vec::new();
    hit_chain(ui, root, mx, my, &mut chain);

    let new_hovered = top_hit(ui, &chain);
    let global = ui.global();
    let chain_handles: Vec<Handle<()>> = chain.iter().map(|&(h, _)| h).collect();

    // mouse move
    {
        if let Some(pressed) = ui.interaction.pressed {
            if !chain_handles.contains(&pressed) {
                ui.emit(pressed, MouseMove::new(mx, my));
            }
        }
        match new_hovered {
            Some(h) => ui.emit_bubbling(h, MouseMove::new(mx, my)),
            None => ui.emit(global, MouseMove::new(mx, my)),
        }
    }

    // hover enter / leave
    let prev_hovered = ui.interaction.hovered;
    if prev_hovered != new_hovered {
        if let Some(prev) = prev_hovered {
            ui.emit_bubbling(prev, HoverEnd::new());
        }
        if let Some(next) = new_hovered {
            ui.emit_bubbling(next, Hover::new());
        }
        ui.interaction.hovered = new_hovered;
    }

    // scroll
    if input.mouse.just_scrolled {
        let dx = input.mouse.scroll_x;
        let dy = input.mouse.scroll_y;
        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Scroll::new(dx, dy)),
            None => ui.emit(global, Scroll::new(dx, dy)),
        }
    }

    // left press
    if input.mouse.left.just_pressed {
        let (lx, ly) = (input.mouse.left.click_x, input.mouse.left.click_y);
        ui.interaction.pressed = new_hovered;

        if input.mouse.left.just_double_clicked {
            if let Some(target) = ui.interaction.pressed {
                ui.emit_bubbling(target, DoubleClick::new(lx, ly));
            }
        }

        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Press::new(lx, ly)),
            None => ui.emit(global, Press::new(lx, ly)),
        }

        // focus
        let new_focused = new_hovered;
        if ui.interaction.focused != new_focused {
            if let Some(prev) = ui.interaction.focused {
                ui.emit_bubbling(prev, FocusLost::new());
            }
            if let Some(next) = new_focused {
                ui.emit_bubbling(next, FocusGained::new());
            }
            ui.interaction.focused = new_focused;
        }
    }

    // left release
    if input.mouse.left.just_released {
        let (lx, ly) = (input.mouse.left.click_x, input.mouse.left.click_y);
        let pressed = ui.interaction.pressed;

        if let Some(p) = pressed {
            if new_hovered == Some(p) {
                ui.emit_bubbling(p, Click::new(lx, ly));
            }
        }

        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Release::new(lx, ly)),
            None => ui.emit(global, Release::new(lx, ly)),
        }

        ui.interaction.pressed = None;
    }

    // right press / release
    if input.mouse.right.just_pressed {
        let (rx, ry) = (input.mouse.right.click_x, input.mouse.right.click_y);
        match new_hovered {
            Some(h) => ui.emit_bubbling(h, Press::new(rx, ry)),
            None => ui.emit(global, Press::new(rx, ry)),
        }
    }

    if input.mouse.right.just_released {
        let (rx, ry) = (input.mouse.right.click_x, input.mouse.right.click_y);
        if let Some(h) = new_hovered {
            ui.emit_bubbling(h, RightClick::new(rx, ry));
        }
    }

    // keyboard
    if let Some(focused) = ui.interaction.focused {
        let mods = input.keyboard.modifiers.clone();
        for (key, text) in input.keyboard.just_pressed().to_vec() {
            ui.emit_bubbling(focused, KeyPress::new(key, text, mods.clone()));
        }
        for key in input.keyboard.just_released().to_vec() {
            ui.emit_bubbling(focused, KeyRelease::new(key));
        }
    }
}
