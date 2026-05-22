#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut group_a = Group::new();
    group_a.layout = Layout::Row { gap: 8.0 };
    group_a.z = 0;
    let group_a = ui.add(group_a);

    let mut group_b = Group::new();
    group_b.layout = Layout::Row { gap: 8.0 };
    group_b.z = 10;
    let group_b = ui.add(group_b);

    let btn1 = ui.add(Button::new("Group A - 1"));
    ui.get_mut(btn1).unwrap().set_color([1.0, 0.0, 0.0, 1.0]);
    let btn2 = ui.add(Button::new("Group A - 2"));
    let btn3 = ui.add(Button::new("Group B - 1"));
    let btn4 = ui.add(Button::new("Group B - 2"));

    ui.append(group_a, btn1);
    ui.append(group_a, btn2);
    ui.append(group_b, btn3);
    ui.append(group_b, btn4);

    ui.get_mut(group_b).unwrap().set_x(50.0);
    ui.get_mut(group_b).unwrap().set_y(20.0);

    println!("group_a z: {}", ui.get(group_a).unwrap().z);
    println!("group_b z: {}", ui.get(group_b).unwrap().z);
    println!("btn1 z: {}", ui.get(btn1).unwrap().z);
    println!("btn2 z: {}", ui.get(btn2).unwrap().z);
    println!("btn3 z: {}", ui.get(btn3).unwrap().z);
    println!("btn4 z: {}", ui.get(btn4).unwrap().z);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
