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

    // recurse into children first so deepest node is at front of hits
    let children = ui.children(handle).to_vec();
    for child in children {
        hit_test(ui, child, mx, my, hits);
    }

    hits.push(handle);
}

fn fire_signal(ui: &mut Ui, hits: &[Handle<()>], signal: Signal) {
    // take all connections out to avoid borrow conflict
    let mut connections = ui.take_connections();

    for handle in hits {
        // find a matching connection for this handle+signal
        let pos = connections
            .iter()
            .position(|c| c.handle == *handle && c.signal == signal);
        if let Some(pos) = pos {
            let cb = &connections[pos].callback;
            // safety: we own connections, ui is free to mutate
            // we cast to fn pointer to call without holding a borrow on connections
            let cb_ptr: *const dyn Fn(&mut Ui) = cb.as_ref();
            unsafe { (*cb_ptr)(ui) };
            // consumed — stop bubbling
            ui.restore_connections(connections);
            return;
        }
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

    // hover
    let mut hover_hits: Vec<Handle<()>> = Vec::new();
    hit_test(ui, root, mx, my, &mut hover_hits);
    fire_signal(ui, &hover_hits, Signal::Hover);

    // click
    if mouse.left_just_released {
        let mut click_hits: Vec<Handle<()>> = Vec::new();
        hit_test(ui, root, mx, my, &mut click_hits);
        fire_signal(ui, &click_hits, Signal::Click);
    }
}
