use crate::color::Color;
use glyphon::{
    Attrs, Buffer, Cache, Color as GlyphonColor, Family, FontSystem, Metrics, Resolution, Shaping,
    Style as GlyphonStyle, SwashCache, TextArea, TextAtlas, TextBounds,
    TextRenderer as GlyphonRenderer, Viewport, Weight,
};
use wgpu;

pub struct TextParams {
    pub family: String,
    pub size: f32,
    pub weight: u16,
    pub italic: bool,
    pub color: Color,
    pub width: f32,
    pub clip: Option<[f32; 4]>,
}

struct Entry {
    buffer: Buffer,
    x: f32,
    y: f32,
    width: f32,
    scale: f32,
    clip: Option<[f32; 4]>,
    text: String,
    family: String,
    size: f32,
    weight: u16,
    italic: bool,
    color: GlyphonColor,
}

pub struct TextRenderer {
    cache: Cache,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    viewport: Viewport,
    renderer: GlyphonRenderer,
    entries: Vec<Entry>,
    active: usize,
    screen_width: f32,
    screen_height: f32,
    scale_factor: f64,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let cache = Cache::new(device);
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let renderer = GlyphonRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            None,
        );
        let viewport = Viewport::new(device, &cache);
        Self {
            cache,
            swash_cache,
            atlas,
            viewport,
            renderer,
            entries: Vec::new(),
            active: 0,
            screen_width: 800.0,
            screen_height: 600.0,
            scale_factor: 1.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32, scale_factor: f64) {
        self.screen_width = width;
        self.screen_height = height;
        self.scale_factor = scale_factor;
    }

    pub fn draw(
        &mut self,
        font_system: &mut FontSystem,
        text: &str,
        x: f32,
        y: f32,
        p: TextParams,
    ) {
        let glyphon_color = GlyphonColor::rgb(
            (p.color.r * 255.0) as u8,
            (p.color.g * 255.0) as u8,
            (p.color.b * 255.0) as u8,
        );
        let scale = self.scale_factor as f32;
        let line_height = p.size * 1.4;
        let idx = self.active;
        self.active += 1;

        let attrs = Attrs::new()
            .family(Family::Name(p.family.as_str()))
            .weight(Weight(p.weight))
            .style(if p.italic {
                GlyphonStyle::Italic
            } else {
                GlyphonStyle::Normal
            });

        if idx < self.entries.len() {
            let entry = &mut self.entries[idx];
            entry.x = x;
            entry.y = y;
            entry.scale = scale;
            entry.color = glyphon_color;
            entry.clip = p.clip;

            let changed = entry.text != text
                || entry.family != p.family
                || entry.size != p.size
                || entry.weight != p.weight
                || entry.italic != p.italic
                || entry.width != p.width;

            if changed {
                entry.text = text.to_string();
                entry.family = p.family.clone();
                entry.size = p.size;
                entry.weight = p.weight;
                entry.italic = p.italic;
                entry.width = p.width;
                entry
                    .buffer
                    .set_metrics(font_system, Metrics::new(p.size, line_height));
                entry.buffer.set_size(
                    font_system,
                    if p.width == f32::MAX {
                        None
                    } else {
                        Some(p.width)
                    },
                    None,
                );
                entry
                    .buffer
                    .set_text(font_system, text, &attrs, Shaping::Advanced);
                entry.buffer.shape_until_scroll(font_system, false);
            }
        } else {
            let mut buffer = Buffer::new(font_system, Metrics::new(p.size, line_height));
            buffer.set_size(
                font_system,
                if p.width == f32::MAX {
                    None
                } else {
                    Some(p.width)
                },
                None,
            );
            buffer.set_text(font_system, text, &attrs, Shaping::Advanced);
            buffer.shape_until_scroll(font_system, false);
            self.entries.push(Entry {
                buffer,
                x,
                y,
                width: p.width,
                clip: p.clip,
                scale,
                text: text.to_string(),
                family: p.family,
                size: p.size,
                weight: p.weight,
                italic: p.italic,
                color: glyphon_color,
            });
        }
    }

    pub fn render<'pass>(
        &'pass mut self,
        font_system: &mut FontSystem,
        screen_width: f32,
        screen_height: f32,
        scale_factor: f64,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        let phys_w = (screen_width * scale_factor as f32) as u32;
        let phys_h = (screen_height * scale_factor as f32) as u32;
        self.viewport.update(
            queue,
            Resolution {
                width: phys_w,
                height: phys_h,
            },
        );
        if self.active == 0 {
            return;
        }

        let text_areas: Vec<TextArea> = self.entries[..self.active]
            .iter()
            .map(|e| {
                let scale = e.scale;
                let bounds = match e.clip {
                    Some([cx, cy, cx2, cy2]) => TextBounds {
                        left: (cx * scale) as i32,
                        top: (cy * scale) as i32,
                        right: (cx2 * scale) as i32,
                        bottom: (cy2 * scale) as i32,
                    },
                    None => TextBounds {
                        left: 0,
                        top: 0,
                        right: phys_w as i32,
                        bottom: phys_h as i32,
                    },
                };
                TextArea {
                    buffer: &e.buffer,
                    left: e.x * scale,
                    top: e.y * scale,
                    scale,
                    bounds,
                    default_color: e.color,
                    custom_glyphs: &[],
                }
            })
            .collect();

        self.renderer
            .prepare(
                device,
                queue,
                font_system,
                &mut self.atlas,
                &self.viewport,
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();
        self.renderer
            .render(&self.atlas, &self.viewport, pass)
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.active = 0;
    }
    pub fn trim_atlas(&mut self) {
        self.atlas.trim();
    }
}
