use bento_wgpu::{ImageKey, RenderContext, Renderer};
use std::collections::HashMap;

struct PendingImage {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

pub(crate) struct ImageManager {
    pending: Vec<(ImageKey, PendingImage)>,
    next_key: u64,
}

impl ImageManager {
    pub(crate) fn new() -> Self {
        Self {
            pending: Vec::new(),
            next_key: 1,
        }
    }

    /// load a raster image from disk
    pub(crate) fn load_image(&mut self, path: &str) -> ImageKey {
        let img = image::open(path)
            .unwrap_or_else(|e| panic!("load_image — could not open {path}: {e}"))
            .into_rgba8();
        let (width, height) = img.dimensions();
        self.push(img.into_raw(), width, height)
    }

    /// rasterize an svg at the given pixel size
    pub(crate) fn load_image_svg(&mut self, path: &str, width: u32, height: u32) -> ImageKey {
        let data = std::fs::read(path)
            .unwrap_or_else(|e| panic!("load_image_svg — could not read {path}: {e}"));
        let tree = resvg::usvg::Tree::from_data(&data, &resvg::usvg::Options::default())
            .unwrap_or_else(|e| panic!("load_image_svg — could not parse {path}: {e}"));

        let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
            .expect("load_image_svg — invalid dimensions");

        let scale_x = width as f32 / tree.size().width();
        let scale_y = height as f32 / tree.size().height();
        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::from_scale(scale_x, scale_y),
            &mut pixmap.as_mut(),
        );

        // resvg outputs premultiplied RGBA — un-premultiply for straight alpha upload
        let mut rgba = pixmap.take();
        for pixel in rgba.chunks_exact_mut(4) {
            let a = pixel[3];
            if a > 0 {
                pixel[0] = ((pixel[0] as u16 * 255) / a as u16).min(255) as u8;
                pixel[1] = ((pixel[1] as u16 * 255) / a as u16).min(255) as u8;
                pixel[2] = ((pixel[2] as u16 * 255) / a as u16).min(255) as u8;
            }
        }

        self.push(rgba, width, height)
    }

    /// upload raw rgba bytes already in memory
    pub(crate) fn load_raw(&mut self, rgba: Vec<u8>, width: u32, height: u32) -> ImageKey {
        self.push(rgba, width, height)
    }

    /// upload all pending images to the given renderer
    /// clled by App::resumed() after each BentoWindow is created
    pub(crate) fn flush(&mut self, renderer: &mut Renderer, ctx: &RenderContext) {
        for (key, img) in self.pending.drain(..) {
            renderer.upload_image(ctx, key, &img.rgba, img.width, img.height);
        }
    }

    fn push(&mut self, rgba: Vec<u8>, width: u32, height: u32) -> ImageKey {
        let key = ImageKey(self.next_key);
        self.next_key += 1;
        self.pending.push((
            key,
            PendingImage {
                rgba,
                width,
                height,
            },
        ));
        key
    }
}

impl Default for ImageManager {
    fn default() -> Self {
        Self::new()
    }
}
