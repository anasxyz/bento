#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let container = ui.add(Container::new(100.0, 100.0, 400.0, 300.0));
    let rect = ui.add(Rect::new(10.0, 10.0, 100.0, 50.0));
    ui.append(container, rect);

    println!(
        "container x {} y {} w {} h {}",
        ui.get_mut(container).unwrap().x(),
        ui.get_mut(container).unwrap().y(),
        ui.get_mut(container).unwrap().w(),
        ui.get_mut(container).unwrap().h()
    );

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        move |ui: &mut Ui| {
            ui.get_mut(container).unwrap().set_offset(100.0, 0.0);

            println!(
                "container x {} y {} w {} h {}",
                ui.get_mut(container).unwrap().x(),
                ui.get_mut(container).unwrap().y(),
                ui.get_mut(container).unwrap().w(),
                ui.get_mut(container).unwrap().h()
            );
        }
    });

    ui.listen_global(move |e: &KeyPress, ui| {
        match e.key {
            Key::D => {
                ui.print_tree();
            }
            _ => {}
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
