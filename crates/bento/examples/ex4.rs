#![allow(dead_code)]
#![allow(unused)]

use bento::*;

#[component]
fn app() -> impl View {
    let items = state(vec![
        "item 1".to_string(),
        "item 2".to_string(),
        "item 3".to_string(),
    ]);
    group()
        .direction(Direction::Column)
        .gap(8.0)
        .padding(16.0)
        .each(
            items,
            |label| label.clone(),
            move |label| {
                let done = state(false);
                let label3 = label.clone();
                group()
                    .direction(Direction::Row)
                    .gap(8.0)
                    .child(
                        rect(|| [0.2, 0.4, 0.8, 1.0])
                            .width(Size::Fixed(100.0))
                            .height(Size::Fixed(20.0)),
                    )
                    .child(
                        rect(move || {
                            if done.get() {
                                [0.2, 0.8, 0.2, 1.0]
                            } else {
                                [0.5, 0.5, 0.2, 1.0]
                            }
                        })
                        .width(Size::Fixed(50.0))
                        .height(Size::Fixed(20.0))
                        .on(move |_: &Click| {
                            done.update(|v| !v);
                        }),
                    )
                    .child(
                        rect(|| [0.8, 0.2, 0.2, 1.0])
                            .width(Size::Fixed(50.0))
                            .height(Size::Fixed(20.0))
                            .on(move |_: &Click| {
                                items.update(|mut v| {
                                    v.retain(|l| l != &label3);
                                    v
                                });
                            }),
                    )
            },
        )
        .child(
            rect(|| [0.2, 0.8, 0.2, 1.0])
                .width(Size::Fixed(80.0))
                .height(Size::Fixed(20.0))
                .on(move |_: &Click| {
                    items.update(|mut v| {
                        let n = v.len() + 1;
                        v.push(format!("item {}", n));
                        v
                    });
                }),
        )
}
#[main]
fn main() {
    App::run(app());
}
