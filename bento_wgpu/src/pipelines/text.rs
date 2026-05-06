use crate::{
    TextAlign,
    pipelines::rect::RectInstance,
    scene::{ColorRange, DecorationRange, FontFamilyRange, ItalicRange, WeightRange},
};
use bytemuck::{Pod, Zeroable};
use cosmic_text::{
    Attrs, Buffer, CacheKey, Family, Metrics, Shaping, Style as CStyle, SwashCache, Weight,
};
use etagere::{Allocation, AtlasAllocator, size2};
use std::collections::HashMap;
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GlyphInstance {
    pub position: [f32; 2],
    pub origin: [f32; 2],
    pub size: [f32; 2],
    pub uv: [f32; 2],
    pub uv_size: [f32; 2],
    pub color: [f32; 4],
    pub transform: [f32; 4],
    pub is_color: u32,
    pub _pad: [u32; 3],
    pub clip: [f32; 4],
}

// everything needed to draw one piece of text
// constructed by the renderer
// from TextNode and passed as a slice to TextPipeline::prepare
pub struct TextSpec<'a> {
    pub text: &'a str,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: [f32; 4],
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub weight: u16,
    pub italic: bool,
    pub font_family: &'a str,
    pub max_width: Option<f32>,
    pub line_height: Option<f32>,
    pub letter_spacing: f32,
    pub align: TextAlign,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,

    // visual only
    pub color_ranges: &'a [ColorRange],
    pub background_ranges: &'a [DecorationRange],
    pub underline_ranges: &'a [DecorationRange],
    pub strikethrough_ranges: &'a [DecorationRange],

    // shaping relevant
    pub weight_ranges: &'a [WeightRange],
    pub italic_ranges: &'a [ItalicRange],
    pub font_family_ranges: &'a [FontFamilyRange],
}

const ATLAS_SIZE: u32 = 2048;

pub struct AtlasEntry {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub left: i32,
    pub top: i32,
    pub is_color: bool,
    allocation: Allocation,
}

pub struct GlyphAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub entries: HashMap<CacheKey, AtlasEntry>,
    packer: AtlasAllocator,
    pub swash: SwashCache,
}

impl GlyphAtlas {
    pub fn new(device: &wgpu::Device) -> Self {
        let (texture, view) = Self::make_texture(device);
        Self {
            texture,
            view,
            entries: HashMap::new(),
            packer: AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32)),
            swash: SwashCache::new(),
        }
    }

    fn make_texture(device: &wgpu::Device) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph atlas"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }

    pub fn clear(&mut self, device: &wgpu::Device) {
        self.packer = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        self.entries.clear();
        self.swash = SwashCache::new();
        let (texture, view) = Self::make_texture(device);
        self.texture = texture;
        self.view = view;
    }

    // look up an already rasterised glyph
    // returns none if not in atlas
    pub fn get(&self, key: CacheKey) -> Option<&AtlasEntry> {
        self.entries.get(&key)
    }

    // rasterise a glyph and insert it into the atlas
    // returns none if the
    // glyph has no pixels (for ex space)
    // clears and repacks if atlas is full
    pub fn insert(
        &mut self,
        key: CacheKey,
        font_system: &mut cosmic_text::FontSystem,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Option<&AtlasEntry> {
        if self.entries.contains_key(&key) {
            return self.entries.get(&key);
        }

        let image = self.swash.get_image_uncached(font_system, key)?;
        let w = image.placement.width;
        let h = image.placement.height;
        if w == 0 || h == 0 {
            return None;
        }

        let alloc = match self.packer.allocate(size2(w as i32 + 1, h as i32 + 1)) {
            Some(a) => a,
            None => {
                self.clear(device);
                self.packer.allocate(size2(w as i32 + 1, h as i32 + 1))?
            }
        };

        let x = alloc.rectangle.min.x as u32;
        let y = alloc.rectangle.min.y as u32;

        use cosmic_text::SwashContent;
        let rgba: Vec<u8> = match image.content {
            SwashContent::Color => image.data.to_vec(),
            SwashContent::Mask | SwashContent::SubpixelMask => {
                image.data.iter().flat_map(|&a| [a, a, a, a]).collect()
            }
        };

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );

        let is_color = matches!(image.content, SwashContent::Color);
        self.entries.insert(
            key,
            AtlasEntry {
                x,
                y,
                w,
                h,
                left: image.placement.left,
                top: image.placement.top,
                is_color,
                allocation: alloc,
            },
        );
        self.entries.get(&key)
    }
}

