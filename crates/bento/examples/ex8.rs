use bento::*;
use taffy::prelude::*;

#[component]
pub fn slider(value: Signal<f32>, min: f32, max: f32) -> impl View {
    let dragging = state(false);
    let track_ref = node_ref();

    let track_ref_for_move = track_ref;
    let track_ref_for_click = track_ref;
    let track_ref_for_left = track_ref;

    let thumb_pos = derived(move || ((value.get() - min) / (max - min)).clamp(0.0, 1.0));

    let track = rect(|| [0.3, 0.3, 0.3, 1.0])
        .w(fill())
        .h(px(4.0))
        .node_ref(track_ref)
        .on(move |e: &Click| {
            if let Some(id) = track_ref_for_click.get() {
                let (tx, _, tw, _) = get_rect(id);
                let t = ((e.x - tx) / tw).clamp(0.0, 1.0);
                value.set(min + t * (max - min));
            }
        });

    let thumb = rect(|| [1.0, 1.0, 1.0, 1.0])
        .w(px(16.0))
        .h(px(16.0))
        .position(Position::Absolute)
        .inset_top(|| 8.0)
        .inset_left(move || {
            if let Some(id) = track_ref_for_left.get() {
                let (tx, _, tw, _) = get_rect(id);
                // tx is absolute, but inset_left is relative to parent
                // so subtract parent's x
                thumb_pos.get() * tw - 8.0
            } else {
                0.0
            }
        })
        .on(move |_: &MouseDown| dragging.set(true))
        .on(move |_: &MouseUp| dragging.set(false))
        .on(move |e: &MouseMove| {
            if dragging.get() {
                if let Some(id) = track_ref_for_move.get() {
                    let (tx, _, tw, _) = get_rect(id);
                    let t = ((e.x - tx) / tw).clamp(0.0, 1.0);
                    value.set(min + t * (max - min));
                }
            }
        });

    group()
        .direction(row())
        .align_items(AlignItems::Center)
        .w(fill())
        .h(px(32.0))
        .child(track)
        .child(thumb)
}

#[component]
fn app() -> impl View {
    let value = state(0.5f32);

    group()
        .direction(col())
        .w(fill())
        .h(fill())
        .p(24.0)
        .gap(8.0)
        .child(text(move || format!("value: {:.2}", value.get())))
        .child(slider(value, 0.0, 1.0))
}

#[main]
fn main() {
    App::run(app());
}
