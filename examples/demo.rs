#![allow(dead_code, unused)]
use winit::keyboard::KeyCode;
use bento::{App, BentoApp, Color, Ctx, Height, Width, Rect, Text, Button};

struct Demo {
    counter: i32,
    sidebar_open: bool,
    selected_color: Color,
}

impl BentoApp for Demo {
    fn once(&mut self, ctx: &mut Ctx) {
        let font = ctx.ui.fonts.default();

        ctx.ui.rect(
            "header",
            0.0,
            0.0,
            Width::Full,
            Height::Fixed(50.0),
            Color::rgb(0.12, 0.14, 0.18),
            Color::TRANSPARENT,
            0.0,
        );

        ctx.ui.text(
            "title",
            "Bento UI Demo",
            font,
            20.0,
            15.0,
            Color::rgb(0.9, 0.92, 0.95),
        );

        ctx.ui.rect(
            "content",
            0.0,
            50.0,
            Width::Full,
            Height::Percent(1.0),
            Color::rgb(0.09, 0.10, 0.12),
            Color::TRANSPARENT,
            0.0,
        );

        ctx.ui.rect(
            "counter_box",
            20.0,
            70.0,
            Width::Fixed(300.0),
            Height::Fixed(100.0),
            Color::rgb(0.15, 0.17, 0.22),
            Color::rgb(0.25, 0.28, 0.35),
            2.0,
        );

        ctx.ui.text(
            "counter_label",
            "Counter:",
            font,
            30.0,
            80.0,
            Color::rgb(0.65, 0.68, 0.75),
        );

        ctx.ui.text(
            "counter_value",
            "0",
            font,
            30.0,
            110.0,
            Color::rgb(0.35, 0.75, 0.95),
        );

        ctx.ui.button("btn_increment", "+ Increment", 20.0, 190.0);
        ctx.ui.button("btn_decrement", "- Decrement", 150.0, 190.0);
        ctx.ui.button("btn_reset", "Reset", 280.0, 190.0);

        ctx.ui.text(
            "color_label",
            "Select Background Color:",
            font,
            20.0,
            250.0,
            Color::rgb(0.65, 0.68, 0.75),
        );

        ctx.ui.button("btn_red", "Red", 20.0, 280.0);
        ctx.ui.button("btn_green", "Green", 90.0, 280.0);
        ctx.ui.button("btn_blue", "Blue", 170.0, 280.0);

        ctx.ui.button("btn_sidebar", "Toggle Sidebar", 20.0, 340.0);

        ctx.ui.text(
            "info",
            "Press ESC to exit",
            font,
            20.0,
            420.0,
            Color::rgb(0.45, 0.48, 0.55),
        );
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.input.keys_just_pressed.contains(&KeyCode::Escape) {
            ctx.exit();
        }

        if ctx.is_clicked("btn_increment") {
            self.counter += 1;
            if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                text.text = self.counter.to_string();
                ctx.mark_dirty();
            }
        }

