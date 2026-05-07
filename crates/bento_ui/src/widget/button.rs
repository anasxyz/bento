use crate::widget::{Widget, rect::Rect, text::Text};
use bento_shared::{TextMeasureRequest, TextMeasurer, scene::Scene};

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub color: [f32; 4],
    pub label: String,
    pub font_size: f32,
    pub font_family: String,
    pub font_weight: u16,
    pub italic: bool,
    pub letter_spacing: f32,
    pub text_color: [f32; 4],
    pub padding_x: f32,
    pub padding_y: f32,
    pub max_width: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub border_color: [f32; 4],
    pub border_width: f32,
    pub radius: f32,
    pub opacity: f32,
    // computed
    pub w: f32,
    pub h: f32,
    background: Rect,
    label_text: Text,
}

impl Button {
    pub fn new(label: &str, x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            color: [0.2, 0.5, 1.0, 1.0],
            label: label.to_string(),
            font_size: 16.0,
            font_family: String::new(),
            font_weight: 400,
            italic: false,
            letter_spacing: 0.0,
            text_color: [1.0, 1.0, 1.0, 1.0],
            padding_x: 16.0,
            padding_y: 8.0,
            max_width: None,
            min_width: None,
            min_height: None,
            border_color: [0.0; 4],
            border_width: 0.0,
            radius: 0.0,
            opacity: 1.0,
            w: 0.0,
            h: 0.0,
            background: Rect::new(0.0, 0.0, 0.0, 0.0),
            label_text: Text::new(label, 0.0, 0.0, 16.0),
        }
    }
}

impl Widget for Button {
    fn build(&mut self, scene: &mut Scene) {
        self.background.build(scene);
        self.label_text.build(scene);
    }

    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        let max_text_width = self.max_width.map(|mw| mw - self.padding_x * 2.0);

        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            size: self.font_size,
            max_width: max_text_width,
            font_family: &self.font_family,
            weight: self.font_weight,
            italic: self.italic,
            letter_spacing: self.letter_spacing,
            line_height: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        self.w = (result.width + self.padding_x * 2.0)
            .max(self.min_width.unwrap_or(0.0))
            .ceil();
        self.h = (result.height + self.padding_y * 2.0)
            .max(self.min_height.unwrap_or(0.0))
            .ceil();

        if let Some(mw) = self.max_width {
            self.w = self.w.min(mw);
        }

        self.background.x = self.x;
        self.background.y = self.y;
        self.background.w = self.w;
        self.background.h = self.h;
        self.background.color = self.color;
        self.background.border_color = self.border_color;
        self.background.border_widths = [self.border_width; 4];
        self.background.radii = [self.radius; 4];
        self.background.opacity = self.opacity;
        self.background.update(scene, measurer);

        let text_x = self.x + self.padding_x;
        let text_y = self.y + (self.h - result.height) / 2.0;

        self.label_text.x = text_x;
        self.label_text.y = text_y;
        self.label_text.text = self.label.clone();
        self.label_text.size = self.font_size;
        self.label_text.color = self.text_color;
        self.label_text.font_family = self.font_family.clone();
        self.label_text.weight = self.font_weight;
        self.label_text.italic = self.italic;
        self.label_text.letter_spacing = self.letter_spacing;
        self.label_text.max_width = max_text_width;
        self.label_text.opacity = self.opacity;
        self.label_text.update(scene, measurer);
    }
}