struct TextCache {
    // last seen values for dirty checking
    text: String,
    x: f32,
    y: f32,
    size: f32,
    color: [f32; 4],
    rotate: f32,
    scale_x: f32,
    scale_y: f32,
    weight: u16,
    italic: bool,
    font_family: String,
    max_width: Option<f32>,
    line_height: Option<f32>,
    letter_spacing: f32,
    align: TextAlign,
    opacity: f32,
    clip: Option<[f32; 4]>,

    color_ranges: Vec<ColorRange>,
    background_ranges: Vec<DecorationRange>,
    underline_ranges: Vec<DecorationRange>,
    strikethrough_ranges: Vec<DecorationRange>,
    weight_ranges: Vec<(usize, usize, u16)>,
    italic_ranges: Vec<(usize, usize)>,
    font_family_ranges: Vec<(usize, usize, String)>,

    // cached outputs
    buffer: Option<Buffer>,
    glyphs: Vec<GlyphInstance>,
    bg_rects: Vec<RectInstance>,
    line_rects: Vec<RectInstance>,
}

impl TextCache {
    fn empty() -> Self {
        Self {
            text: String::new(),
            x: f32::NAN,
            y: f32::NAN,
            size: f32::NAN,
            color: [f32::NAN; 4],
            rotate: f32::NAN,
            scale_x: f32::NAN,
            scale_y: f32::NAN,
            weight: 0,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: 1.0,
            clip: None,

            color_ranges: Vec::new(),
            background_ranges: Vec::new(),
            underline_ranges: Vec::new(),
            strikethrough_ranges: Vec::new(),
            weight_ranges: Vec::new(),
            italic_ranges: Vec::new(),
            font_family_ranges: Vec::new(),

            buffer: None,
            glyphs: Vec::new(),
            bg_rects: Vec::new(),
            line_rects: Vec::new(),
        }
    }

    fn needs_reshape(&self, s: &TextSpec) -> bool {
        self.text != s.text
            || self.size != s.size
            || self.rotate != s.rotate
            || self.scale_x != s.scale_x
            || self.scale_y != s.scale_y
            || self.weight != s.weight
            || self.italic != s.italic
            || self.font_family != s.font_family
            || self.max_width != s.max_width
            || self.line_height != s.line_height
            || self.letter_spacing != s.letter_spacing
            || self.align != s.align
            || self.weight_ranges.len() != s.weight_ranges.len()
            || self
                .weight_ranges
                .iter()
                .zip(s.weight_ranges.iter())
                .any(|(a, b)| a.0 != b.start || a.1 != b.end || a.2 != b.weight)
            || self.italic_ranges.len() != s.italic_ranges.len()
            || self
                .italic_ranges
                .iter()
                .zip(s.italic_ranges.iter())
                .any(|(a, b)| a.0 != b.start || a.1 != b.end)
            || self.font_family_ranges.len() != s.font_family_ranges.len()
            || self
                .font_family_ranges
                .iter()
                .zip(s.font_family_ranges.iter())
                .any(|(a, b)| a.0 != b.start || a.1 != b.end || a.2 != b.font_family)
    }

    fn needs_redraw(&self, s: &TextSpec) -> bool {
        self.x != s.x
            || self.y != s.y
            || self.color != s.color
            || self.opacity != s.opacity
            || self.clip != s.clip
            || self.color_ranges != s.color_ranges
            || self.background_ranges != s.background_ranges
            || self.underline_ranges != s.underline_ranges
            || self.strikethrough_ranges != s.strikethrough_ranges
    }

    fn update_from(&mut self, s: &TextSpec) {
        self.text = s.text.to_string();
        self.x = s.x;
        self.y = s.y;
        self.size = s.size;
        self.color = s.color;
        self.rotate = s.rotate;
        self.scale_x = s.scale_x;
        self.scale_y = s.scale_y;
        self.weight = s.weight;
        self.italic = s.italic;
        self.font_family = s.font_family.to_string();
        self.max_width = s.max_width;
        self.line_height = s.line_height;
        self.letter_spacing = s.letter_spacing;
        self.align = s.align.clone();
        self.opacity = s.opacity;
        self.clip = s.clip;

        self.color_ranges = s.color_ranges.to_vec();
        self.background_ranges = s.background_ranges.to_vec();
        self.underline_ranges = s.underline_ranges.to_vec();
        self.strikethrough_ranges = s.strikethrough_ranges.to_vec();
        self.weight_ranges = s
            .weight_ranges
            .iter()
            .map(|r| (r.start, r.end, r.weight))
            .collect();
        self.italic_ranges = s.italic_ranges.iter().map(|r| (r.start, r.end)).collect();
        self.font_family_ranges = s
            .font_family_ranges
            .iter()
            .map(|r| (r.start, r.end, r.font_family.clone()))
            .collect();
    }
}