        if ctx.is_clicked("btn_decrement") {
            self.counter -= 1;
            if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                text.text = self.counter.to_string();
                ctx.mark_dirty();
            }
        }

        if ctx.is_clicked("btn_reset") {
            self.counter = 0;
            if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                text.text = "0".to_string();
                ctx.mark_dirty();
            }
        }

        if ctx.is_clicked("btn_red") {
            self.selected_color = Color::rgb(0.35, 0.15, 0.18);
            if let Some(rect) = ctx.ui.get_mut::<Rect>("counter_box") {
                rect.color = self.selected_color;
                ctx.mark_dirty();
            }
        }

        if ctx.is_clicked("btn_green") {
            self.selected_color = Color::rgb(0.15, 0.30, 0.20);
            if let Some(rect) = ctx.ui.get_mut::<Rect>("counter_box") {
                rect.color = self.selected_color;
                ctx.mark_dirty();
            }
        }

        if ctx.is_clicked("btn_blue") {
            self.selected_color = Color::rgb(0.15, 0.22, 0.35);
            if let Some(rect) = ctx.ui.get_mut::<Rect>("counter_box") {
                rect.color = self.selected_color;
                ctx.mark_dirty();
            }
        }

        if ctx.is_clicked("btn_sidebar") {
            if !self.sidebar_open {
                if !ctx.ui.exists::<Rect>("sidebar") {
                    ctx.ui.rect(
                        "sidebar",
                        0.0,
                        50.0,
                        Width::Fixed(250.0),
                        Height::Percent(1.0),
                        Color::rgb(0.13, 0.15, 0.19),
                        Color::rgb(0.20, 0.23, 0.28),
                        2.0,
                    );

                    let font = ctx.ui.fonts.default();
                    ctx.ui.text(
                        "sidebar_title",
                        "Sidebar Menu",
                        font,
                        20.0,
                        70.0,
                        Color::rgb(0.9, 0.92, 0.95),
                    );

                    ctx.ui.button("sidebar_btn1", "Option 1", 20.0, 110.0);
                    ctx.ui.button("sidebar_btn2", "Option 2", 20.0, 150.0);
                    ctx.ui.button("sidebar_btn3", "Option 3", 20.0, 190.0);
                } else {
                    ctx.ui.show("sidebar");
                    ctx.ui.show("sidebar_title");
                    ctx.ui.show("sidebar_btn1");
                    ctx.ui.show("sidebar_btn2");
                    ctx.ui.show("sidebar_btn3");
                }

                self.sidebar_open = true;

                let offset = 250.0;
                if let Some(content) = ctx.ui.get_mut::<Rect>("content") {
                    content.x = offset;
                }
                if let Some(box_) = ctx.ui.get_mut::<Rect>("counter_box") {
                    box_.x = 20.0 + offset;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("counter_label") {
                    text.x = 30.0 + offset;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                    text.x = 30.0 + offset;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_increment") {
                    btn.x = 20.0 + offset;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_decrement") {
                    btn.x = 150.0 + offset;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_reset") {
                    btn.x = 280.0 + offset;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("color_label") {
                    text.x = 20.0 + offset;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_red") {
                    btn.x = 20.0 + offset;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_green") {
                    btn.x = 90.0 + offset;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_blue") {
                    btn.x = 170.0 + offset;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_sidebar") {
                    btn.x = 20.0 + offset;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("info") {
                    text.x = 20.0 + offset;
                }
                ctx.mark_dirty();
            } else {
                ctx.ui.hide("sidebar");
                ctx.ui.hide("sidebar_title");
                ctx.ui.hide("sidebar_btn1");
                ctx.ui.hide("sidebar_btn2");
                ctx.ui.hide("sidebar_btn3");

                self.sidebar_open = false;

                if let Some(content) = ctx.ui.get_mut::<Rect>("content") {
                    content.x = 0.0;
                }
                if let Some(box_) = ctx.ui.get_mut::<Rect>("counter_box") {
                    box_.x = 20.0;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("counter_label") {
                    text.x = 30.0;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                    text.x = 30.0;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_increment") {
                    btn.x = 20.0;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_decrement") {
                    btn.x = 150.0;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_reset") {
                    btn.x = 280.0;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("color_label") {
                    text.x = 20.0;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_red") {
                    btn.x = 20.0;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_green") {
                    btn.x = 90.0;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_blue") {
                    btn.x = 170.0;
                }
                if let Some(btn) = ctx.ui.get_mut::<Button>("btn_sidebar") {
                    btn.x = 20.0;
                }
                if let Some(text) = ctx.ui.get_mut::<Text>("info") {
                    text.x = 20.0;
                }
                ctx.mark_dirty();
            }
        }

        if ctx.is_clicked("sidebar_btn1") {
            println!("Sidebar Option 1 clicked!");
        }
        if ctx.is_clicked("sidebar_btn2") {
            println!("Sidebar Option 2 clicked!");
        }
        if ctx.is_clicked("sidebar_btn3") {
            println!("Sidebar Option 3 clicked!");
        }

        if ctx.input.keys_just_pressed.contains(&KeyCode::ArrowUp) {
            self.counter += 1;
            if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                text.text = self.counter.to_string();
                ctx.mark_dirty();
            }
        }

        if ctx.input.keys_just_pressed.contains(&KeyCode::ArrowDown) {
            self.counter -= 1;
            if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                text.text = self.counter.to_string();
                ctx.mark_dirty();
            }
        }

        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyR) {
            self.counter = 0;
            if let Some(text) = ctx.ui.get_mut::<Text>("counter_value") {
                text.text = "0".to_string();
                ctx.mark_dirty();
            }
        }
    }
}

fn main() {
    App::new("Bento UI Demo - Interactive Example", 800, 600).run(Demo {
        counter: 0,
        sidebar_open: false,
        selected_color: Color::rgb(0.15, 0.17, 0.22),
    });
}
