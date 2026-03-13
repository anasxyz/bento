use crate::color::Color;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::label::Label;
use crate::element::rect::Rect;
use crate::element::values::Size;
use crate::signals::Signal;
use crate::ui::Ui;

#[derive(Copy, Clone)]
pub struct Button {
    pub(crate) root: Handle<Rect>,
    pub(crate) label: Handle<Label>,
}

impl Button {
    pub fn new(ui: &mut Ui, text: &str) -> Self {
        let root = Rect::new(ui);
        let label = Label::new(ui, text);
        ui.append(root, label);

        let base = Color::rgb(70, 70, 200);
        let hover = base.lighten(0.08);
        let press = base.darken(0.08);

        ui[root].bg_color = base;
        ui[root].border_radius = Some(6.0);
        ui[root].layout_mut().padding = [8.0, 16.0, 8.0, 16.0];
        ui[label].text_color = Color::WHITE;
        ui[label].font_size = 16.0;

        ui.connect(root, Signal::Hover, move |ui| {
            ui[root].bg_color = hover;
        });
        ui.connect(root, Signal::HoverEnd, move |ui| {
            ui[root].bg_color = base;
        });
        ui.connect(root, Signal::Press, move |ui| {
            ui[root].bg_color = press;
        });
        ui.connect(root, Signal::Release, move |ui| {
            ui[root].bg_color = hover;
        });

        Self { root, label }
    }

    pub fn set_text(&self, ui: &mut Ui, text: &str) {
        ui[self.label].text = text.to_string();
    }

    pub fn text<'a>(&self, ui: &'a Ui) -> &'a str {
        &ui[self.label].text
    }

    pub fn set_color(&self, ui: &mut Ui, color: Color) {
        let hover = color.lighten(0.08);
        let press = color.darken(0.08);
        let root = self.root;

        ui[root].bg_color = color;

        ui.disconnect(root, Signal::Hover);
        ui.disconnect(root, Signal::HoverEnd);
        ui.disconnect(root, Signal::Press);
        ui.disconnect(root, Signal::Release);

        ui.connect(root, Signal::Hover, move |ui| {
            ui[root].bg_color = hover;
        });
        ui.connect(root, Signal::HoverEnd, move |ui| {
            ui[root].bg_color = color;
        });
        ui.connect(root, Signal::Press, move |ui| {
            ui[root].bg_color = press;
        });
        ui.connect(root, Signal::Release, move |ui| {
            ui[root].bg_color = hover;
        });
    }

    pub fn set_font_size(&self, ui: &mut Ui, size: f32) {
        ui[self.label].font_size = size;
    }

    pub fn set_width(&self, ui: &mut Ui, width: f32) {
        ui[self.root].layout_mut().width = Size::Fixed(width);
    }

    pub fn set_height(&self, ui: &mut Ui, height: f32) {
        ui[self.root].layout_mut().height = Size::Fixed(height);
    }

    pub fn set_border_radius(&self, ui: &mut Ui, radius: f32) {
        ui[self.root].border_radius = Some(radius);
    }

    pub fn set_padding(&self, ui: &mut Ui, padding: [f32; 4]) {
        ui[self.root].layout_mut().padding = padding;
    }

    // explicit handle access for append/connect
    pub fn handle(&self) -> Handle<Rect> {
        self.root
    }
}

impl From<Button> for Handle<Rect> {
    fn from(b: Button) -> Handle<Rect> {
        b.root
    }
}
