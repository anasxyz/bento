use crate::widget::types::*;

pub struct Base {
    // computed position and size
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,

    // desired size
    pub(crate) width: Size,
    pub(crate) height: Size,
    pub(crate) min_w: Size,
    pub(crate) max_w: Size,
    pub(crate) min_h: Size,
    pub(crate) max_h: Size,

    // children layout
    pub(crate) direction: Direction, // Row | Col | RowReverse | ColReverse
    pub(crate) wrap: Wrap,           // None | Wrap | WrapReverse
    pub(crate) distribute: Distribute, // Start | Center | End | SpaceBetween | SpaceAround | SpaceEvenly
    pub(crate) align: Align,           // Start | Center | End | Stretch | Baseline
    pub(crate) gap: [f32; 2],          // [row_gap, col_gap]

    // self layout within parent
    pub(crate) self_align: SelfAlign, // Auto | Start | Center | End | Stretch | Baseline
    pub(crate) grow: f32,
    pub(crate) shrink: f32,
    pub(crate) basis: Size,

    // spacing
    pub(crate) padding: [f32; 4], // top, right, bottom, left
    pub(crate) margin: [f32; 4],

    // positioning
    pub(crate) position: Position, // Relative | Absolute
    pub(crate) inset: [Size; 4],   // top, right, bottom, left
    pub(crate) overflow: Overflow, // Visible | Hidden | Scroll
    pub(crate) aspect_ratio: Option<f32>,

    // rendering
    pub(crate) layer: u32,
    pub(crate) visible: bool,
    pub(crate) displayed: bool, // false = removed from layout

    // state
    pub(crate) focused: bool,

    // dirty flags
    pub(crate) layout_dirty: bool,
    pub(crate) render_dirty: bool,

    // extra
    pub(crate) content_w: f32,
    pub(crate) content_h: f32,
}

impl Base {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,

            width: Size::Auto,
            height: Size::Auto,
            min_w: Size::Auto,
            max_w: Size::Auto,
            min_h: Size::Auto,
            max_h: Size::Auto,

            direction: Direction::Row,
            wrap: Wrap::None,
            distribute: Distribute::Start,
            align: Align::Stretch,
            gap: [0.0, 0.0],

            self_align: SelfAlign::Auto,
            grow: 0.0,
            shrink: 1.0,
            basis: Size::Auto,

            padding: [0.0; 4],
            margin: [0.0; 4],

            position: Position::Relative,
            inset: [Size::Auto, Size::Auto, Size::Auto, Size::Auto],
            overflow: Overflow::Visible,
            aspect_ratio: None,

            layer: 0,
            visible: true,
            displayed: true,

            focused: false,

            layout_dirty: true,
            render_dirty: true,

            content_w: 0.0,
            content_h: 0.0,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn w(&self) -> f32 {
        self.w
    }
    pub fn h(&self) -> f32 {
        self.h
    }

