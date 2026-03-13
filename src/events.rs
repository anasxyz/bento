use crate::element::handle::Handle;
use crate::mouse::MouseState;
use crate::signals::Signal;
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

// Walk up the parent chain from a given node to find the first node
// that has a connection for the given signal.
fn find_handler(ui: &Ui, start: Handle<()>, signal: &Signal) -> Option<Handle<()>> {
    let mut current = Some(start);
    while let Some(handle) = current {
        let has_handler = ui
            .connections_ref()
            .iter()
            .any(|c| c.handle == handle && c.signal == *signal);
        if has_handler {
            return Some(handle);
        }
        current = ui.parent(handle);
    }
    None
}

// Fire all matching connections for a handle+signal, in registration order
fn fire_signal_for(ui: &mut Ui, handle: Handle<()>, signal: Signal) {
    let mut connections = ui.take_connections();
    let indices: Vec<usize> = connections
        .iter()
        .enumerate()
        .filter(|(_, c)| c.handle == handle && c.signal == signal)
        .map(|(i, _)| i)
        .collect();
    for i in indices {
        let cb_ptr: *const dyn Fn(&mut Ui) = connections[i].callback.as_ref();
        unsafe { (*cb_ptr)(ui) };
    }
    ui.restore_connections(connections);
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

    let new_hovered = hover_hits
        .first()
        .and_then(|&deepest| find_handler(ui, deepest, &Signal::Hover));

    if ui.interaction.hovered != new_hovered {
        if let Some(prev) = ui.interaction.hovered {
            fire_signal_for(ui, prev, Signal::HoverEnd);
        }
        if let Some(next) = new_hovered {
            fire_signal_for(ui, next, Signal::Hover);
        }
        ui.interaction.hovered = new_hovered;
    }

    // --- press ---
    if mouse.left_just_pressed {
        let mut press_hits: Vec<Handle<()>> = Vec::new();
        hit_test(ui, root, mx, my, &mut press_hits);
        let pressed = press_hits.first().and_then(|&deepest| {
            find_handler(ui, deepest, &Signal::Press).or(find_handler(ui, deepest, &Signal::Click))
        });
        if let Some(h) = pressed {
            fire_signal_for(ui, h, Signal::Press);
        }
        ui.interaction.pressed = press_hits.first().copied();
    }

    // --- release + click ---
    if mouse.left_just_released {
        let mut release_hits: Vec<Handle<()>> = Vec::new();
        hit_test(ui, root, mx, my, &mut release_hits);

        if let Some(&deepest) = release_hits.first() {
            if let Some(release_handler) = find_handler(ui, deepest, &Signal::Release) {
                fire_signal_for(ui, release_handler, Signal::Release);
            }

            if let Some(pressed) = ui.interaction.pressed {
                let is_same_or_child = release_hits.contains(&pressed);
                if is_same_or_child {
                    if let Some(click_handler) = find_handler(ui, deepest, &Signal::Click) {
                        fire_signal_for(ui, click_handler, Signal::Click);
                    }
                }
            }
        }

        ui.interaction.pressed = None;
    }
}
