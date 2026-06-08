use bento::*;
use taffy::prelude::*;

#[component]
pub fn slider(value: Signal<f32>, min: f32, max: f32) -> impl View {
    let dragging = state(false);
    let track_ref = node_ref();
    let thumb_pos = derived(move || ((value.get() - min) / (max - min)).clamp(0.0, 1.0));

    let track = group()
        .direction(row())
        .align_items(AlignItems::Center)
        .w(fill())
        .h(px(32.0))
        .node_ref(track_ref)
        .child(
            rect()
                .color([0.3, 0.3, 0.3, 1.0])
                .w(fill())
                .h(px(4.0))
        );

    let thumb = rect()
        .color([1.0, 1.0, 1.0, 1.0])
        .w(px(16.0))
        .h(px(16.0))
        .position(Position::Absolute)
        .inset_top(px(8.0))
        .inset_left(move || pct(thumb_pos.get()))
        .m_left(px(-8.0));

    group()
        .direction(row())
        .align_items(AlignItems::Center)
        .w(fill())
        .h(px(32.0))
        .on(move |e: &MouseDown| {
            if let Some(id) = track_ref.get() {
                let (tx, _, tw, _) = get_rect(id);
                let t = ((e.x - tx) / tw).clamp(0.0, 1.0);
                value.set(min + t * (max - min));
                dragging.set(true);
            }
        })
        .on(move |_: &MouseUp| dragging.set(false))
        .on(move |e: &MouseMove| {
            if dragging.get() {
                if let Some(id) = track_ref.get() {
                    let (tx, _, tw, _) = get_rect(id);
                    let t = ((e.x - tx) / tw).clamp(0.0, 1.0);
                    value.set(min + t * (max - min));
                }
            }
        })
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
        .p(px(24.0))
        .gap(px(8.0))
        .child(text(move || format!("value: {:.2}", value.get())))
        .child(slider(value, 0.0, 1.0))
}

#[main]
fn main() {
    App::run(app());
}