fn shape_and_rasterise(
    spec: &TextSpec,
    font_system: &mut cosmic_text::FontSystem,
    atlas: &mut GlyphAtlas,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    scale: f32,
) -> Buffer {
    let line_height = spec.line_height.unwrap_or(spec.size * 1.4);
    let mut buffer = Buffer::new(font_system, Metrics::new(spec.size, line_height));
    buffer.set_size(font_system, spec.max_width, None);

    let align = match spec.align {
        TextAlign::Left => Some(cosmic_text::Align::Left),
        TextAlign::Center => Some(cosmic_text::Align::Center),
        TextAlign::Right => Some(cosmic_text::Align::Right),
    };

    let base_attrs = Attrs::new();
    let node_attrs = {
        let mut a = Attrs::new().weight(Weight(spec.weight));
        if spec.italic {
            a = a.style(CStyle::Italic);
        }
        if !spec.font_family.is_empty() {
            a = a.family(Family::Name(spec.font_family));
        }
        if spec.letter_spacing != 0.0 {
            a = a.letter_spacing(spec.letter_spacing);
        }
        a
    };

    let mut boundaries = std::collections::BTreeSet::new();
    boundaries.insert(0usize);
    boundaries.insert(spec.text.len());
    for r in spec.weight_ranges {
        boundaries.insert(char_to_byte(spec.text, r.start));
        boundaries.insert(char_to_byte(spec.text, r.end));
    }
    for r in spec.italic_ranges {
        boundaries.insert(char_to_byte(spec.text, r.start));
        boundaries.insert(char_to_byte(spec.text, r.end));
    }
    for r in spec.font_family_ranges {
        boundaries.insert(char_to_byte(spec.text, r.start));
        boundaries.insert(char_to_byte(spec.text, r.end));
    }
    for (i, c) in spec.text.char_indices() {
        if is_emoji(c) {
            boundaries.insert(i);
            boundaries.insert(i + c.len_utf8());
        }
    }

    let boundaries: Vec<usize> = boundaries.into_iter().collect();
    let mut rich_spans: Vec<(&str, Attrs)> = Vec::new();

    for w in boundaries.windows(2) {
        let (start, end) = (w[0], w[1]);
        if start >= end {
            continue;
        }
        let slice = &spec.text[start..end];
        let first_char = slice.chars().next().unwrap();

        let span_attrs = if is_emoji(first_char) {
            base_attrs.clone()
        } else {
            let mut a = node_attrs.clone();
            for r in spec.weight_ranges {
                let sb = char_to_byte(spec.text, r.start);
                let eb = char_to_byte(spec.text, r.end);
                if sb <= start && start < eb {
                    a = a.weight(Weight(r.weight));
                    break;
                }
            }
            for r in spec.italic_ranges {
                let sb = char_to_byte(spec.text, r.start);
                let eb = char_to_byte(spec.text, r.end);
                if sb <= start && start < eb {
                    a = a.style(CStyle::Italic);
                    break;
                }
            }
            for r in spec.font_family_ranges {
                let sb = char_to_byte(spec.text, r.start);
                let eb = char_to_byte(spec.text, r.end);
                if sb <= start && start < eb && !r.font_family.is_empty() {
                    a = a.family(Family::Name(r.font_family.as_str()));
                    break;
                }
            }
            a
        };
        rich_spans.push((slice, span_attrs));
    }

    buffer.set_rich_text(
        font_system,
        rich_spans.into_iter(),
        &base_attrs,
        Shaping::Advanced,
        align,
    );
    buffer.shape_until_scroll(font_system, false);

    let raster_scale = scale * spec.scale_x.max(spec.scale_y);
    let subpixel_offset = ((spec.x * scale).fract(), (spec.y * scale).fract());
    for run in buffer.layout_runs() {
        for glyph in run.glyphs {
            let physical = glyph.physical(subpixel_offset, raster_scale);
            atlas.insert(physical.cache_key, font_system, device, queue);
        }
    }

    buffer
}

