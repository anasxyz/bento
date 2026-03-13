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

fn fire_signal(ui: &mut Ui, hits: &[Handle<()>], signal: Signal) {
    let mut connections = ui.take_connections();
    for handle in hits {
        let pos = connections
            .iter()
            .position(|c| c.handle == *handle && c.signal == signal);
        if let Some(pos) = pos {
            let cb_ptr: *const dyn Fn(&mut Ui) = connections[pos].callback.as_ref();
            unsafe { (*cb_ptr)(ui) };
            ui.restore_connections(connections);
            return;
        }
    }
    ui.restore_connections(connections);
}

fn fire_signal_for(ui: &mut Ui, handle: Handle<()>, signal: Signal) {
    let mut connections = ui.take_connections();
    let pos = connections
        .iter()
        .position(|c| c.handle == handle && c.signal == signal);
    if let Some(pos) = pos {
        let cb_ptr: *const dyn Fn(&mut Ui) = connections[pos].callback.as_ref();
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
    let new_hovered = hover_hits.first().copied();

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
        let pressed = press_hits.first().copied();
        if let Some(h) = pressed {
            fire_signal_for(ui, h, Signal::Press);
        }
        ui.interaction.pressed = pressed;
    }

    // --- release + click ---
    if mouse.left_just_released {
        let mut release_hits: Vec<Handle<()>> = Vec::new();
        hit_test(ui, root, mx, my, &mut release_hits);
        let released = release_hits.first().copied();

        if let Some(h) = released {
            fire_signal_for(ui, h, Signal::Release);
            // only fire click if released on same element that was pressed
            if ui.interaction.pressed == Some(h) {
                fire_signal_for(ui, h, Signal::Click);
            }
        }
        ui.interaction.pressed = None;
    }
}
