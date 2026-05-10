use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let text = ui.add(Text::new("#include <bento.h>\n\nint main(void) {\n    printf(\"Hello World\\n\"); \n}", 10.0, 10.0, 14.0));
    ui.with(text, |t| {
        t.set_font_family("JetBrainsMono Nerd Font".to_string());
        t.set_color([1.0, 1.0, 1.0, 1.0]);
        t.set_opacity(0.0);
        t.animate_opacity(1.0, 0.8, Easing::EaseOut, LoopMode::Once);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