// glyph instance building

fn build_glyphs(
    buffer: &Buffer,
    atlas: &GlyphAtlas,
    spec: &TextSpec,
    scale: f32,
) -> Vec<GlyphInstance> {
    let byte_to_char: Vec<usize> = {
        let mut map = vec![0usize; spec.text.len() + 1];
        for (char_idx, (byte_idx, _)) in spec.text.char_indices().enumerate() {
            map[byte_idx] = char_idx;
        }
        // fill in non boundary bytes with the char they belong to
        let mut last = 0;
        for i in 0..map.len() {
            if spec.text.is_char_boundary(i) {
                last = map[i];
            } else {
                map[i] = last;
            }
        }
        map
    };
    let raster_scale = scale * spec.scale_x.max(spec.scale_y);
    let origin_x = (spec.x * scale).floor();
    let origin_y = (spec.y * scale).floor();
    let subpixel_offset = ((spec.x * scale).fract(), (spec.y * scale).fract());
    let (cos_r, sin_r) = (spec.rotate.cos(), spec.rotate.sin());
    let transform = [
        cos_r * spec.scale_x,
        sin_r * spec.scale_x,
        -sin_r * spec.scale_y,
        cos_r * spec.scale_y,
    ];

    let mut instances = Vec::new();

    for run in buffer.layout_runs() {
        for glyph in run.glyphs {
            let physical = glyph.physical(subpixel_offset, raster_scale);
            let Some(entry) = atlas.get(physical.cache_key) else {
                continue;
            };

            let gx = (physical.x as f32 + entry.left as f32) / spec.scale_x.max(spec.scale_y);
            let gy = ((run.line_y * raster_scale).floor() + physical.y as f32 - entry.top as f32)
                / spec.scale_x.max(spec.scale_y);

            let char_idx = byte_to_char[glyph.start.min(spec.text.len())];
            let base_color = spec
                .color_ranges
                .iter()
                .find(|r| r.start <= char_idx && char_idx < r.end)
                .map(|r| r.color)
                .unwrap_or(spec.color);
            let color = [
                base_color[0],
                base_color[1],
                base_color[2],
                base_color[3] * spec.opacity,
            ];

            instances.push(GlyphInstance {
                position: [gx, gy],
                origin: [origin_x, origin_y],
                size: [
                    entry.w as f32 / spec.scale_x.max(spec.scale_y),
                    entry.h as f32 / spec.scale_x.max(spec.scale_y),
                ],
                uv: [
                    entry.x as f32 / ATLAS_SIZE as f32,
                    entry.y as f32 / ATLAS_SIZE as f32,
                ],
                uv_size: [
                    entry.w as f32 / ATLAS_SIZE as f32,
                    entry.h as f32 / ATLAS_SIZE as f32,
                ],
                color: glyph
                    .color_opt
                    .map(|c| {
                        [
                            c.r() as f32 / 255.0,
                            c.g() as f32 / 255.0,
                            c.b() as f32 / 255.0,
                            c.a() as f32 / 255.0 * spec.opacity,
                        ]
                    })
                    .unwrap_or(color),
                transform,
                is_color: entry.is_color as u32,
                _pad: [0; 3],
                clip: spec
                    .clip
                    .map(|c| [c[0] * scale, c[1] * scale, c[2] * scale, c[3] * scale])
                    .unwrap_or([0.0, 0.0, f32::MAX, f32::MAX]),
            });
        }
    }
    instances
}

// decoration building

struct DecorationAccum {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 4],
    clip: [f32; 4],
}

impl DecorationAccum {
    fn extend(&mut self, glyph_w: f32) {
        self.w += glyph_w;
    }

    fn flush(self, out: &mut Vec<RectInstance>) {
        if self.w > 0.0 {
            out.push(RectInstance {
                pos_size: [self.x, self.y, self.w, self.h],
                color: self.color,
                radii: [0.0; 4],
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                transform: [1.0, 0.0, 0.0, 1.0],
                clip: self.clip,
            });
        }
    }
}

