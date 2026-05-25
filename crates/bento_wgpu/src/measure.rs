use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping, Style as CStyle, Weight};
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct MeasureKey {
    text: String,
    size: u32,
    max_width: u32,
    weight: u16,
    italic: bool,
    font_family: String,
    letter_spacing: u32,
    line_height: u32,
    weight_ranges: Vec<(usize, usize, u16)>,
    italic_ranges: Vec<(usize, usize)>,
    font_family_ranges: Vec<(usize, usize, String)>,
}

impl MeasureKey {
    fn from_request(req: &TextMeasureRequest<'_>) -> Self {
        Self {
            text: req.text.to_string(),
            size: req.size.to_bits(),
            max_width: req.max_width.unwrap_or(f32::MAX).to_bits(),
            weight: req.weight,
            italic: req.italic,
            font_family: req.font_family.to_string(),
            letter_spacing: req.letter_spacing.to_bits(),
            line_height: req.line_height.unwrap_or(0.0).to_bits(),
            weight_ranges: req
                .weight_ranges
                .iter()
                .map(|r| (r.start, r.end, r.weight))
                .collect(),
            italic_ranges: req.italic_ranges.iter().map(|r| (r.start, r.end)).collect(),
            font_family_ranges: req
                .font_family_ranges
                .iter()
                .map(|r| (r.start, r.end, r.font_family.clone()))
                .collect(),
        }
    }
}

struct MeasureCache {
    cache: HashMap<MeasureKey, (TextMeasureResult, Buffer)>,
}

impl MeasureCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

pub struct TextMeasurer {
    pub font_system: cosmic_text::FontSystem,
    cache: MeasureCache,
    reuse_buffers: HashMap<u64, Buffer>,
}

impl TextMeasurer {
    pub fn new() -> Self {
        let font_system = cosmic_text::FontSystem::new();
        Self {
            font_system,
            cache: MeasureCache::new(),
            reuse_buffers: HashMap::new(),
        }
    }

    pub fn trim_shape_cache(&mut self) {
        self.font_system.shape_run_cache.trim(2);
    }

    pub fn measure(&mut self, req: TextMeasureRequest<'_>) -> TextMeasureResult {
        let t = std::time::Instant::now();
        let key = MeasureKey::from_request(&req);
        if let Some((result, _)) = self.cache.cache.get(&key) {
            return result.clone();
        }

        let line_height = req.line_height.unwrap_or(req.size * 1.4);
        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(req.size, line_height));
        buffer.set_size(&mut self.font_system, req.max_width, None);

        let node_attrs = {
            let mut a = Attrs::new().weight(Weight(req.weight));
            if req.italic {
                a = a.style(CStyle::Italic);
            }
            if !req.font_family.is_empty() {
                a = a.family(Family::Name(req.font_family));
            }
            if req.letter_spacing != 0.0 {
                a = a.letter_spacing(req.letter_spacing);
            }
            a
        };

        let mut boundaries = std::collections::BTreeSet::new();
        boundaries.insert(0usize);
        boundaries.insert(req.text.len());
        for r in req.weight_ranges {
            boundaries.insert(char_to_byte(req.text, r.start));
            boundaries.insert(char_to_byte(req.text, r.end));
        }
        for r in req.italic_ranges {
            boundaries.insert(char_to_byte(req.text, r.start));
            boundaries.insert(char_to_byte(req.text, r.end));
        }
        for r in req.font_family_ranges {
            boundaries.insert(char_to_byte(req.text, r.start));
            boundaries.insert(char_to_byte(req.text, r.end));
        }

        let boundaries: Vec<usize> = boundaries.into_iter().collect();
        let base_attrs = Attrs::new();
        let mut rich_spans: Vec<(&str, Attrs)> = Vec::new();

        for w in boundaries.windows(2) {
            let (start, end) = (w[0], w[1]);
            if start >= end {
                continue;
            }
            let slice = &req.text[start..end];
            let mut a = node_attrs.clone();
            for r in req.weight_ranges {
                let sb = char_to_byte(req.text, r.start);
                let eb = char_to_byte(req.text, r.end);
                if sb <= start && start < eb {
                    a = a.weight(Weight(r.weight));
                    break;
                }
            }
            for r in req.italic_ranges {
                let sb = char_to_byte(req.text, r.start);
                let eb = char_to_byte(req.text, r.end);
                if sb <= start && start < eb {
                    a = a.style(CStyle::Italic);
                    break;
                }
            }
            for r in req.font_family_ranges {
                let sb = char_to_byte(req.text, r.start);
                let eb = char_to_byte(req.text, r.end);
                if sb <= start && start < eb && !r.font_family.is_empty() {
                    a = a.family(Family::Name(r.font_family.as_str()));
                    break;
                }
            }
            rich_spans.push((slice, a));
        }

