#![allow(dead_code)]
#![allow(unused)]

mod ui;
mod widget;
mod input;

use widget::{WidgetHandle, WidgetId, AsAny};
use input::{Input};

pub use ui::{Ui};
pub use widget::{Widget, Rect, Text, Button};
pub use input::{MouseButton, Key};
pub use ui::{KeyPress, KeyRelease, Click, MouseMove, MouseDown, MouseUp, MouseScroll, MouseEnter, MouseLeave, FocusGained, FocusLost, HoverEnter, HoverLeave, WindowResized};
