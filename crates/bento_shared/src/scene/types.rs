#[derive(Clone, PartialEq, Debug)]
pub struct DecorationRange {
    pub start: usize,
    pub end: usize,
    pub color: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct WeightRange {
    pub start: usize,
    pub end: usize,
    pub weight: u16,
}

#[derive(Clone, Debug)]
pub struct ItalicRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub struct FontFamilyRange {
    pub start: usize,
    pub end: usize,
    pub font_family: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ColorRange {
    pub start: usize,
    pub end: usize,
    pub color: [f32; 4],
}

#[derive(Clone, PartialEq, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
