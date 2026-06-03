#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[derive(Clone, Debug)]
struct Item {
    id: u32,
    text: String,
}

#[component]
fn app() -> impl View {
    let items = state(vec![
        Item { id: 1, text: "item 1".into() },
        Item { id: 2, text: "item 2".into() },
        Item { id: 3, text: "item 3".into() },
    ]);

    group()
        .direction(Direction::Column)
        .gap(8.0)
        .padding(16.0)
        .each(items, |item| item.id, move |item| {
            group()
                .direction(Direction::Row)
                .gap(8.0)
                .child(text(move || item.text.clone()))
                .child(text(|| "remove".to_string()).on(move |e: &Click| {
                    items.update(|mut v| {
                        v.retain(|i| i.id != item.id);
                        v
                    });
                }))
        })
        .child(
            text(|| "add item".to_string()).on(move |e: &Click| {
                items.update(|mut v| {
                    let id = v.len() as u32 + 1;
                    v.push(Item { id, text: format!("item {}", id) });
                    v
                });
            })
        )
}

#[main]
fn main() {
    App::run(app());
}
