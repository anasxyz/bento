mod widget;
mod handle;

pub use widget::{AsAny, Widget};
pub use handle::{WidgetHandle, WidgetId};

mod primitives;

pub use primitives::Rect;
pub use primitives::Text;
pub use primitives::Button;
pub use primitives::Container;
