use std::fmt;

use slab::Slab;

use crate::scene::types::*;

#[derive(Debug)]
pub struct RectNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub z: i32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,

    pub parent: Option<SceneNodeId>,
    pub slot: u32,
}

impl RectNode {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x,
            y,
            w,
            h,
            color: [1.0, 1.0, 1.0, 1.0],
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            z: 1,
            opacity: 1.0,
            clip: None,

            parent: None,
            slot: u32::MAX,
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    pub fn w(&mut self, w: f32) -> &mut Self {
        self.w = w;
        self
    }
    pub fn h(&mut self, h: f32) -> &mut Self {
        self.h = h;
        self
    }
    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.w = w;
        self.h = h;
        self
    }
    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }
    pub fn radii(&mut self, radii: [f32; 4]) -> &mut Self {
        self.radii = radii;
        self
    }
    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.radii = [r; 4];
        self
    }
    pub fn border(&mut self, color: [f32; 4], widths: [f32; 4]) -> &mut Self {
        self.border_color = color;
        self.border_widths = widths;
        self
    }
    pub fn border_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.border_color = color;
        self
    }
    pub fn border_widths(&mut self, widths: [f32; 4]) -> &mut Self {
        self.border_widths = widths;
        self
    }
    pub fn border_width(&mut self, w: f32) -> &mut Self {
        self.border_widths = [w; 4];
        self
    }
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.rotate = angle;
        self
    }
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }
    pub fn scale_x(&mut self, x: f32) -> &mut Self {
        self.scale_x = x;
        self
    }
    pub fn scale_y(&mut self, y: f32) -> &mut Self {
        self.scale_y = y;
        self
    }
    pub fn z(&mut self, z: i32) -> &mut Self {
        self.z = z;
        self
    }
    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.opacity = opacity;
        self
    }
    pub fn clip(&mut self, clip: [f32; 4]) -> &mut Self {
        self.clip = Some(clip);
        self
    }
    pub fn no_clip(&mut self) -> &mut Self {
        self.clip = None;
        self
    }
}

#[derive(Debug)]
pub struct TextNode {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub size: f32,
    pub color: [f32; 4],
    pub z: i32,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub weight: u16,
    pub italic: bool,
    pub font_family: String,
    pub max_width: Option<f32>,
    pub line_height: Option<f32>,
    pub align: TextAlign,
    pub letter_spacing: f32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub color_ranges: Vec<ColorRange>,
    pub background_ranges: Vec<DecorationRange>,
    pub underline_ranges: Vec<DecorationRange>,
    pub strikethrough_ranges: Vec<DecorationRange>,
    pub weight_ranges: Vec<WeightRange>,
    pub italic_ranges: Vec<ItalicRange>,
    pub font_family_ranges: Vec<FontFamilyRange>,

    pub parent: Option<SceneNodeId>,
    pub slot: usize,
}