fn accum_decoration(
    accum: &mut Option<DecorationAccum>,
    color: Option<[f32; 4]>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    opacity: f32,
    clip: [f32; 4],
    out: &mut Vec<RectInstance>,
) {
    match (color, accum) {
        (Some(c), Some(a)) if a.color == c => {
            a.extend(w);
        }
        (Some(c), slot) => {
            if let Some(a) = slot.take() {
                a.flush(out);
            }
            *slot = Some(DecorationAccum {
                x,
                y,
                w,
                h,
                color: [c[0], c[1], c[2], c[3] * opacity],
                clip,
            });
        }
        (None, slot) => {
            if let Some(a) = slot.take() {
                a.flush(out);
            }
        }
    }
}

// walk layout runs and produce background rects and line decoration rects
// returns (bg_rects, line_rects)
fn build_decorations(buffer: &Buffer, spec: &TextSpec) -> (Vec<RectInstance>, Vec<RectInstance>) {
    let byte_to_char: Vec<usize> = {
        let mut map = vec![0usize; spec.text.len() + 1];
        for (char_idx, (byte_idx, _)) in spec.text.char_indices().enumerate() {
            map[byte_idx] = char_idx;
        }
        let mut last = 0;
        for i in 0..map.len() {
            if spec.text.is_char_boundary(i) {
                last = map[i];
            } else {
                map[i] = last;
            }
        }
        map
    };

    let mut bg_rects = Vec::new();
    let mut line_rects = Vec::new();
    let clip = spec.clip.unwrap_or([0.0, 0.0, f32::MAX, f32::MAX]);

    for run in buffer.layout_runs() {
        let line_top = spec.y + run.line_top;
        let line_height = run.line_height;
        let ul_thickness = (spec.size * 0.07).max(1.0);
        let ul_y = spec.y + run.line_y + ul_thickness;
        let st_y = spec.y + run.line_y - run.line_height * 0.25;

        // collect per glyph info for this run
        struct GlyphInfo {
            char_idx: usize,
            x: f32,
            w: f32,
        }

        let glyph_infos: Vec<GlyphInfo> = run
            .glyphs
            .iter()
            .map(|g| GlyphInfo {
                char_idx: byte_to_char[g.start.min(spec.text.len())],
                x: spec.x + g.x,
                w: g.w,
            })
            .collect();

        // helper
        // emit one rect per contiguous range of matching glyphs
        // works for both ltr and rtl by using min/max of x positions
        let emit = |ranges: &[DecorationRange], y: f32, h: f32, out: &mut Vec<RectInstance>| {
            for range in ranges {
                let mut min_x = f32::MAX;
                let mut max_x = f32::MIN;
                for g in run.glyphs {
                    let char_idx = byte_to_char[g.start.min(spec.text.len())];
                    if range.start <= char_idx && char_idx < range.end {
                        min_x = min_x.min(spec.x + g.x);
                        max_x = max_x.max(spec.x + g.x + g.w);
                    }
                }
                if min_x >= max_x {
                    continue;
                }
                out.push(RectInstance {
                    pos_size: [min_x, y, max_x - min_x, h],
                    color: [
                        range.color[0],
                        range.color[1],
                        range.color[2],
                        range.color[3] * spec.opacity,
                    ],
                    radii: [0.0; 4],
                    border_color: [0.0; 4],
                    border_widths: [0.0; 4],
                    transform: [1.0, 0.0, 0.0, 1.0],
                    clip,
                });
            }
        };

        emit(spec.background_ranges, line_top, line_height, &mut bg_rects);
        emit(spec.underline_ranges, ul_y, ul_thickness, &mut line_rects);
        emit(
            spec.strikethrough_ranges,
            st_y,
            ul_thickness,
            &mut line_rects,
        );
    }

    (bg_rects, line_rects)
}

// text pipeline

pub struct TextPipeline {
    pub atlas: GlyphAtlas,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    screen_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    capacity: usize,
    ranges: Vec<(u32, u32)>,
    cache: Vec<TextCache>,
    scale: f32,

