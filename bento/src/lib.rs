#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod app;
mod color;
mod fonts;
mod layout;
mod runner;
mod settings;
mod ui;
mod widget;
mod widgets;
mod window;

pub use crate::widget::{AnyWidget, AsAny, Base, Handle, HasBase, LayoutExt, Widget};
pub use app::{App, WindowHandle};
pub use color::{Color, hex, hsl, hsla, hwb, hwba, rgb, rgba};
pub use fonts::{FontAttrs, Fonts};
pub use layout::{
    AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, Layout, Overflow, Position,
    Size,
};
pub use settings::WindowConfig;
pub use ui::Ui;
pub use widgets::Rect;