    pub fn width(&self) -> &Size {
        &self.width
    }
    pub fn height(&self) -> &Size {
        &self.height
    }
    pub fn min_w(&self) -> &Size {
        &self.min_w
    }
    pub fn max_w(&self) -> &Size {
        &self.max_w
    }
    pub fn min_h(&self) -> &Size {
        &self.min_h
    }
    pub fn max_h(&self) -> &Size {
        &self.max_h
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    pub fn wrap(&self) -> &Wrap {
        &self.wrap
    }
    pub fn distribute(&self) -> &Distribute {
        &self.distribute
    }
    pub fn align(&self) -> &Align {
        &self.align
    }
    pub fn gap(&self) -> [f32; 2] {
        self.gap
    }

    pub fn self_align(&self) -> &SelfAlign {
        &self.self_align
    }
    pub fn grow(&self) -> f32 {
        self.grow
    }
    pub fn shrink(&self) -> f32 {
        self.shrink
    }
    pub fn basis(&self) -> &Size {
        &self.basis
    }

    pub fn padding(&self) -> [f32; 4] {
        self.padding
    }
    pub fn margin(&self) -> [f32; 4] {
        self.margin
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn inset(&self) -> &[Size; 4] {
        &self.inset
    }
    pub fn overflow(&self) -> &Overflow {
        &self.overflow
    }
    pub fn aspect_ratio(&self) -> Option<f32> {
        self.aspect_ratio
    }

    pub fn layer(&self) -> u32 {
        self.layer
    }
    pub fn visible(&self) -> bool {
        self.visible
    }
    pub fn displayed(&self) -> bool {
        self.displayed
    }
    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn content_w(&self) -> f32 {
        self.content_w
    }
    pub fn content_h(&self) -> f32 {
        self.content_h
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.layout_dirty = true;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.layout_dirty = true;
    }
    pub fn set_w(&mut self, w: f32) {
        self.w = w;
        self.layout_dirty = true;
    }
    pub fn set_h(&mut self, h: f32) {
        self.h = h;
        self.layout_dirty = true;
    }

    pub fn set_width(&mut self, w: Size) {
        self.width = w;
        self.layout_dirty = true;
    }
    pub fn set_height(&mut self, h: Size) {
        self.height = h;
        self.layout_dirty = true;
    }
    pub fn set_min_w(&mut self, w: Size) {
        self.min_w = w;
        self.layout_dirty = true;
    }
    pub fn set_max_w(&mut self, w: Size) {
        self.max_w = w;
        self.layout_dirty = true;
    }
    pub fn set_min_h(&mut self, h: Size) {
        self.min_h = h;
        self.layout_dirty = true;
    }
    pub fn set_max_h(&mut self, h: Size) {
        self.max_h = h;
        self.layout_dirty = true;
    }

    pub fn set_direction(&mut self, d: Direction) {
        self.direction = d;
        self.layout_dirty = true;
    }
    pub fn set_wrap(&mut self, w: Wrap) {
        self.wrap = w;
        self.layout_dirty = true;
    }
    pub fn set_distribute(&mut self, d: Distribute) {
        self.distribute = d;
        self.layout_dirty = true;
    }
    pub fn set_align(&mut self, a: Align) {
        self.align = a;
        self.layout_dirty = true;
    }
    pub fn set_gap(&mut self, g: [f32; 2]) {
        self.gap = g;
        self.layout_dirty = true;
    }

    pub fn set_self_align(&mut self, a: SelfAlign) {
        self.self_align = a;
        self.layout_dirty = true;
    }
    pub fn set_grow(&mut self, g: f32) {
        self.grow = g;
        self.layout_dirty = true;
    }
    pub fn set_shrink(&mut self, s: f32) {
        self.shrink = s;
        self.layout_dirty = true;
    }
    pub fn set_basis(&mut self, b: Size) {
        self.basis = b;
        self.layout_dirty = true;
    }

    pub fn set_padding(&mut self, p: [f32; 4]) {
        self.padding = p;
        self.layout_dirty = true;
    }
    pub fn set_margin(&mut self, m: [f32; 4]) {
        self.margin = m;
        self.layout_dirty = true;
    }

    pub fn set_position(&mut self, p: Position) {
        self.position = p;
        self.layout_dirty = true;
    }
    pub fn set_inset(&mut self, i: [Size; 4]) {
        self.inset = i;
        self.layout_dirty = true;
    }
    pub fn set_overflow(&mut self, o: Overflow) {
        self.overflow = o;
        self.layout_dirty = true;
    }
    pub fn set_aspect_ratio(&mut self, a: Option<f32>) {
        self.aspect_ratio = a;
        self.layout_dirty = true;
    }

    pub fn set_layer(&mut self, l: u32) {
        self.layer = l;
        self.render_dirty = true;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
        self.render_dirty = true;
    }
    pub fn set_displayed(&mut self, d: bool) {
        self.displayed = d;
    }

    pub fn set_focused(&mut self, f: bool) {
        self.focused = f;
    }

    pub fn set_layout_dirty(&mut self, d: bool) {
        self.layout_dirty = d;
    }
    pub fn set_render_dirty(&mut self, d: bool) {
        self.render_dirty = d;
    }

    pub fn set_content_w(&mut self, w: f32) {
        self.content_w = w;
    }
    pub fn set_content_h(&mut self, h: f32) {
        self.content_h = h;
    }
}
