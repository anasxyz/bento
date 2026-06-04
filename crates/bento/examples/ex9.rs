#![allow(dead_code)]
#![allow(unused)]
use bento::*;
use taffy::prelude::*;

// example 1 tooltip that follows the mouse
#[component]
fn tooltip_demo() -> impl View {
    let mouse_x = state(0.0f32);
    let mouse_y = state(0.0f32);
    let visible = state(false);

    group()
        .child(text(|| "click anywhere to toggle tooltip".into()))
        .child(
            rect(|| [0.1, 0.1, 0.1, 0.95])
                .w(px(120.0))
                .h(px(32.0))
                .position(Position::Absolute)
                .inset_left(move || px(mouse_x.get() + 10.0))
                .inset_top(move || px(mouse_y.get() + 10.0)),
        )
        .w(fill())
        .h(fill())
        .on(move |e: &MouseMove| {
            mouse_x.set(e.x);
            mouse_y.set(e.y);
        })
        .on(move |_: &Click| visible.update(|v| !v))
}

// example 2 draggable box
#[component]
fn draggable_demo() -> impl View {
    let x = state(100.0f32);
    let y = state(100.0f32);
    let offset_x = state(0.0f32);
    let offset_y = state(0.0f32);
    let dragging = state(false);

    group()
        .w(fill())
        .h(fill())
        .child(
            rect(|| [0.2, 0.5, 1.0, 1.0])
                .w(px(80.0))
                .h(px(80.0))
                .position(Position::Absolute)
                .inset_left(move || px(x.get()))
                .inset_top(move || px(y.get()))
                .on(move |e: &MouseDown| {
                    offset_x.set(e.x - x.get());
                    offset_y.set(e.y - y.get());
                    dragging.set(true);
                })
                .on(move |_: &MouseUp| dragging.set(false))
                .on(move |e: &MouseMove| {
                    if dragging.get() {
                        x.set(e.x - offset_x.get());
                        y.set(e.y - offset_y.get());
                    }
                })
        )
}

// example 3 multiple sliders driving a color
#[component]
fn sliders_demo() -> impl View {
    let r = state(1.0f32);
    let g = state(0.5f32);
    let b = state(0.2f32);

    group()
        .direction(col())
        .w(fill())
        .h(fill())
        .p(px(24.0))
        .gap(px(16.0))
        .child(
            rect(move || [r.get(), g.get(), b.get(), 1.0])
                .w(fill())
                .h(px(80.0)),
        )
        .child(slider(r, 0.0, 1.0))
        .child(slider(g, 0.0, 1.0))
        .child(slider(b, 0.0, 1.0))
}

#[component("MySlider")]
pub fn slider(value: Signal<f32>, min: f32, max: f32) -> impl View {
    let dragging = state(false);
    let track_ref = node_ref();

    let thumb_pos = derived(move || ((value.get() - min) / (max - min)).clamp(0.0, 1.0));

    let track = rect(|| [0.3, 0.3, 0.3, 1.0])
        .w(fill())
        .h(px(4.0))
        .node_ref(track_ref)
        .on(move |e: &Click| {
            if let Some(id) = track_ref.get() {
                let (tx, _, tw, _) = get_rect(id);
                let t = ((e.x - tx) / tw).clamp(0.0, 1.0);
                value.set(min + t * (max - min));
            }
        });

    let thumb = rect(|| [1.0, 1.0, 1.0, 1.0])
        .w(px(16.0))
        .h(px(16.0))
        .position(Position::Absolute)
        .inset_top(|| px(8.0))
        .inset_left(move || pct(thumb_pos.get()))
        .m_left(px(-8.0))
        .on(move |_: &MouseDown| dragging.set(true))
        .on(move |_: &MouseUp| dragging.set(false))
        .on(move |e: &MouseMove| {
            if dragging.get() {
                if let Some(id) = track_ref.get() {
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

#[component("App")]
fn app() -> impl View {
    sliders_demo()
}

#[main]
fn main() {
    App::run(app());
}
