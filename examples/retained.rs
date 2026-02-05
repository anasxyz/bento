// examples/retained_mode.rs
use rentex::App;

fn main() {
    let app = App::new("Retained Mode Example", 800, 600);

    // The closure is only called when the scene is dirty
    // Initially it runs once to build the scene
    app.run(|canvas| {
        // Clear and rebuild the scene
        canvas.scene.clear();

        // Draw a red rectangle
        canvas
            .scene
            .rect(50.0, 50.0, 200.0, 100.0, [1.0, 0.0, 0.0, 1.0]);

        // Draw a green circle
        canvas
            .scene
            .circle(400.0, 150.0, 50.0, [0.0, 1.0, 0.0, 1.0]);

        // Draw a blue rounded button
        canvas
            .scene
            .rounded_rect(50.0, 200.0, 200.0, 60.0, 8.0, [0.2, 0.4, 0.8, 1.0]);

        // Add some text
        canvas.scene.text("Hello, Retained Mode!", 300.0, 220.0);

        canvas.scene.button(500.0, 500.0, 100.0, 50.0, "Click Me");
    });
}
