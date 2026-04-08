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
