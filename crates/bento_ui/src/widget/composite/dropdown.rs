use bento_wgpu::{RectDraw, TextAlign, TextDraw};

use crate::{
    Button, Click, FocusLost, Group, HoverEnter, HoverLeave, Layout, MouseDown, Size, Ui, Widget,
    widget::{Canvas, WidgetHandle},
};

pub struct Dropdown {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub width: Size,
    pub height: Size,
    pub z: i32,
    pub label: String,
    pub options: WidgetHandle<Group>,
    pub button: WidgetHandle<Button>,

    screen_x: f32,
    screen_y: f32,
    open: bool,
    hovered: bool,
}

impl Dropdown {
    pub fn new(label: &str) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 120.0,
            h: 32.0,
            width: Size::Fixed(120.0),
            height: Size::Fixed(32.0),
            z: 0,
            label: label.to_string(),
            options: WidgetHandle::invalid(),
            button: WidgetHandle::invalid(),
            screen_x: 0.0,
            screen_y: 0.0,
            open: false,
            hovered: false,
        }
    }
}

impl Widget for Dropdown {
    fn name(&self) -> &str {
        "Dropdown"
    }

    fn init(&mut self) {
        self.w = 120.0;
        self.h = 32.0;
        self.options = WidgetHandle::invalid();
        self.open = false;
    }

    fn build(ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<Dropdown>();

        let label = ui.get(handle).map(|d| d.label.clone()).unwrap_or_default();
        let btn = ui.add(Button::new(&label));
        if let Some(d) = ui.get_mut(handle) {
            d.button = btn;
        }
        ui.append(handle, btn);

        // create options group
        let mut options = Group::new();
        options.layout = Layout::Column { gap: 0.0 };
        options.background = Some([0.2, 0.2, 0.2, 1.0]);
        options.width = Size::Fixed(200.0);
        options.visible = false;
        options.z = 100;
        options.y = 32.0;
        let og = ui.add(options);
        if let Some(d) = ui.get_mut(handle) {
            d.options = og;
        }
        ui.append(handle, og);

        // toggle on button click
        ui.listen(btn, move |_: &Click, ui: &mut Ui| {
            println!("click");
            let (open, og) = ui
                .get(handle)
                .map(|d| (d.open, d.options))
                .unwrap_or((false, WidgetHandle::invalid()));
            if !open {
                ui.set(og, |g| g.visible = true);
                ui.set(handle, |d| d.open = true);
            } else {
                ui.set(og, |g| g.visible = false);
                ui.set(handle, |d| d.open = false);
            }
        });

        ui.listen(btn, move |_: &FocusLost, ui: &mut Ui| {
            let og = ui
                .get(handle)
                .map(|d| d.options)
                .unwrap_or(WidgetHandle::invalid());
            ui.set(og, |g| g.visible = false);
            ui.set(handle, |d| d.open = false);
        });
    }

    fn update(ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<Dropdown>();
        let (label, btn, w, h) = ui
            .get(handle)
            .map(|d| (d.label.clone(), d.button, d.w, d.h))
            .unwrap_or_default();
        if btn.is_valid() {
            ui.set(btn, |b| {
                b.label_text = label;
                b.width = Size::Fixed(w);
                b.height = Size::Fixed(h);
            });
        }
    }

    fn render(&mut self, canvas: &mut Canvas) {
        self.screen_x = canvas.x;
        self.screen_y = canvas.y;
    }

    fn size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    fn set_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }
    fn width_sizing(&self) -> &Size {
        &self.width
    }
    fn height_sizing(&self) -> &Size {
        &self.height
    }
    fn z(&self) -> i32 {
        self.z
    }
}
