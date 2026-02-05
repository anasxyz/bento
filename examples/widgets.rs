// examples/consistent_api_demo.rs
use rentex::widgets::*;

fn main() {
    let app = rentex::App::new("Consistent API Demo", 800, 600);
    
    // === Everything is created the same way! ===
    
    // Shapes
    let rect = Rect::new(50.0, 50.0, 100.0, 80.0)
        .color([1.0, 0.0, 0.0, 1.0])
        .outline([0.0, 0.0, 0.0, 1.0], 2.0);
    
    let circle = Circle::new(250.0, 90.0, 40.0)
        .color([0.0, 1.0, 0.0, 1.0])
        .outline([0.0, 0.5, 0.0, 1.0], 2.0);
    
    let rounded = RoundedRect::new(350.0, 50.0, 120.0, 80.0, 10.0)
        .color([0.0, 0.0, 1.0, 1.0])
        .outline([0.0, 0.0, 0.5, 1.0], 2.0);
    
    let label = Text::new("Hello, consistent API!", 50.0, 20.0);
    
    // Widgets
    let mut button = Button::new(50.0, 150.0, 150.0, 50.0, "Click Me!");
    let mut checkbox = Checkbox::new(50.0, 220.0, "Enable feature");
    let mut slider = Slider::new(50.0, 270.0, 300.0);
    let card = Card::new(400.0, 150.0, 350.0, 300.0);
    
    app.run(move |canvas| {
        let m = canvas.mouse;
        
        // Update widgets
        button.is_hovered = button.contains(m.x, m.y);
        button.is_pressed = button.is_hovered && m.left_pressed;
        
        if checkbox.contains(m.x, m.y) && m.left_just_pressed {
            checkbox.toggle();
        }
        
        if slider.contains_handle(m.x, m.y) && m.left_just_pressed {
            slider.is_dragging = true;
        }
        if m.left_just_released {
            slider.is_dragging = false;
        }
        if slider.is_dragging {
            slider.set_value_from_x(m.x);
        }
        
        // === Draw everything the same way! ===
        
        label.draw(canvas);
        
        rect.draw(canvas);
        circle.draw(canvas);
        rounded.draw(canvas);
        
        card.draw(canvas);
        button.draw(canvas);
        checkbox.draw(canvas);
        slider.draw(canvas);
        
        // Text inside card
        Text::new("Consistent API:", 420.0, 170.0).draw(canvas);
        Text::new("Rect::new(...).color(...).draw(canvas)", 420.0, 200.0).draw(canvas);
        Text::new("Circle::new(...).color(...).draw(canvas)", 420.0, 225.0).draw(canvas);
        Text::new("Text::new(...).draw(canvas)", 420.0, 250.0).draw(canvas);
        Text::new("Button::new(...).draw(canvas)", 420.0, 275.0).draw(canvas);
        
        Text::new("Everything works the same!", 420.0, 320.0).draw(canvas);
    });
}