        let has_ranges = !req.weight_ranges.is_empty()
            || !req.italic_ranges.is_empty()
            || !req.font_family_ranges.is_empty();

        if has_ranges {
            buffer.set_rich_text(
                &mut self.font_system,
                rich_spans.into_iter(),
                &base_attrs,
                Shaping::Advanced,
                None,
            );
        } else {
            buffer.set_text(
                &mut self.font_system,
                req.text,
                &node_attrs,
                Shaping::Advanced,
                None,
            );
        }

        buffer.shape_until_scroll(&mut self.font_system, false);

        let result = extract_result(&buffer, req.text);
        self.cache.cache.insert(key, (result.clone(), buffer));
        println!("measure took {:?}", t.elapsed());
        result
    }

    pub fn measure_reuse(&mut self, id: u64, req: TextMeasureRequest<'_>) -> TextMeasureResult {
        let t = std::time::Instant::now();
        let line_height = req.line_height.unwrap_or(req.size * 1.4);
        let font_system = &mut self.font_system;

        let buffer = self
            .reuse_buffers
            .entry(id)
            .or_insert_with(|| Buffer::new(font_system, Metrics::new(req.size, line_height)));

        buffer.set_metrics(font_system, Metrics::new(req.size, line_height));
        buffer.set_size(font_system, req.max_width, None);

        let node_attrs = {
            let mut a = Attrs::new().weight(Weight(req.weight));
            if req.italic {
                a = a.style(CStyle::Italic);
            }
            if !req.font_family.is_empty() {
                a = a.family(Family::Name(req.font_family));
            }
            if req.letter_spacing != 0.0 {
                a = a.letter_spacing(req.letter_spacing);
            }
            a
        };

        buffer.set_text(font_system, req.text, &node_attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(font_system, false);

        println!("measure_reuse took {:?}", t.elapsed());

        extract_result(buffer, req.text)
    }

    pub fn remove_reuse_buffer(&mut self, id: u64) {
        self.reuse_buffers.remove(&id);
    }

    pub fn take_buffer(&mut self, req: &TextMeasureRequest<'_>) -> Option<Buffer> {
        let key = MeasureKey::from_request(req);
        self.cache.cache.remove(&key).map(|(_, buf)| buf)
    }
}

fn extract_result(buffer: &Buffer, text: &str) -> TextMeasureResult {
    let mut lines: Vec<LineMetrics> = Vec::new();
    let mut total_width: f32 = 0.0;
    for run in buffer.layout_runs() {
        let line_w = run.glyphs.iter().fold(0.0f32, |acc, g| acc.max(g.x + g.w));
        lines.push(LineMetrics {
            width: line_w,
            height: run.line_height,
            baseline: run.line_y - run.line_top,
        });
        total_width = total_width.max(line_w.ceil());
    }
    let total_height = lines.iter().map(|l| l.height).sum();

    let mut glyph_positions = vec![0.0f32];
    let mut line_glyph_positions: Vec<Vec<f32>> = Vec::new();
    let mut line_start_chars: Vec<usize> = Vec::new();
    for run in buffer.layout_runs() {
        let mut line_positions = vec![0.0f32];
        let start_char = run
            .glyphs
            .first()
            .map(|g| byte_to_char(text, g.start))
            .unwrap_or(0);
        line_start_chars.push(start_char);
        for glyph in run.glyphs {
            glyph_positions.push(glyph.x + glyph.w);
            line_positions.push(glyph.x + glyph.w);
        }
        line_glyph_positions.push(line_positions);
    }

    TextMeasureResult {
        width: total_width,
        height: total_height,
        line_count: lines.len(),
        lines,
        glyph_positions,
        line_glyph_positions,
        line_start_chars,
    }
}

fn char_to_byte(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

fn byte_to_char(text: &str, byte_idx: usize) -> usize {
    text[..byte_idx.min(text.len())].chars().count()
}

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
    pub glyph_positions: Vec<f32>,
    pub line_glyph_positions: Vec<Vec<f32>>,
    pub line_start_chars: Vec<usize>,
}

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
