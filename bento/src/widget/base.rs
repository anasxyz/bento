use crate::{
    Cursor,
    layout::{
        AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, Layout, Overflow, Position,
        Size,
    },
};

pub struct Base {
    pub layout: Layout,
    pub focused: bool,
    pub cursor: Cursor,
    pub layer: u32,
    pub visible: bool,
    pub(crate) layout_dirty: bool,
    pub(crate) render_dirty: bool,
    pub(crate) content_height: f32,
    pub(crate) content_width: f32,
}

impl Base {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            focused: false,
            cursor: Cursor::Default,
            layer: 0,
            visible: true,
            layout_dirty: true,
            render_dirty: true,
            content_height: 0.0,
            content_width: 0.0,
        }
    }
}

impl Default for Base {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HasBase {
    fn base(&self) -> &Base;
    fn base_mut(&mut self) -> &mut Base;
}

pub trait LayoutExt: HasBase + Sized {
    fn set_width(&mut self, v: Size) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.width = v;
        self
    }
    fn set_height(&mut self, v: Size) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.height = v;
        self
    }
    fn set_min_w(&mut self, v: Size) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.min_w = v;
        self
    }
    fn set_max_w(&mut self, v: Size) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.max_w = v;
        self
    }
    fn set_min_h(&mut self, v: Size) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.min_h = v;
        self
    }
    fn set_max_h(&mut self, v: Size) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.max_h = v;
        self
    }
    fn set_padding(&mut self, v: [f32; 4]) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.padding = v;
        self
    }
    fn set_margin(&mut self, v: [f32; 4]) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.margin = v;
        self
    }
    fn set_row_gap(&mut self, v: f32) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.row_gap = v;
        self
    }
    fn set_col_gap(&mut self, v: f32) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.col_gap = v;
        self
    }
    fn set_flex_direction(&mut self, v: FlexDirection) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.flex_direction = v;
        self
    }
    fn set_flex_wrap(&mut self, v: FlexWrap) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.flex_wrap = v;
        self
    }
    fn set_flex_grow(&mut self, v: f32) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.flex_grow = v;
        self
    }
    fn set_flex_shrink(&mut self, v: f32) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.flex_shrink = v;
        self
    }
    fn set_flex_basis(&mut self, v: Size) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.flex_basis = v;
        self
    }
    fn set_align_items(&mut self, v: AlignItems) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.align_items = v;
        self
    }
    fn set_align_self(&mut self, v: AlignSelf) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.align_self = v;
        self
    }
    fn set_justify_content(&mut self, v: JustifyContent) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.justify_content = v;
        self
    }
    fn set_position(&mut self, v: Position) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.position = v;
        self
    }
    fn set_inset(&mut self, v: [Size; 4]) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.inset = v;
        self
    }
    fn set_overflow(&mut self, v: Overflow) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.overflow = v;
        self
    }
    fn set_layer(&mut self, v: u32) -> &mut Self {
        self.base_mut().layer = v;
        self.base_mut().render_dirty = true;
        self
    }
    fn set_display(&mut self, v: bool) -> &mut Self {
        self.base_mut().layout.displayed = v;
        if v {
            self.base_mut().visible = true;
        }
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self
    }
    fn set_visibility(&mut self, v: bool) -> &mut Self {
        self.base_mut().visible = v;
        self.base_mut().render_dirty = true;
        self
    }
    fn is_displayed(&self) -> bool {
        self.base().layout.displayed
    }
    fn is_visible(&self) -> bool {
        self.base().visible
    }
    fn set_aspect_ratio(&mut self, v: Option<f32>) -> &mut Self {
        self.base_mut().layout_dirty = true;
        self.base_mut().render_dirty = true;
        self.base_mut().layout.aspect_ratio = v;
        self
    }
}

impl<T: HasBase> LayoutExt for T {}
