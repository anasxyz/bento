#![allow(dead_code, unused)]
use winit::keyboard::KeyCode;

use bento::{App, BentoApp, Color, Ctx, FontId, Fonts, Rect, Text};

struct Demo {
    x: f32,
    menu_open: bool,
}

impl BentoApp for Demo {
    fn once(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("main_font", "JetBrainsMono Nerd Font", 14.0);

        ctx.rect(
            "rect1",
            10.0,
            10.0,
            100.0,
            50.0,
            Color::RED,
            Color::BLACK,
            0.0,
        );

        ctx.text("text1", "Hello World", font, 100.0, 10.0, Color::WHITE);

        ctx.button("button1", "Click Me", 10.0, 200.0);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.input.keys_just_pressed.contains(&KeyCode::Escape) {
            ctx.exit();
        }

        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyW) {
            ctx.rects[0].y -= 30.0;
            ctx.mark_dirty();
        }
        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyS) {
            ctx.rects[0].y += 30.0;
            ctx.mark_dirty();
        }
        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyA) {
            ctx.rects[0].x -= 30.0;
            ctx.mark_dirty();
        }
        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyD) {
            ctx.rects[0].x += 30.0;
            ctx.mark_dirty();
        }

        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyL) {
            self.menu_open = !self.menu_open;
            ctx.toggle("rect1");
            ctx.mark_dirty();
        }
    }
}

fn main() {
    App::new("bento", 440, 260).run(Demo { x: 0.0, menu_open: false });
}