impl TextNode {
    pub fn new(text: &str, x: f32, y: f32, size: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            w: 0.0,
            h: 0.0,
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            z: 1,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            align: TextAlign::Left,
            letter_spacing: 0.0,
            opacity: 1.0,
            clip: None,
            color_ranges: Vec::new(),
            background_ranges: Vec::new(),
            underline_ranges: Vec::new(),
            strikethrough_ranges: Vec::new(),
            weight_ranges: Vec::new(),
            italic_ranges: Vec::new(),
            font_family_ranges: Vec::new(),

            parent: None,
            slot: usize::MAX,
        }
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }
    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn size(&mut self, size: f32) -> &mut Self {
        self.size = size;
        self
    }
    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }
    pub fn z(&mut self, z: i32) -> &mut Self {
        self.z = z;
        self
    }
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.rotate = angle;
        self
    }
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }
    pub fn scale_x(&mut self, x: f32) -> &mut Self {
        self.scale_x = x;
        self
    }
    pub fn scale_y(&mut self, y: f32) -> &mut Self {
        self.scale_y = y;
        self
    }
    pub fn weight(&mut self, weight: u16) -> &mut Self {
        self.weight = weight;
        self
    }
    pub fn italic(&mut self, italic: bool) -> &mut Self {
        self.italic = italic;
        self
    }
    pub fn font_family(&mut self, family: &str) -> &mut Self {
        self.font_family = family.to_string();
        self
    }
    pub fn max_width(&mut self, width: f32) -> &mut Self {
        self.max_width = Some(width);
        self
    }
    pub fn no_max_width(&mut self) -> &mut Self {
        self.max_width = None;
        self
    }
    pub fn line_height(&mut self, height: f32) -> &mut Self {
        self.line_height = Some(height);
        self
    }
    pub fn no_line_height(&mut self) -> &mut Self {
        self.line_height = None;
        self
    }
    pub fn align(&mut self, align: TextAlign) -> &mut Self {
        self.align = align;
        self
    }
    pub fn letter_spacing(&mut self, spacing: f32) -> &mut Self {
        self.letter_spacing = spacing;
        self
    }
    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.opacity = opacity;
        self
    }
    pub fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) -> &mut Self {
        self.clip = Some([x, y, w, h]);
        self
    }
    pub fn no_clip(&mut self) -> &mut Self {
        self.clip = None;
        self
    }

    pub fn add_color(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.color_ranges.push(ColorRange { start, end, color });
        self
    }
    pub fn add_background(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.background_ranges
            .push(DecorationRange { start, end, color });
        self
    }
    pub fn add_underline(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.underline_ranges
            .push(DecorationRange { start, end, color });
        self
    }
    pub fn add_strikethrough(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.strikethrough_ranges
            .push(DecorationRange { start, end, color });
        self
    }
    pub fn add_weight(&mut self, start: usize, end: usize, weight: u16) -> &mut Self {
        self.weight_ranges.push(WeightRange { start, end, weight });
        self
    }
    pub fn add_italic(&mut self, start: usize, end: usize) -> &mut Self {
        self.italic_ranges.push(ItalicRange { start, end });
        self
    }
    pub fn add_font_family(&mut self, start: usize, end: usize, family: &str) -> &mut Self {
        self.font_family_ranges.push(FontFamilyRange {
            start,
            end,
            font_family: family.to_string(),
        });
        self
    }
    pub fn clear_colors(&mut self) -> &mut Self {
        self.color_ranges.clear();
        self
    }
    pub fn clear_backgrounds(&mut self) -> &mut Self {
        self.background_ranges.clear();
        self
    }
    pub fn clear_underlines(&mut self) -> &mut Self {
        self.underline_ranges.clear();
        self
    }
    pub fn clear_strikethroughs(&mut self) -> &mut Self {
        self.strikethrough_ranges.clear();
        self
    }
    pub fn clear_weights(&mut self) -> &mut Self {
        self.weight_ranges.clear();
        self
    }
    pub fn clear_italics(&mut self) -> &mut Self {
        self.italic_ranges.clear();
        self
    }
    pub fn clear_font_families(&mut self) -> &mut Self {
        self.font_family_ranges.clear();
        self
    }
    pub fn clear_all_ranges(&mut self) -> &mut Self {
        self.color_ranges.clear();
        self.background_ranges.clear();
        self.underline_ranges.clear();
        self.strikethrough_ranges.clear();
        self.weight_ranges.clear();
        self.italic_ranges.clear();
        self.font_family_ranges.clear();
        self
    }
}

#[derive(Debug)]
pub struct ImageNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub image_id: u64,
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub z: i32,

    pub parent: Option<SceneNodeId>,
    pub slot: usize,
}

impl ImageNode {
    pub fn new(x: f32, y: f32, w: f32, h: f32, image_id: u64) -> Self {
        Self {
            x,
            y,
            w,
            h,
            image_id,
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 1,

            parent: None,
            slot: usize::MAX,
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.w = w;
        self.h = h;
        self
    }
    pub fn radii(&mut self, radii: [f32; 4]) -> &mut Self {
        self.radii = radii;
        self
    }
    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.radii = [r; 4];
        self
    }
    pub fn border(&mut self, color: [f32; 4], widths: [f32; 4]) -> &mut Self {
        self.border_color = color;
        self.border_widths = widths;
        self
    }
    pub fn border_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.border_color = color;
        self
    }
    pub fn border_widths(&mut self, widths: [f32; 4]) -> &mut Self {
        self.border_widths = widths;
        self
    }
    pub fn border_width(&mut self, w: f32) -> &mut Self {
        self.border_widths = [w; 4];
        self
    }
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.rotate = angle;
        self
    }
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }
    pub fn scale_x(&mut self, x: f32) -> &mut Self {
        self.scale_x = x;
        self
    }
    pub fn scale_y(&mut self, y: f32) -> &mut Self {
        self.scale_y = y;
        self
    }
    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.opacity = opacity;
        self
    }
    pub fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) -> &mut Self {
        self.clip = Some([x, y, w, h]);
        self
    }
    pub fn no_clip(&mut self) -> &mut Self {
        self.clip = None;
        self
    }
    pub fn z(&mut self, z: i32) -> &mut Self {
        self.z = z;
        self
    }
}

