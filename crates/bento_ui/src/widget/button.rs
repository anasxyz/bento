use bento_shared::{RectNode, Scene, TextAlign, TextMeasureRequest, TextMeasurer, TextNode};

pub struct Button {
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
}

impl Button {
    pub fn new(label: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            label: label.to_string(),
            x,
            y,
            w,
            h,
            color: [0.2, 0.5, 1.0, 1.0],
        }
    }

    pub fn build(&self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        let t = std::time::Instant::now();

        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            size: 16.0,
            max_width: Some(self.w),
            font_family: "",
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        println!("measure took: {:?}", t.elapsed());

        let text_y = self.y + (self.h - result.height) / 2.0;

        let mut rect = RectNode::new(self.x, self.y, self.w, self.h);
        rect.color(self.color);
        scene.add_rect(rect);

        let mut text = TextNode::new(&self.label, self.x, text_y, 16.0);
        text.color([1.0, 1.0, 1.0, 1.0]);
        text.max_width(self.w);
        text.align(TextAlign::Center);
        scene.add_text(text);
    }
}