    pub bg_rects: Vec<RectInstance>,
    pub bg_ranges: Vec<(usize, usize)>,
    pub line_rects: Vec<RectInstance>,
    pub line_ranges: Vec<(usize, usize)>,
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
        let atlas = GlyphAtlas::new(device);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("glyph sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let screen_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text screen uniform"),
            size: 8,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &screen_buffer,
            0,
            bytemuck::cast_slice(&[screen_w * scale, screen_h * scale]),
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("text bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&atlas.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GlyphInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 40,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 56,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 72,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Uint32,
                },
                // account for _pad being [u32; 3] so 72 + 4 + 12 = 88
                wgpu::VertexAttribute {
                    offset: 88,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let capacity = 1024;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("glyph vertex buffer"),
            size: (capacity * std::mem::size_of::<GlyphInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            atlas,
            pipeline,
            vertex_buffer,
            screen_buffer,
            bind_group,
            bind_group_layout,
            sampler,
            capacity,
            ranges: Vec::new(),
            cache: Vec::new(),
            scale: 1.0,
            bg_rects: Vec::new(),
            bg_ranges: Vec::new(),
            line_rects: Vec::new(),
            line_ranges: Vec::new(),
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: f32, height: f32, scale: f32) {
        queue.write_buffer(
            &self.screen_buffer,
            0,
            bytemuck::cast_slice(&[width * scale, height * scale]),
        );
        // only clear cache if scale changes
        if scale != self.scale {
            self.scale = scale;
            self.cache.clear();
        }
    }

    pub fn prepare(
        &mut self,
        specs: &[TextSpec],
        font_system: &mut cosmic_text::FontSystem,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        while self.cache.len() < specs.len() {
            self.cache.push(TextCache::empty());
        }

        let mut instances = Vec::<GlyphInstance>::new();
        let mut any_changed = false;
        self.ranges.clear();
        self.bg_rects.clear();
        self.bg_ranges.clear();
        self.line_rects.clear();
        self.line_ranges.clear();

        for (i, (cache, spec)) in self.cache.iter_mut().zip(specs.iter()).enumerate() {
            let reshape = cache.needs_reshape(spec);
            let redraw = reshape || cache.needs_redraw(spec);

            if reshape {
                println!("[text] slot {} reshaping", i);
            } else if redraw {
                println!("[text] slot {} redraw only (no reshape)", i);
            } else {
                println!("[text] slot {} fully cached, skipping", i);
            }

            if redraw {
                any_changed = true;

                if reshape {
                    cache.buffer = Some(shape_and_rasterise(
                        spec,
                        font_system,
                        &mut self.atlas,
                        device,
                        queue,
                        self.scale,
                    ));
                }

                if let Some(buffer) = &cache.buffer {
                    cache.glyphs = build_glyphs(buffer, &self.atlas, spec, self.scale);
                    let (bg, lines) = build_decorations(buffer, spec);
                    cache.bg_rects = bg;
                    cache.line_rects = lines;
                }

                cache.update_from(spec);
            }

            let start = instances.len() as u32;
            instances.extend_from_slice(&cache.glyphs);
            self.ranges.push((start, cache.glyphs.len() as u32));

            let bg_start = self.bg_rects.len();
            self.bg_rects.extend_from_slice(&cache.bg_rects);
            self.bg_ranges.push((bg_start, self.bg_rects.len()));

            let line_start = self.line_rects.len();
            self.line_rects.extend_from_slice(&cache.line_rects);
            self.line_ranges.push((line_start, self.line_rects.len()));
        }

        if instances.is_empty() || !any_changed {
            return;
        }

        if instances.len() > self.capacity {
            self.capacity = instances.len().next_power_of_two();
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("glyph vertex buffer"),
                size: (self.capacity * std::mem::size_of::<GlyphInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&instances));
    }

    pub fn draw_range<'pass>(&'pass self, pass: &mut wgpu::RenderPass<'pass>, index: usize) {
        let Some(&(start, count)) = self.ranges.get(index) else {
            return;
        };
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..6, start..start + count);
    }
}

fn char_to_byte(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

fn floor_char_boundary(text: &str, byte_pos: usize) -> usize {
    let byte_pos = byte_pos.min(text.len());
    (0..=byte_pos)
        .rev()
        .find(|&i| text.is_char_boundary(i))
        .unwrap_or(0)
}

fn is_emoji(c: char) -> bool {
    matches!(c as u32,
        0x2600..=0x27BF
        | 0x1F000..=0x1FAFF
        | 0x2B50 | 0x2B55 | 0x2B1B..=0x2B1C
        | 0x2B05..=0x2B07
        | 0x2934..=0x2935
        | 0x3030 | 0x303D | 0x3297 | 0x3299
    )
}
