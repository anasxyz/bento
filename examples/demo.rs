use rentex::App;
use rentex::widgets::WidgetManager;
use winit::keyboard::KeyCode;

fn main() {
    let app = App::new("RNTX demo", 800, 600);
    let mut widgets = WidgetManager::new();

    let btn = widgets.button("Click Me");
    widgets
        .get_mut(btn)
        .position(100.0, 100.0)
        .size(200.0, 50.0)
        .color([0.0, 0.0, 1.0, 1.0]);

    let mut counter = 0;

    app.run(widgets, move |widgets, mouse, input| {
        if input.just_pressed(KeyCode::Space) {
            counter += 1;
            widgets.get_mut(btn).text = format!("Clicked: {}", counter);
        }
    });
}
