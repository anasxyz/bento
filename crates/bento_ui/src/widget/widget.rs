use crate::Ui;
use crate::acc::Accumulated;
use crate::layout::Size;
use crate::widget::WidgetHandle;
use bento_wgpu::TextMeasurer;
use bento_wgpu::DrawList;
use std::any::Any;

pub trait Widget {
    /// The name of the widget. 
    /// Used mainly for debugging.
    fn name(&self) -> &str { "unnamed" }

    /// Called when the widget is added to the ui.
    /// Usually used to set up internal event listeners.
    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {}

    /// Updates widget state. Called on every frame.
    /// Also used to allow widgets to measure themselves.
    fn update(&mut self, measurer: &mut TextMeasurer) {}

    /// Returns the size of the widget.
    /// Fields `w` and `h` are the size of the widget.
    fn size(&self) -> (f32, f32) { (0.0, 0.0) }

    /// Sets the size of the widget.
    fn set_size(&mut self, w: f32, h: f32) {}

    /// Returns the width sizing of the widget.
    /// The width sizing of a widget determines how the widget wants be sized.
    /// If the widget is set to `Size::Auto` it will be sized to the size of its content.
    /// If the widget is set to `Size::Fill` it will be sized to the size of its parent.
    /// If the widget is set to `Size::Fixed(w)` it will be sized to `w`.
    fn width_sizing(&self) -> &Size { &Size::Auto }

    /// Returns the height sizing of the widget.
    /// The height sizing of a widget determines how the widget wants be sized.
    /// If the widget is set to `Size::Auto` it will be sized to the size of its content.
    /// If the widget is set to `Size::Fill` it will be sized to the size of its parent.
    /// If the widget is set to `Size::Fixed(h)` it will be sized to `h`.
    fn height_sizing(&self) -> &Size { &Size::Auto }

    /// Returns the position of the widget.
    /// Fields `x` and `y` are the position of the widget.
    fn position(&self) -> (f32, f32) { (0.0, 0.0) }

    /// Sets the position of the widget.
    fn set_position(&mut self, x: f32, y: f32) {}

    /// Returns the z-index of the widget.
    fn z(&self) -> i32 { 0 }

    /// Allows the widget to describe how it should be rendered.
    /// The canvas is used to issue draw commands.
    fn render(&mut self, canvas: &mut Canvas) {}
}

pub trait AnyWidget: Widget + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<W: Widget + Any> AnyWidget for W {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub struct Canvas<'a> {
    pub draw_list: &'a mut DrawList,
    pub x: f32,
    pub y: f32,
    pub z: i32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}
