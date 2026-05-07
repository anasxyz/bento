use crate::measure::{LineMetrics, TextMeasureRequest, TextMeasureResult, TextMeasurer};
use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping, Style as CStyle, Weight};

pub struct CosmicTextMeasurer<'a> {
    pub font_system: &'a mut cosmic_text::FontSystem,
    buffer: Buffer,
}

impl<'a> CosmicTextMeasurer<'a> {
    pub fn new(font_system: &'a mut cosmic_text::FontSystem) -> Self {
        Self {
            buffer: Buffer::new(font_system, Metrics::new(16.0, 22.4)),
            font_system,
        }
    }
}

impl<'a> TextMeasurer for CosmicTextMeasurer<'a> {
    fn measure(&mut self, req: TextMeasureRequest) -> TextMeasureResult {
        let line_height = req.line_height.unwrap_or(req.size * 1.4);
        let mut buffer = Buffer::new(self.font_system, Metrics::new(req.size, line_height));
        self.buffer
            .set_metrics(self.font_system, Metrics::new(req.size, line_height));
        self.buffer.set_size(self.font_system, req.max_width, None);

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

        // build boundaries the same way text.rs does
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

        buffer.set_rich_text(
            self.font_system,
            rich_spans.into_iter(),
            &base_attrs,
            Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(self.font_system, false);

        let mut lines: Vec<LineMetrics> = Vec::new();
        let mut total_width: f32 = 0.0;

        for run in buffer.layout_runs() {
            let line_w = run.glyphs.iter().fold(0.0f32, |acc, g| acc.max(g.x + g.w));
            let lm = LineMetrics {
                width: line_w,
                height: run.line_height,
                baseline: run.line_y - run.line_top,
            };
            total_width = total_width.max(line_w);
            lines.push(lm);
        }

        let total_height = lines.iter().map(|l| l.height).sum();

        TextMeasureResult {
            width: total_width,
            height: total_height,
            line_count: lines.len(),
            lines,
        }
    }
}

fn char_to_byte(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{measure::TextMeasureRequest, scene::{FontFamilyRange, ItalicRange, WeightRange}};

    fn make_measurer(fs: &mut cosmic_text::FontSystem) -> CosmicTextMeasurer<'_> {
        CosmicTextMeasurer::new(fs)
    }

