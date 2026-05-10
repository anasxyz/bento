use slab::Slab;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNodeId(pub usize);

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

#[derive(Clone, PartialEq, Debug)]
pub struct ColorRange {
    pub start: usize,
    pub end: usize,
    pub color: [f32; 4],
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
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct TextNode {
    pub text: String,
    pub x: f32,
    pub y: f32,
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

#[derive(Debug)]
pub enum Node {
    Rect(RectNode),
    Text(TextNode),
    Image(ImageNode),
    Group(GroupNode),
}

impl Node {
    pub fn set_position(&mut self, x: f32, y: f32, w: f32, h: f32) {
        match self {
            Node::Rect(r) => {
                r.x = x;
                r.y = y;
                r.w = w;
                r.h = h;
            }
            Node::Text(t) => {
                t.x = x;
                t.y = y;
            }
            Node::Image(i) => {
                i.x = x;
                i.y = y;
                i.w = w;
                i.h = h;
            }
            Node::Group(g) => {
                g.offset_x = x;
                g.offset_y = y;
            }
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub nodes: Slab<Node>,
    pub root: Vec<SceneNodeId>,
    parent_stack: Vec<SceneNodeId>,
    // when true all nodes added using attach() are recorded in tracked below
    tracking: bool,
    tracked: Vec<SceneNodeId>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            nodes: Slab::new(),
            root: Vec::new(),
            parent_stack: Vec::new(),
            tracking: false,
            tracked: Vec::new(),
        }
    }

    pub fn add_rect(&mut self, rect: RectNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(Node::Rect(rect)));
        self.attach(id);
        id
    }

    pub fn add_text(&mut self, text: TextNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(Node::Text(text)));
        self.attach(id);
        id
    }

    pub fn add_image(&mut self, image: ImageNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(Node::Image(image)));
        self.attach(id);
        id
    }

    pub fn add_group(&mut self, group: GroupNode) -> SceneNodeId {
        let id = SceneNodeId(self.nodes.insert(Node::Group(group)));
        self.attach(id);
        id
    }

    pub fn push_parent(&mut self, id: SceneNodeId) {
        self.parent_stack.push(id);
    }

    pub fn pop_parent(&mut self) {
        self.parent_stack.pop();
    }

    // begins recording all nodes added to the scene
    pub fn start_tracking(&mut self) {
        self.tracking = true;
        self.tracked.clear();
    }

    // stop recording and return the ids of all nodes added since start_tracking()
    pub fn stop_tracking(&mut self) -> Vec<SceneNodeId> {
        self.tracking = false;
        std::mem::take(&mut self.tracked)
    }

    fn attach(&mut self, id: SceneNodeId) {
        if self.tracking {
            self.tracked.push(id);
        }

        let parent = self.parent_stack.last().copied();
        match parent {
            Some(parent_id) => {
                if let Some(Node::Group(g)) = self.nodes.get_mut(parent_id.0) {
                    g.children.push(id);
                }
                match self.nodes.get_mut(id.0) {
                    Some(Node::Rect(r)) => r.parent = Some(parent_id),
                    Some(Node::Text(t)) => t.parent = Some(parent_id),
                    Some(Node::Image(i)) => i.parent = Some(parent_id),
                    Some(Node::Group(g)) => g.parent = Some(parent_id),
                    None => {}
                }
            }
            None => self.root.push(id),
        }
    }

    pub fn remove(&mut self, id: SceneNodeId) {
        let children = match self.nodes.get(id.0) {
            Some(Node::Group(g)) => g.children.clone(),
            _ => vec![],
        };

        for child_id in children {
            self.remove(child_id);
        }

        // remove from parents children or from root
        let parent = match self.nodes.get(id.0) {
            Some(Node::Rect(r)) => r.parent,
            Some(Node::Text(t)) => t.parent,
            Some(Node::Image(i)) => i.parent,
            Some(Node::Group(g)) => g.parent,
            None => return,
        };

        match parent {
            Some(parent_id) => {
                if let Some(Node::Group(g)) = self.nodes.get_mut(parent_id.0) {
                    g.children.retain(|&c| c != id);
                }
            }
            None => self.root.retain(|&r| r != id),
        }

        self.nodes.remove(id.0);
    }

    pub fn reparent(&mut self, id: SceneNodeId, new_parent: SceneNodeId) {
        // detach from current parent or root
        let old_parent = match self.nodes.get(id.0) {
            Some(Node::Rect(r)) => r.parent,
            Some(Node::Text(t)) => t.parent,
            Some(Node::Image(i)) => i.parent,
            Some(Node::Group(g)) => g.parent,
            None => return,
        };
        match old_parent {
            Some(p) => {
                if let Some(Node::Group(g)) = self.nodes.get_mut(p.0) {
                    g.children.retain(|&c| c != id);
                }
            }
            None => self.root.retain(|&r| r != id),
        }
        // attach under new parent
        if let Some(Node::Group(g)) = self.nodes.get_mut(new_parent.0) {
            g.children.push(id);
        }
        match self.nodes.get_mut(id.0) {
            Some(Node::Rect(r)) => r.parent = Some(new_parent),
            Some(Node::Text(t)) => t.parent = Some(new_parent),
            Some(Node::Image(i)) => i.parent = Some(new_parent),
            Some(Node::Group(g)) => g.parent = Some(new_parent),
            None => {}
        }
    }

    pub fn parent_of(&self, id: SceneNodeId) -> Option<SceneNodeId> {
        match self.nodes.get(id.0) {
            Some(Node::Rect(r)) => r.parent,
            Some(Node::Text(t)) => t.parent,
            Some(Node::Image(i)) => i.parent,
            Some(Node::Group(g)) => g.parent,
            None => None,
        }
    }

    pub fn get(&self, id: SceneNodeId) -> Option<&Node> {
        self.nodes.get(id.0)
    }

    pub fn get_mut(&mut self, id: SceneNodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id.0)
    }
}
