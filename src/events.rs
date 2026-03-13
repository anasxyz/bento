use crate::element::handle::Handle;
use crate::mouse::MouseState;
use crate::ui::Ui;

// Find the deepest node under the mouse cursor.
// Returns a stack from deepest hit node up to root, for bubbling.
fn hit_test(
    ui: &Ui,
    handle: Handle<()>,
    mx: f32,
    my: f32,
    hits: &mut Vec<Handle<()>>,
) {
    let el = match ui.get_dyn(handle) {
        Some(e) => e,
        None => return,
    };

    let layout = el.layout();

    if !layout.visible {
        return;
    }

    let inside = mx >= layout.x
        && mx <= layout.x + layout.w
        && my >= layout.y
        && my <= layout.y + layout.h;

    if !inside {
        return;
    }

    // push self first (will be after children, so deepest child is at front)
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

    // fire on_hover from deepest hit upward, stop at first consumer
    for handle in &hover_hits {
        let has_hover = ui.get_dyn(*handle)
            .map(|e| e.callbacks().has_hover())
            .unwrap_or(false);
        if has_hover {
            // take the callback out to avoid borrow conflict
            let cb = ui.get_dyn_mut(*handle)
                .and_then(|e| e.callbacks_mut().on_hover.take());
            if let Some(cb) = cb {
                cb(ui);
                // put it back
                if let Some(e) = ui.get_dyn_mut(*handle) {
                    e.callbacks_mut().on_hover = Some(cb);
                }
            }
            break; // consumed
        }
    }

    // --- hover_end: nodes that were hovered last frame but not this frame ---
    // (requires tracking previous hover set — skip for now, add later)

    // --- click ---
    if mouse.left_just_released {
        let mut click_hits: Vec<Handle<()>> = Vec::new();
        hit_test(ui, root, mx, my, &mut click_hits);

        for handle in &click_hits {
            let has_click = ui.get_dyn(*handle)
                .map(|e| e.callbacks().has_click())
                .unwrap_or(false);
            if has_click {
                let cb = ui.get_dyn_mut(*handle)
                    .and_then(|e| e.callbacks_mut().on_click.take());
                if let Some(cb) = cb {
                    cb(ui);
                    if let Some(e) = ui.get_dyn_mut(*handle) {
                        e.callbacks_mut().on_click = Some(cb);
                    }
                }
                break; // consumed
            }
        }
    }
}
