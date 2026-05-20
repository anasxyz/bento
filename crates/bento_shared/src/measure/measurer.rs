use crate::measure::types::{LineMetrics, TextMeasureRequest, TextMeasureResult, TextMeasurer};
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

pub struct MeasureCache {
    buffer: Buffer,
    cache: HashMap<MeasureKey, TextMeasureResult>,
}

impl MeasureCache {
    pub fn new(font_system: &mut cosmic_text::FontSystem) -> Self {
        Self {
            buffer: Buffer::new(font_system, Metrics::new(16.0, 22.4)),
            cache: HashMap::new(),
        }
    }
}

pub struct CosmicTextMeasurer<'a> {
    pub font_system: &'a mut cosmic_text::FontSystem,
    cache: &'a mut MeasureCache,
}

impl<'a> CosmicTextMeasurer<'a> {
    pub fn new(font_system: &'a mut cosmic_text::FontSystem, cache: &'a mut MeasureCache) -> Self {
        Self { font_system, cache }
    }
}

impl<'a> TextMeasurer for CosmicTextMeasurer<'a> {
    fn measure(&mut self, req: TextMeasureRequest<'_>) -> TextMeasureResult {
        let key = MeasureKey::from_request(&req);
        if let Some(result) = self.cache.cache.get(&key) {
            return result.clone();
        }

        let line_height = req.line_height.unwrap_or(req.size * 1.4);

        self.cache
            .buffer
            .set_metrics(self.font_system, Metrics::new(req.size, line_height));
        self.cache
            .buffer
            .set_size(self.font_system, req.max_width, None);

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
            self.cache.buffer.set_rich_text(
                self.font_system,
                rich_spans.into_iter(),
                &base_attrs,
                Shaping::Advanced,
                None,
            );
        } else {
            self.cache.buffer.set_text(
                self.font_system,
                req.text,
                &node_attrs,
                Shaping::Advanced,
                None,
            );
        }

        self.cache
            .buffer
            .shape_until_scroll(self.font_system, false);

        let mut lines: Vec<LineMetrics> = Vec::new();
        let mut total_width: f32 = 0.0;

        for run in self.cache.buffer.layout_runs() {
            let line_w = run.glyphs.iter().fold(0.0f32, |acc, g| acc.max(g.x + g.w));
            let lm = LineMetrics {
                width: line_w,
                height: run.line_height,
                baseline: run.line_y - run.line_top,
            };
            total_width = total_width.max(line_w.ceil());
            lines.push(lm);
        }

        let total_height = lines.iter().map(|l| l.height).sum();

        let result = TextMeasureResult {
            width: total_width,
            height: total_height,
            line_count: lines.len(),
            lines,
        };

        self.cache.cache.insert(key, result.clone());
        result
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
    use crate::{
        measure::TextMeasureRequest,
        scene::{FontFamilyRange, ItalicRange, WeightRange},
    };

