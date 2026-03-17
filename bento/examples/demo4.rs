use bento::*;
use bento_derive::Element;

#[derive(Element)]
struct ProgressBar {
    base: Base,
    value: f32,
}

impl ProgressBar {
    fn new(value: f32) -> Self {
        Self {
            base: Base::new(),
            value,
        }
    }

    fn set_value(&mut self, value: f32) -> &mut Self {
        self.value = value;
        self.base.dirty = true;
        self
    }
}

impl Element for ProgressBar {
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
        let l = &self.base.layout;
        let filled = l.w * self.value.clamp(0.0, 1.0);
        vec![
            DrawCall::Rect {
                x: l.x,
                y: l.y,
                w: l.w,
                h: l.h,
                color: [0.2, 0.2, 0.2, opacity],
                radius: 4.0,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z,
            },
            DrawCall::Rect {
                x: l.x,
                y: l.y,
                w: filled,
                h: l.h,
                color: [0.3, 0.7, 0.4, opacity],
                radius: 4.0,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z + 1,
            },
        ]
    }
}

fn main() {
    let mut ui = Ui::new();

    let bar = ui.add(ProgressBar::new(0.65));
    ui.get_mut(bar)
        .unwrap()
        .set_width(Size::Fixed(300.0))
        .set_height(Size::Fixed(12.0));

    ui.set_root(bar);

    ui.connect(ui.global(), move |ui, event| {
        if let Event::MouseMove { x, .. } = event {
            if let Some(bar) = ui.get_mut(bar) {
                bar.set_value((x / 300.0).clamp(0.0, 1.0));
            }
        }
    });

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
