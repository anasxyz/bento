use crate::scene::{FontFamilyRange, ItalicRange, WeightRange};

pub struct TextMeasureRequest<'a> {
    pub text: &'a str,
    pub font_family: &'a str,
    pub size: f32,
    pub weight: u16,
    pub italic: bool,
    pub letter_spacing: f32,
    pub line_height: Option<f32>,
    pub max_width: Option<f32>,

    pub weight_ranges: &'a [WeightRange],
    pub italic_ranges: &'a [ItalicRange],
    pub font_family_ranges: &'a [FontFamilyRange],
}

#[derive(Clone)]
pub struct LineMetrics {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
}

#[derive(Clone)]
pub struct TextMeasureResult {
    pub width: f32,
    pub height: f32,
    pub line_count: usize,
    pub lines: Vec<LineMetrics>,
}

pub trait TextMeasurer {
    fn measure(&mut self, req: TextMeasureRequest) -> TextMeasureResult;
}
