use bento_wgpu::{RectDraw, TextAlign, TextDraw};

use crate::{
    Click, FocusLost, Group, HoverEnter, HoverLeave, Layout, MouseDown, Size, Ui, Widget,
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
    pub options_group: WidgetHandle<Group>,

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
            options_group: WidgetHandle::invalid(),
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
        self.options_group = WidgetHandle::invalid();
        self.open = false;
    }

    fn build(ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<Dropdown>();

        let mut options = Group::new();
        options.layout = Layout::Column { gap: 0.0 };
        options.background = Some([0.2, 0.2, 0.2, 1.0]);
        options.width = Size::Fixed(120.0);
        options.visible = false;
        options.z = 100;

        let h = if let Some(d) = ui.get(handle) {
            d.h
        } else {
            32.0
        };
        options.y = h;

        let og = ui.add(options);

        if let Some(d) = ui.get_mut(handle) {
            d.options_group = og;
        }
        ui.append(handle, og);

        ui.listen(handle, move |_: &MouseDown, ui: &mut Ui| {
            let (open, og) = ui
                .get(handle)
                .map(|d| (d.open, d.options_group))
                .unwrap_or((false, WidgetHandle::invalid()));
            if !open {
                ui.set(og, |g| g.visible = true);
                ui.set(handle, |d| d.open = true);
            } else {
                ui.set(og, |g| g.visible = false);
                ui.set(handle, |d| d.open = false);
            }
        });

        ui.listen(handle, move |_: &FocusLost, ui: &mut Ui| {
            if let Some(og) = ui.get(handle).map(|d| d.options_group) {
                ui.set(og, |g| g.visible = false);
            }
            ui.set(handle, |d| d.open = false);
        });

        ui.listen(handle, move |_: &HoverEnter, ui: &mut Ui| {
            if let Some(d) = ui.get_mut(handle) {
                d.hovered = true;
            }
            ui.request_redraw();
        });

        ui.listen(handle, move |_: &HoverLeave, ui: &mut Ui| {
            if let Some(d) = ui.get_mut(handle) {
                d.hovered = false;
            }
            ui.request_redraw();
        });
    }

    fn render(&mut self, canvas: &mut Canvas) {
        self.screen_x = canvas.x;
        self.screen_y = canvas.y;

        canvas.draw_list.push_rect(RectDraw {
            x: canvas.x,
            y: canvas.y,
            w: self.w,
            h: self.h,
            color: [0.2, 0.2, 0.2, 1.0],
            radii: [0.0; 4],
            border_color: [0.3, 0.3, 0.3, 1.0],
            border_widths: [1.0; 4],
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            opacity: canvas.opacity,
            clip: canvas.clip,
            z: canvas.z,
        });

        canvas.draw_list.push_text(TextDraw {
            x: canvas.x + 8.0,
            y: canvas.y + (self.h - 14.0) / 2.0,
            w: self.w,
            h: self.h,
            text: self.label.clone(),
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            tab_width: 4,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: canvas.opacity,
            clip: canvas.clip,
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            z: canvas.z + 1,
            color_ranges: vec![],
            background_ranges: vec![],
            underline_ranges: vec![],
            strikethrough_ranges: vec![],
            weight_ranges: vec![],
            italic_ranges: vec![],
            font_family_ranges: vec![],
        });
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
