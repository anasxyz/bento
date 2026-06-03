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
                let label2 = label.clone();
                let label3 = label.clone();
                group()
                    .direction(Direction::Row)
                    .gap(8.0)
                    .child(text(move || {
                        if done.get() {
                            format!("[x] {}", label)
                        } else {
                            format!("[ ] {}", label)
                        }
                    }))
                    .child(text(|| "toggle".to_string()).on(move |_: &Click| {
                        done.update(|v| !v);
                    }))
                    .child(text(|| "remove".to_string()).on(move |_: &Click| {
                        items.update(|mut v| {
                            v.retain(|l| l != &label3);
                            v
                        });
                    }))
            },
        )
        .child(text(|| "add item".to_string()).on(move |_: &Click| {
            items.update(|mut v| {
                let n = v.len() + 1;
                v.push(format!("item {}", n));
                v
            });
        }))
}

#[main]
fn main() {
    App::run(app());
}
