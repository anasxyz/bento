use bento::*;
fn main() {
    let mut ui = Ui::new();
    let btn = ui.add(rect().bg(rgb(100, 0, 0)).w(px(100.0)).h(px(40.0)));
    let root = ui.add(col().w(pct(100.0)).h(pct(100.0)));

    ui.append(root, btn);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |ui| {
        // mutate directly each frame
        let color = ui[btn].style.fill;
        // ui[btn].style.fill = rgb(0, 200, 0);
    });
}
