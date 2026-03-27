use super::attrs::FontAttrs;
use super::cache::{FontCache, MeasureKey};
use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Style, Weight};

pub fn measure_text(
    font_system: &mut FontSystem,
    cache: &mut FontCache,
    text: &str,
    attrs: &FontAttrs,
    max_width: Option<f32>,
) -> (f32, f32) {
    if text.is_empty() {
        return (0.0, attrs.line_height());
    }

    let key = MeasureKey {
        text: text.to_string(),
        family: attrs.family.clone(),
        weight: attrs.weight,
        italic: attrs.italic,
        size_x10: (attrs.size * 10.0) as u32,
        max_width: max_width.map(|w| w as u32).unwrap_or(0),
    };

    if let Some(cached) = cache.get(&key) {
        return cached;
    }

    let line_height = attrs.line_height();
    let mut buffer = Buffer::new(font_system, Metrics::new(attrs.size, line_height));
    buffer.set_size(font_system, max_width, None);
    buffer.set_text(
        font_system,
        text,
        &Attrs::new()
            .family(Family::Name(&attrs.family))
            .weight(Weight(attrs.weight))
            .style(if attrs.italic {
                Style::Italic
            } else {
                Style::Normal
            }),
        Shaping::Advanced,
    );
    buffer.shape_until_scroll(font_system, false);

    let mut w = 0.0f32;
    let mut h = 0.0f32;
    for run in buffer.layout_runs() {
        w = w.max(run.line_w);
        h += line_height;
    }
    let result = (w, h.max(line_height));
    cache.insert(key, result);

    // DEBUG
    // println!("measured '{}' -> ({}, {})", text, result.0, result.1);

    result
}