    fn req<'a>(text: &'a str, size: f32, max_width: Option<f32>) -> TextMeasureRequest<'a> {
        TextMeasureRequest {
            text,
            font_family: "",
            size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        }
    }

    #[test]
    fn empty_string() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req("", 16.0, None));
        assert_eq!(r.width, 0.0, "empty string should have zero width");
        for line in &r.lines {
            assert_eq!(line.width, 0.0, "empty string lines should have zero width");
        }
    }

    #[test]
    fn whitespace_only() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req("   ", 16.0, None));
        assert!(r.height > 0.0, "whitespace should still have line height");
    }

    #[test]
    fn single_char() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req("A", 16.0, None));
        assert_eq!(r.line_count, 1);
        assert!(r.width > 0.0);
        assert!(r.height > 0.0);
    }

    #[test]
    fn no_wrap_without_max_width() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req(
            "The quick brown fox jumps over the lazy dog",
            16.0,
            None,
        ));
        assert_eq!(r.line_count, 1, "should not wrap without max_width");
    }

    #[test]
    fn wraps_with_max_width() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req(
            "The quick brown fox jumps over the lazy dog",
            16.0,
            Some(100.0),
        ));
        assert!(r.line_count > 1, "should wrap within max_width");
        for line in &r.lines {
            assert!(
                line.width <= 100.0 + 1.0,
                "line width {} exceeds max_width 100.0",
                line.width
            );
        }
    }

    #[test]
    fn height_equals_sum_of_line_heights() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req(
            "The quick brown fox jumps over the lazy dog",
            16.0,
            Some(100.0),
        ));
        let sum: f32 = r.lines.iter().map(|l| l.height).sum();
        assert!(
            (r.height - sum).abs() < 0.01,
            "total height {} != sum of line heights {}",
            r.height,
            sum
        );
    }

    #[test]
    fn line_count_matches_lines_vec() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req(
            "The quick brown fox jumps over the lazy dog",
            16.0,
            Some(100.0),
        ));
        assert_eq!(r.line_count, r.lines.len());
    }

    #[test]
    fn larger_size_produces_larger_dimensions() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let small = m.measure(req("Hello world", 12.0, None));
        let large = m.measure(req("Hello world", 24.0, None));
        assert!(large.width > small.width, "larger font should be wider");
        assert!(large.height > small.height, "larger font should be taller");
    }

    #[test]
    fn bold_is_wider_than_regular() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let regular = m.measure(req("Hello world", 16.0, None));
        let bold_ranges = [WeightRange {
            start: 0,
            end: 11,
            weight: 700,
        }];
        let bold = m.measure(TextMeasureRequest {
            weight_ranges: &bold_ranges,
            ..req("Hello world", 16.0, None)
        });
        assert!(
            bold.width >= regular.width,
            "bold should be at least as wide as regular"
        );
    }

    #[test]
    fn italic_range_does_not_crash() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let ranges = [ItalicRange { start: 0, end: 5 }];
        let r = m.measure(TextMeasureRequest {
            italic_ranges: &ranges,
            ..req("Hello world", 16.0, None)
        });
        assert!(r.width > 0.0);
        assert_eq!(r.line_count, 1);
    }

    #[test]
    fn font_family_range_does_not_crash() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let ranges = [FontFamilyRange {
            start: 0,
            end: 5,
            font_family: "serif".to_string(),
        }];
        let r = m.measure(TextMeasureRequest {
            font_family_ranges: &ranges,
            ..req("Hello world", 16.0, None)
        });
        assert!(r.width > 0.0);
        assert_eq!(r.line_count, 1);
    }

    #[test]
    fn deterministic() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r1 = m.measure(req("Hello world", 16.0, Some(80.0)));
        let r2 = m.measure(req("Hello world", 16.0, Some(80.0)));
        assert_eq!(r1.width, r2.width);
        assert_eq!(r1.height, r2.height);
        assert_eq!(r1.line_count, r2.line_count);
        for (a, b) in r1.lines.iter().zip(r2.lines.iter()) {
            assert_eq!(a.width, b.width);
            assert_eq!(a.height, b.height);
            assert_eq!(a.baseline, b.baseline);
        }
    }

    #[test]
    fn very_long_word_does_not_panic() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let long = "a".repeat(500);
        let r = m.measure(req(&long, 16.0, Some(100.0)));
        assert!(r.width > 0.0);
        assert!(r.height > 0.0);
    }

    #[test]
    fn multiline_explicit_newline() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);
        let r = m.measure(req("line one\nline two\nline three", 16.0, None));
        assert_eq!(r.line_count, 3, "explicit newlines should produce 3 lines");
    }

    #[test]
    fn buffer_reuse_gives_same_results() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut m = make_measurer(&mut fs);

        // measure something large first to dirty the buffer
        let _ = m.measure(req(
            "The quick brown fox jumps over the lazy dog and more text here",
            24.0,
            Some(100.0),
        ));

        // then measure something small — should not be affected by previous call
        let r = m.measure(req("Hi", 16.0, None));
        assert_eq!(r.line_count, 1);
        assert!(r.width > 0.0);

        // and back to the large one — should match a fresh measurement
        let mut fs2 = cosmic_text::FontSystem::new();
        let mut m2 = make_measurer(&mut fs2);
        let expected = m2.measure(req(
            "The quick brown fox jumps over the lazy dog and more text here",
            24.0,
            Some(100.0),
        ));
        let actual = m.measure(req(
            "The quick brown fox jumps over the lazy dog and more text here",
            24.0,
            Some(100.0),
        ));
        assert_eq!(actual.width, expected.width);
        assert_eq!(actual.height, expected.height);
        assert_eq!(actual.line_count, expected.line_count);
    }
}
