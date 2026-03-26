use glyphon::{
    Attrs, Buffer, Cache, Color as GColor, Family, FontSystem, Metrics, Resolution, Shaping,
    Style as GStyle, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer as GlyphonRenderer,
    Viewport, Weight,
};
use wgpu;

#[derive(Clone)]
struct SubmitMeta {
    x: f32,
    y: f32,
    color: GColor,
    clip: Option<[f32; 4]>,
}

struct BufferEntry {
    buffer: Buffer,
    text: String,
    family: String,
    size: f32,
    weight: u16,
    italic: bool,
    width: f32,
}

pub struct TextPipeline {
    cache: Cache,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    viewport: Viewport,
    renderer: GlyphonRenderer,
    entries: Vec<BufferEntry>,
    meta: Vec<SubmitMeta>,
    active: usize,
    screen_w: f32,
    screen_h: f32,
    scale: f32,
}

impl TextPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        screen_w: f32,
        screen_h: f32,
        scale: f32,
    ) -> Self {
        let cache = Cache::new(device);
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let renderer =
            GlyphonRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);
        let viewport = Viewport::new(device, &cache);
        Self {
            cache,
            swash_cache,
            atlas,
            viewport,
            renderer,
            entries: Vec::new(),
            meta: Vec::new(),
            active: 0,
            screen_w,
            screen_h,
            scale,
        }
    }

    pub fn resize(&mut self, screen_w: f32, screen_h: f32, scale: f32) {
        self.screen_w = screen_w;
        self.screen_h = screen_h;
        self.scale = scale;
    }

    pub fn begin_frame(&mut self) {
        self.active = 0;
    }

    pub fn submit(
        &mut self,
        font_system: &mut FontSystem,
        x: f32,
        y: f32,
        content: &str,
        family: &str,
        size: f32,
        weight: u16,
        italic: bool,
        color: [f32; 4],
        width: f32,
        clip: Option<[f32; 4]>,
    ) {
        let line_height = size * 1.4;
        let idx = self.active;
        self.active += 1;

        let attrs = Attrs::new()
            .family(Family::Name(family))
            .weight(Weight(weight))
            .style(if italic {
                GStyle::Italic
            } else {
                GStyle::Normal
            });

        if idx < self.entries.len() {
            let e = &mut self.entries[idx];
            let needs_reshape = e.text != content
                || e.family != family
                || e.size != size
                || e.weight != weight
                || e.italic != italic
                || e.width != width;
            if needs_reshape {
                e.text.clear();
                e.text.push_str(content);
                e.family.clear();
                e.family.push_str(family);
                e.size = size;
                e.weight = weight;
                e.italic = italic;
                e.width = width;
                e.buffer
                    .set_metrics(font_system, Metrics::new(size, line_height));
                e.buffer.set_size(
                    font_system,
                    if width >= f32::MAX { None } else { Some(width) },
                    None,
                );
                e.buffer
                    .set_text(font_system, content, &attrs, Shaping::Advanced);
                e.buffer.shape_until_scroll(font_system, false);
            }
        } else {
            let mut buf = Buffer::new(font_system, Metrics::new(size, line_height));
            buf.set_size(
                font_system,
                if width >= f32::MAX { None } else { Some(width) },
                None,
            );
            buf.set_text(font_system, content, &attrs, Shaping::Advanced);
            buf.shape_until_scroll(font_system, false);
            self.entries.push(BufferEntry {
                buffer: buf,
                text: content.to_string(),
                family: family.to_string(),
                size,
                weight,
                italic,
                width,
            });
        }

        let gc = GColor::rgba(
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
            (color[3] * 255.0) as u8,
        );
        let m = SubmitMeta {
            x,
            y,
            color: gc,
            clip,
        };
        if idx < self.meta.len() {
            self.meta[idx] = m;
        } else {
            self.meta.push(m);
        }
    }

    pub fn render<'pass>(
        &'pass mut self,
        font_system: &'pass mut FontSystem,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        let phys_w = (self.screen_w * self.scale) as u32;
        let phys_h = (self.screen_h * self.scale) as u32;
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

        let scale = self.scale;
        let areas: Vec<TextArea> = self.entries[..self.active]
            .iter()
            .zip(self.meta[..self.active].iter())
            .map(|(e, m)| {
                let bounds = match m.clip {
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
                    left: m.x * scale,
                    top: m.y * scale,
                    scale,
                    bounds,
                    default_color: m.color,
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
                areas,
                &mut self.swash_cache,
            )
            .unwrap();
        self.renderer
            .render(&self.atlas, &self.viewport, pass)
            .unwrap();
    }

    pub fn trim_atlas(&mut self) {
        self.atlas.trim();
    }
}