    fn make_measurer<'a>(
        fs: &'a mut cosmic_text::FontSystem,
        cache: &'a mut MeasureCache,
    ) -> CosmicTextMeasurer<'a> {
        CosmicTextMeasurer::new(fs, cache)
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let r = m.measure(req("", 16.0, None));
        assert_eq!(r.width, 0.0, "empty string should have zero width");
        for line in &r.lines {
            assert_eq!(line.width, 0.0, "empty string lines should have zero width");
        }
    }

    #[test]
    fn whitespace_only() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let r = m.measure(req("   ", 16.0, None));
        assert!(r.height > 0.0, "whitespace should still have line height");
    }

    #[test]
    fn single_char() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let r = m.measure(req("A", 16.0, None));
        assert_eq!(r.line_count, 1);
        assert!(r.width > 0.0);
        assert!(r.height > 0.0);
    }

    #[test]
    fn no_wrap_without_max_width() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let small = m.measure(req("Hello world", 12.0, None));
        let large = m.measure(req("Hello world", 24.0, None));
        assert!(large.width > small.width, "larger font should be wider");
        assert!(large.height > small.height, "larger font should be taller");
    }

    #[test]
    fn bold_is_wider_than_regular() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
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
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let long = "a".repeat(500);
        let r = m.measure(req(&long, 16.0, Some(100.0)));
        assert!(r.width > 0.0);
        assert!(r.height > 0.0);
    }

    #[test]
    fn multiline_explicit_newline() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let r = m.measure(req("line one\nline two\nline three", 16.0, None));
        assert_eq!(r.line_count, 3, "explicit newlines should produce 3 lines");
    }

    #[test]
    fn buffer_reuse_gives_same_results() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);

        let _ = m.measure(req(
            "The quick brown fox jumps over the lazy dog and more text here",
            24.0,
            Some(100.0),
        ));

        let r = m.measure(req("Hi", 16.0, None));
        assert_eq!(r.line_count, 1);
        assert!(r.width > 0.0);

        let mut fs2 = cosmic_text::FontSystem::new();
        let mut cache2 = MeasureCache::new(&mut fs2);
        let mut m2 = make_measurer(&mut fs2, &mut cache2);
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

    #[test]
    fn cache_returns_same_result() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let r1 = m.measure(req("Hello world", 16.0, Some(80.0)));
        let r2 = m.measure(req("Hello world", 16.0, Some(80.0)));
        assert_eq!(r1.width, r2.width);
        assert_eq!(r1.height, r2.height);
        assert_eq!(r1.line_count, r2.line_count);
    }

    #[test]
    fn cache_invalidates_on_text_change() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let r1 = m.measure(req("Hello", 16.0, None));
        let r2 = m.measure(req("Hello world", 16.0, None));
        assert!(
            r2.width > r1.width,
            "different text should produce different result"
        );
    }

    #[test]
    fn cache_invalidates_on_size_change() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);
        let r1 = m.measure(req("Hello world", 16.0, None));
        let r2 = m.measure(req("Hello world", 24.0, None));
        assert!(
            r2.width > r1.width,
            "different size should produce different result"
        );
    }

    #[test]
    fn bench_measure_performance() {
        let mut fs = cosmic_text::FontSystem::new();
        let mut cache = MeasureCache::new(&mut fs);
        let mut m = make_measurer(&mut fs, &mut cache);

        let weight_ranges = [
            WeightRange {
                start: 0,
                end: 10,
                weight: 700,
            },
            WeightRange {
                start: 50,
                end: 70,
                weight: 900,
            },
            WeightRange {
                start: 120,
                end: 140,
                weight: 300,
            },
        ];
        let italic_ranges = [
            ItalicRange { start: 10, end: 25 },
            ItalicRange {
                start: 80,
                end: 100,
            },
        ];
        let font_family_ranges = [
            FontFamilyRange {
                start: 25,
                end: 50,
                font_family: "serif".to_string(),
            },
            FontFamilyRange {
                start: 100,
                end: 120,
                font_family: "monospace".to_string(),
            },
        ];

        let request = TextMeasureRequest {
            text: "The quick brown fox jumps over the lazy dog and then some more words follow here to make this text really quite long indeed. We want to stress test the shaper with a good amount of content spread across many lines when wrapped tightly.",
            font_family: "",
            size: 16.0,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: Some(300.0),
            weight_ranges: &weight_ranges,
            italic_ranges: &italic_ranges,
            font_family_ranges: &font_family_ranges,
        };

        // warmup
        for _ in 0..5 {
            let _ = m.measure(TextMeasureRequest { ..request });
        }

        // timed runs — same input every time, should hit cache
        let iterations = 1000;
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = m.measure(TextMeasureRequest { ..request });
        }
        let elapsed = start.elapsed();
        let per_call = elapsed / iterations;
        println!(
            "cached — total: {:?}, per call: {:?}, calls/sec: {}",
            elapsed,
            per_call,
            (iterations as f64 / elapsed.as_secs_f64()) as u64
        );

        // timed runs — alternating two inputs with ranges, forces reshape every call
        let mut fs2 = cosmic_text::FontSystem::new();
        let mut cache2 = MeasureCache::new(&mut fs2);
        let mut m2 = make_measurer(&mut fs2, &mut cache2);
        let start = std::time::Instant::now();
        for i in 0..iterations {
            let text = if i % 2 == 0 {
                "The quick brown fox jumps over the lazy dog and then some more words follow here."
            } else {
                "Different text entirely to force a cache miss on every single iteration here."
            };
            let _ = m2.measure(TextMeasureRequest { text, ..request });
        }
        let elapsed = start.elapsed();
        println!(
            "uncached styled — total: {:?}, per call: {:?}, calls/sec: {}",
            elapsed,
            elapsed / iterations,
            (iterations as f64 / elapsed.as_secs_f64()) as u64
        );

        // timed runs — same text, alternating max_width only (layout-only path)
        let mut fs4 = cosmic_text::FontSystem::new();
        let mut cache4 = MeasureCache::new(&mut fs4);
        let mut m4 = make_measurer(&mut fs4, &mut cache4);
        let _ = m4.measure(TextMeasureRequest { ..request });
        let start = std::time::Instant::now();
        for i in 0..iterations {
            let max_width = if i % 2 == 0 { Some(300.0) } else { Some(400.0) };
            let _ = m4.measure(TextMeasureRequest {
                max_width,
                ..request
            });
        }
        let elapsed = start.elapsed();
        println!(
            "layout-only (max_width change) — total: {:?}, per call: {:?}, calls/sec: {}",
            elapsed,
            elapsed / iterations,
            (iterations as f64 / elapsed.as_secs_f64()) as u64
        );

        let mut fs3 = cosmic_text::FontSystem::new();
        let mut cache3 = MeasureCache::new(&mut fs3);
        let mut m3 = make_measurer(&mut fs3, &mut cache3);
        let start = std::time::Instant::now();
        for i in 0..iterations {
            let text = if i % 2 == 0 {
                "The quick brown fox jumps over the lazy dog and then some more words follow here."
            } else {
                "Different text entirely to force a cache miss on every single iteration here."
            };
            let _ = m3.measure(TextMeasureRequest {
                text,
                weight_ranges: &[],
                italic_ranges: &[],
                font_family_ranges: &[],
                ..request
            });
        }
        let elapsed = start.elapsed();
        println!(
            "uncached unstyled — total: {:?}, per call: {:?}, calls/sec: {}",
            elapsed,
            elapsed / iterations,
            (iterations as f64 / elapsed.as_secs_f64()) as u64
        );
    }

    #[test]
    fn bench_cold_start_per_call() {
        let weight_ranges = [
            WeightRange {
                start: 0,
                end: 10,
                weight: 700,
            },
            WeightRange {
                start: 50,
                end: 70,
                weight: 900,
            },
        ];

        let iterations = 100u32; // fewer because this is slow
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut fs = cosmic_text::FontSystem::new();
            let mut cache = MeasureCache::new(&mut fs);
            let mut m = CosmicTextMeasurer::new(&mut fs, &mut cache);
            let _ = m.measure(TextMeasureRequest {
                text: "The quick brown fox jumps over the lazy dog and then some more words.",
                font_family: "",
                size: 16.0,
                weight: 400,
                italic: false,
                letter_spacing: 0.0,
                line_height: None,
                max_width: Some(300.0),
                weight_ranges: &weight_ranges,
                italic_ranges: &[],
                font_family_ranges: &[],
            });
        }
        let elapsed = start.elapsed();
        println!(
            "cold start per call — total: {:?}, per call: {:?}, calls/sec: {}",
            elapsed,
            elapsed / iterations,
            (iterations as f64 / elapsed.as_secs_f64()) as u64
        );
    }
}
