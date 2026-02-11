#![allow(dead_code, unused)]

use bento::{App, BentoApp, Ctx, FontId, Fonts};
use winit::keyboard::KeyCode;

struct Demo {}

impl BentoApp for Demo {
    fn once(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("main_font", "JetBrainsMono Nerd Font", 14.0);

        ctx.rect(10.0, 10.0, 100.0, 50.0, [1.0, 0.0, 0.0, 1.0], [0.0; 4], 0.0);
        ctx.circle(200.0, 100.0, 25.0, [0.0, 1.0, 0.0, 1.0], [0.0; 4], 0.0);
        ctx.rounded_rect(
            10.0,
            100.0,
            100.0,
            50.0,
            10.0,
            [0.0, 0.0, 1.0, 1.0],
            [0.0; 4],
            0.0,
        );

        ctx.text(
            "Hello!",
            ctx.fonts.get_by_name("main_font").unwrap(),
            50.0,
            200.0,
        );
    }

    fn update(&mut self, ctx: &mut Ctx) {}
}

fn main() {
    App::new("bento", 440, 260).run(Demo {});
}
