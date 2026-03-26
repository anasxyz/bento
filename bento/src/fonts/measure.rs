// given a string and font attributes, returns (width, height) in logical pixels

use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Style, Weight};

use super::attrs::FontAttrs;

pub fn measure_text(
    font_system: &mut FontSystem,
    text: &str,
    attrs: &FontAttrs,
    max_width: Option<f32>,
) -> (f32, f32) {
    if text.is_empty() {
        return (0.0, attrs.line_height());
    }

    let metrics = Metrics::new(attrs.size, attrs.line_height());
    let mut buffer = Buffer::new(font_system, metrics);

    buffer.set_size(
        font_system,
        max_width,
        None, // unbounded height
    );

    let gattrs = Attrs::new()
        .family(Family::Name(&attrs.family))
        .weight(Weight(attrs.weight))
        .style(if attrs.italic {
            Style::Italic
        } else {
            Style::Normal
        });

    buffer.set_text(font_system, text, &gattrs, Shaping::Advanced);
    buffer.shape_until_scroll(font_system, false);

    // measure the laid out lines
    let mut w = 0.0f32;
    let mut h = 0.0f32;
    for run in buffer.layout_runs() {
        w = w.max(run.line_w);
        h += attrs.line_height();
    }

    (w, h.max(attrs.line_height()))
}
