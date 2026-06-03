#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[derive(Clone, Debug)]
struct Todo {
    id: u32,
    text: String,
    done: bool,
}

#[component]
fn app() -> impl View {
    let todos = state(vec![
        Todo { id: 1, text: "buy milk".into(), done: false },
        Todo { id: 2, text: "say that again".into(), done: false },
        Todo { id: 3, text: "sonion".into(), done: false },
    ]);

    group()
        .child(each(
            todos,
            |todo| todo.id,
            move |todo| {
                group()
                    .child(text(move || format!("[{}] {}", 
                        if todo.done { "x" } else { " " }, 
                        todo.text
                    )))
                    .child(text(|| "done".to_string()).on(move |e: &Click| {
                        todos.update(|mut v| {
                            if let Some(t) = v.iter_mut().find(|t| t.id == todo.id) {
                                t.done = !t.done;
                            }
                            v
                        });
                    }))
            }
        ))
        .child(
            text(|| "add todo".to_string()).on(move |e: &Click| {
                todos.update(|mut v| {
                    let id = v.len() as u32 + 1;
                    v.push(Todo { id, text: format!("Todo {}", id), done: false });
                    v
                });
            })
        )
}

#[main]
fn main() {
    App::run(app());
}