#[derive(Debug)]
pub struct GroupNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub opacity: Option<f32>,
    pub clip: Option<[f32; 4]>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub z: i32,

    pub parent: Option<SceneNodeId>,
    pub children: Vec<SceneNodeId>,
}

impl GroupNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: None,
            clip: None,
            offset_x: 0.0,
            offset_y: 0.0,
            z: 1,

            parent: None,
            children: Vec::new(),
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    pub fn w(&mut self, w: f32) -> &mut Self {
        self.w = w;
        self
    }
    pub fn h(&mut self, h: f32) -> &mut Self {
        self.h = h;
        self
    }
    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.rotate = angle;
        self
    }
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }
    pub fn scale_x(&mut self, x: f32) -> &mut Self {
        self.scale_x = x;
        self
    }
    pub fn scale_y(&mut self, y: f32) -> &mut Self {
        self.scale_y = y;
        self
    }
    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.opacity = Some(opacity);
        self
    }
    pub fn no_opacity(&mut self) -> &mut Self {
        self.opacity = None;
        self
    }
    pub fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) -> &mut Self {
        self.clip = Some([x, y, w, h]);
        self
    }
    pub fn no_clip(&mut self) -> &mut Self {
        self.clip = None;
        self
    }
    pub fn z(&mut self, z: i32) -> &mut Self {
        self.z = z;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNodeId(pub usize);

#[derive(Debug)]
pub enum SceneNode {
    Rect(RectNode),
    Text(TextNode),
    Image(ImageNode),
    Group(GroupNode),
}

impl SceneNode {
    pub fn set_position(&mut self, x: f32, y: f32, w: f32, h: f32) {
        match self {
            SceneNode::Rect(r) => {
                r.x = x;
                r.y = y;
                r.w = w;
                r.h = h;
            }
            SceneNode::Text(t) => {
                t.x = x;
                t.y = y;
            }
            SceneNode::Image(i) => {
                i.x = x;
                i.y = y;
                i.w = w;
                i.h = h;
            }
            SceneNode::Group(g) => {
                g.offset_x = x;
                g.offset_y = y;
            }
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub nodes: Slab<SceneNode>,
    pub root: Vec<SceneNodeId>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            nodes: Slab::new(),
            root: Vec::new(),
        }
    }

    pub fn add_rect(&mut self, rect: RectNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(SceneNode::Rect(rect)));
        self.root.push(id);
        id
    }

    pub fn add_text(&mut self, text: TextNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(SceneNode::Text(text)));
        self.root.push(id);
        id
    }

    pub fn add_image(&mut self, image: ImageNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(SceneNode::Image(image)));
        self.root.push(id);
        id
    }

    pub fn add_group(&mut self, group: GroupNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(SceneNode::Group(group)));
        self.root.push(id);
        id
    }

    pub fn append(&mut self, parent: SceneNodeId, child: SceneNodeId) {
        // remove from current parent or root
        let old_parent = self.parent_of(child);
        match old_parent {
            Some(p) => {
                if let Some(SceneNode::Group(g)) = self.nodes.get_mut(p.0) {
                    g.children.retain(|&c| c != child);
                }
            }
            None => self.root.retain(|&r| r != child),
        }
        // add to new parent
        if let Some(SceneNode::Group(g)) = self.nodes.get_mut(parent.0) {
            g.children.push(child);
        }
        // update child's parent pointer
        match self.nodes.get_mut(child.0) {
            Some(SceneNode::Rect(r)) => r.parent = Some(parent),
            Some(SceneNode::Text(t)) => t.parent = Some(parent),
            Some(SceneNode::Image(i)) => i.parent = Some(parent),
            Some(SceneNode::Group(g)) => g.parent = Some(parent),
            None => {}
        }
    }

    pub fn remove(&mut self, id: SceneNodeId) {
        let children = match self.nodes.get(id.0) {
            Some(SceneNode::Group(g)) => g.children.clone(),
            _ => vec![],
        };
        for child_id in children {
            self.remove(child_id);
        }
        let parent = self.parent_of(id);
        match parent {
            Some(p) => {
                if let Some(SceneNode::Group(g)) = self.nodes.get_mut(p.0) {
                    g.children.retain(|&c| c != id);
                }
            }
            None => self.root.retain(|&r| r != id),
        }
        self.nodes.remove(id.0);
    }

    pub fn parent_of(&self, id: SceneNodeId) -> Option<SceneNodeId> {
        match self.nodes.get(id.0) {
            Some(SceneNode::Rect(r)) => r.parent,
            Some(SceneNode::Text(t)) => t.parent,
            Some(SceneNode::Image(i)) => i.parent,
            Some(SceneNode::Group(g)) => g.parent,
            None => None,
        }
    }

    pub fn get(&self, id: SceneNodeId) -> Option<&SceneNode> {
        self.nodes.get(id.0)
    }

    pub fn get_mut(&mut self, id: SceneNodeId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(id.0)
    }

    pub fn accumulated_offset_and_clip(&self, id: SceneNodeId) -> (f32, f32, Option<[f32; 4]>) {
        let mut offset_x = 0.0;
        let mut offset_y = 0.0;
        let mut clip = None;
        let mut current = self.parent_of(id);
        while let Some(parent_id) = current {
            if let Some(SceneNode::Group(g)) = self.nodes.get(parent_id.0) {
                offset_x += g.offset_x;
                offset_y += g.offset_y;
                if clip.is_none() {
                    clip = g.clip;
                }
                current = g.parent;
            } else {
                break;
            }
        }
        (offset_x, offset_y, clip)
    }

    pub fn hitbox(&self, id: SceneNodeId) -> (f32, f32, f32, f32) {
        let (x, y, w, h) = match self.nodes.get(id.0) {
            Some(SceneNode::Rect(r)) => (r.x, r.y, r.w, r.h),
            Some(SceneNode::Text(t)) => (t.x, t.y, t.w, t.h),
            Some(SceneNode::Image(i)) => (i.x, i.y, i.w, i.h),
            Some(SceneNode::Group(g)) => (g.x, g.y, g.w, g.h),
            None => return (0.0, 0.0, 0.0, 0.0),
        };
        let (ox, oy, clip) = self.accumulated_offset_and_clip(id);
        let sx = x + ox;
        let sy = y + oy;
        match clip {
            Some([cx, cy, cw, ch]) => {
                let x1 = sx.max(cx);
                let y1 = sy.max(cy);
                let x2 = (sx + w).min(cx + cw);
                let y2 = (sy + h).min(cy + ch);
                (x1, y1, (x2 - x1).max(0.0), (y2 - y1).max(0.0))
            }
            None => (sx, sy, w, h),
        }
    }

    pub fn print_tree(&self) {
        fn print_node(nodes: &Slab<SceneNode>, id: SceneNodeId, depth: usize) {
            let indent = "  ".repeat(depth);
            match nodes.get(id.0) {
                Some(SceneNode::Rect(r)) => println!(
                    "{}[{}] Rect [{:.0},{:.0} {:.0}x{:.0}]",
                    indent, id.0, r.x, r.y, r.w, r.h
                ),
                Some(SceneNode::Text(t)) => println!("{}[{}] Text {:?}", indent, id.0, t.text),
                Some(SceneNode::Image(i)) => println!(
                    "{}[{}] Image [{:.0},{:.0} {:.0}x{:.0}]",
                    indent, id.0, i.x, i.y, i.w, i.h
                ),
                Some(SceneNode::Group(g)) => {
                    println!("{}[{}] Group", indent, id.0);
                    for &child in &g.children {
                        print_node(nodes, child, depth + 1);
                    }
                }
                None => println!("{}[{}] [missing]", indent, id.0),
            }
        }

        println!("[Scene]");

        for &root_id in &self.root {
            print_node(&self.nodes, root_id, 0);
        }

        println!("\n");
    }
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node(
            f: &mut fmt::Formatter<'_>,
            nodes: &Slab<SceneNode>,
            id: SceneNodeId,
            depth: usize,
        ) -> fmt::Result {
            let indent = "  ".repeat(depth);
            match nodes.get(id.0) {
                Some(SceneNode::Rect(r)) => writeln!(
                    f,
                    "{}Rect [{:.0},{:.0} {:.0}x{:.0}]",
                    indent, r.x, r.y, r.w, r.h
                )?,
                Some(SceneNode::Text(t)) => {
                    writeln!(f, "{}Text [{:.0},{:.0}] {:?}", indent, t.x, t.y, t.text)?
                }
                Some(SceneNode::Image(i)) => writeln!(
                    f,
                    "{}Image [{:.0},{:.0} {:.0}x{:.0}]",
                    indent, i.x, i.y, i.w, i.h
                )?,
                Some(SceneNode::Group(g)) => {
                    writeln!(f, "{}Group", indent)?;
                    for &child in &g.children {
                        print_node(f, nodes, child, depth + 1)?;
                    }
                }
                None => writeln!(f, "{}[missing]", indent)?,
            }
            Ok(())
        }

        writeln!(f, "Scene ({} nodes):", self.nodes.len())?;
        for &root_id in &self.root {
            print_node(f, &self.nodes, root_id, 0)?;
        }
        Ok(())
    }
}
