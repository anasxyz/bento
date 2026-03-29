#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod app;
mod color;
mod dispatch;
mod fonts;
mod input;
mod layout;
mod runner;
mod settings;
mod ui;
mod widget;
mod widgets;
mod window;

pub use app::{App, WindowHandle};
pub use color::{Color, hex, hsl, hsla, hwb, hwba, rgb, rgba};
pub use fonts::{FontAttrs, Fonts};
pub use input::{Cursor, InputState, Key, KeyState, Modifiers, MouseButton, MouseState};
pub use layout::{
    AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, Layout, LayoutEngine, Overflow,
    Position, Size,
};
pub use settings::WindowConfig;
pub use ui::{
    Blur, Change, Click, DoubleClick, Event, Focus, Hover, HoverEnd, KeyPress, KeyRelease,
    MouseMove, Press, Release, RightClick, Scroll, Ui,
};
pub use widget::{AnyWidget, AsAny, Base, Handle, HasBase, LayoutExt, Widget};
pub use widgets::{Button, Label, Rect, ScrollContainer, TextInput};

pub use bento_derive::Widget;
